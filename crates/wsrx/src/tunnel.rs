use std::sync::Arc;

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::CryptoProvider;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, Error, SignatureScheme};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_tungstenite::Connector;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use super::proxy;

/// Configuration for a tunnel, contains the local and remote addresses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    #[serde(alias = "from")]
    pub local: String,
    #[serde(alias = "to")]
    pub remote: String,
}

/// A tunnel that proxies TCP connections to a remote WebSocket server.
///
/// This struct is responsible for creating a TCP listener and accepting
/// incoming connections. It will then establish a WebSocket connection to the
/// remote server and proxy the data between the TCP connection and the
/// WebSocket connection.
#[derive(Debug)]
pub struct Tunnel {
    config: TunnelConfig,
    token: CancellationToken,
    handle: JoinHandle<()>,
}

impl Serialize for Tunnel {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.config.serialize(serializer)
    }
}

impl Tunnel {
    /// Creates a new `Tunnel` instance.
    pub fn new(remote: impl AsRef<str>, listener: TcpListener) -> Self {
        Self::build(remote, listener, false)
    }

    /// Creates a new `Tunnel` instance that connects to `wss://` remotes
    /// without verifying the server certificate at all. Every certificate,
    /// including self-signed, expired or mismatched ones, is trusted
    /// unconditionally.
    ///
    /// This is insecure and must only be used when the user explicitly opts
    /// in from the application settings.
    pub fn with_insecure_tls(remote: impl AsRef<str>, listener: TcpListener) -> Self {
        Self::build(remote, listener, true)
    }

    fn build(remote: impl AsRef<str>, listener: TcpListener, accept_invalid_certs: bool) -> Self {
        let local = listener
            .local_addr()
            .expect("failed to bind port")
            .to_string();

        info!("CREATE tcp server: {} <-wsrx-> {}", local, remote.as_ref());

        let token = CancellationToken::new();

        let config = TunnelConfig {
            local,
            remote: remote.as_ref().to_string(),
        };

        let tls_connector = if accept_invalid_certs {
            Some(Connector::Rustls(Arc::new(build_insecure_client_config())))
        } else {
            None
        };

        let loop_config = Arc::new(config.clone());
        let loop_token = token.clone();
        let handle = tokio::spawn(async move {
            loop {
                let Ok((tcp, _)) = listener.accept().await else {
                    error!("Failed to accept tcp connection, exiting.");
                    loop_token.cancel();
                    return;
                };

                let peer_addr = tcp.peer_addr().unwrap();

                if loop_token.is_cancelled() {
                    info!(
                        "STOP tcp server: {} <-wsrx-> {}: Task cancelled",
                        loop_config.local, loop_config.remote
                    );
                    return;
                }

                info!("LINK {} <-wsrx-> {}", loop_config.remote, peer_addr);

                let proxy_config = loop_config.clone();
                let proxy_token = loop_token.clone();
                let connector = tls_connector.clone();

                tokio::spawn(async move {
                    use tokio_tungstenite::connect_async_tls_with_config;

                    let ws = match connect_async_tls_with_config(
                        proxy_config.remote.as_str(),
                        None,
                        false,
                        connector,
                    )
                    .await
                    {
                        Ok((ws, _)) => ws,
                        Err(e) => {
                            error!("Failed to connect to {}: {}", proxy_config.remote, e);
                            return;
                        }
                    };

                    match proxy(ws.into(), tcp, proxy_token).await {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Failed to proxy: {e}");
                        }
                    }
                });
            }
        });

        Self {
            config,
            token,
            handle,
        }
    }
}

/// Implements the `Drop` trait for the `Tunnel` struct.
///
/// This will cancel the cancellation token and abort the task when the
/// `Tunnel` instance is dropped.
impl Drop for Tunnel {
    fn drop(&mut self) {
        info!(
            "REMOVE tcp server: {} <-wsrx-> {}",
            self.config.local, self.config.remote
        );
        self.token.cancel();
        self.handle.abort();
    }
}

impl std::ops::Deref for Tunnel {
    type Target = TunnelConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl std::ops::DerefMut for Tunnel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.config
    }
}

/// A `ServerCertVerifier` that accepts every server certificate without any
/// verification. Used only when the user explicitly opts in to insecure TLS
/// connections.
#[derive(Debug)]
struct AcceptAnyServerCert {
    schemes: Vec<SignatureScheme>,
}

impl ServerCertVerifier for AcceptAnyServerCert {
    fn verify_server_cert(
        &self, _end_entity: &CertificateDer<'_>, _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>, _ocsp_response: &[u8], _now: UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self, _message: &[u8], _cert: &CertificateDer<'_>, _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self, _message: &[u8], _cert: &CertificateDer<'_>, _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.schemes.clone()
    }
}

/// Builds a rustls client config that unconditionally trusts any server
/// certificate. Prefers the process-wide crypto provider when one is
/// installed, and falls back to the ring provider otherwise.
fn build_insecure_client_config() -> rustls::ClientConfig {
    let provider = CryptoProvider::get_default()
        .cloned()
        .unwrap_or_else(|| Arc::new(rustls::crypto::ring::default_provider()));
    let schemes = provider
        .signature_verification_algorithms
        .supported_schemes();

    rustls::ClientConfig::builder_with_provider(provider)
        .with_protocol_versions(rustls::ALL_VERSIONS)
        .expect("supported protocol versions")
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(AcceptAnyServerCert { schemes }))
        .with_no_client_auth()
}
