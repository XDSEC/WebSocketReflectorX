use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicBool, Ordering},
};

use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use super::proxy;

/// Configuration for a tunnel, contains the local and remote addresses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    #[serde(alias = "from")]
    pub local: String,
    #[serde(alias = "to")]
    pub remote: String,
}

/// Options controlling how a [`Tunnel`] dials its remote WebSocket.
#[derive(Debug, Clone, Default)]
pub struct TunnelOptions {
    /// Shared flag: while set, TLS certificates of `wss://` remotes are not
    /// verified, allowing connections to servers with expired, self-signed
    /// or otherwise invalid certificates.
    ///
    /// The flag is shared on purpose: flipping it at runtime is observed by
    /// every WebSocket connection established afterwards, including the
    /// connections of already running tunnels.
    pub accept_invalid_certs: Option<Arc<AtomicBool>>,
}

impl TunnelOptions {
    /// Creates options owning a private flag initialized to `enabled`.
    pub fn new(accept_invalid_certs: bool) -> Self {
        Self {
            accept_invalid_certs: Some(Arc::new(AtomicBool::new(accept_invalid_certs))),
        }
    }

    /// Creates options backed by a caller-owned flag, so flips of that flag
    /// are observed live by the tunnel.
    pub fn shared(accept_invalid_certs: Arc<AtomicBool>) -> Self {
        Self {
            accept_invalid_certs: Some(accept_invalid_certs),
        }
    }

    fn accepts_invalid_certs(&self) -> bool {
        self.accept_invalid_certs
            .as_ref()
            .is_some_and(|flag| flag.load(Ordering::Relaxed))
    }
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
    /// Creates a new `Tunnel` instance with default options.
    pub fn new(remote: impl AsRef<str>, listener: TcpListener) -> Self {
        Self::with_options(remote, listener, TunnelOptions::default())
    }

    /// Creates a new `Tunnel` instance with the given connection options.
    pub fn with_options(
        remote: impl AsRef<str>, listener: TcpListener, options: TunnelOptions,
    ) -> Self {
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
                let proxy_options = options.clone();

                tokio::spawn(async move {
                    let ws = match connect_remote(
                        proxy_config.remote.as_str(),
                        proxy_options.accepts_invalid_certs(),
                    )
                    .await
                    {
                        Ok(ws) => ws,
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

/// Establishes the WebSocket connection to `remote`. When
/// `accept_invalid_certs` is set, TLS certificate verification is skipped so
/// servers with expired or invalid certificates can be reached.
async fn connect_remote(
    remote: &str, accept_invalid_certs: bool,
) -> Result<
    WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    tokio_tungstenite::tungstenite::Error,
> {
    if !accept_invalid_certs {
        return connect_async(remote).await.map(|(ws, _)| ws);
    }

    warn!(
        "TLS certificate verification is disabled for {}, accepting expired or invalid certificates",
        remote
    );
    tokio_tungstenite::connect_async_tls_with_config(
        remote,
        None,
        false,
        Some(tokio_tungstenite::Connector::Rustls(
            insecure_client_config(),
        )),
    )
    .await
    .map(|(ws, _)| ws)
}

/// Cached accept-all TLS client config, built on first use.
static INSECURE_CLIENT_CONFIG: OnceLock<Arc<rustls::ClientConfig>> = OnceLock::new();

/// Builds a rustls client config that accepts any server certificate.
///
/// Uses the process-wide crypto provider when one is installed (the desktop
/// app installs one at startup) and falls back to `ring` otherwise, so the
/// config can also be built in library contexts without a default provider.
fn insecure_client_config() -> Arc<rustls::ClientConfig> {
    INSECURE_CLIENT_CONFIG
        .get_or_init(build_insecure_client_config)
        .clone()
}

fn build_insecure_client_config() -> Arc<rustls::ClientConfig> {
    use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};

    /// A certificate verifier that unconditionally accepts the server
    /// certificate chain, i.e. it disables TLS verification entirely.
    #[derive(Debug)]
    struct AcceptAnyServerCert(rustls::crypto::WebPkiSupportedAlgorithms);

    impl ServerCertVerifier for AcceptAnyServerCert {
        fn verify_server_cert(
            &self, _end_entity: &rustls::pki_types::CertificateDer<'_>,
            _intermediates: &[rustls::pki_types::CertificateDer<'_>],
            _server_name: &rustls::pki_types::ServerName<'_>, _ocsp_response: &[u8],
            _now: rustls::pki_types::UnixTime,
        ) -> Result<ServerCertVerified, rustls::Error> {
            Ok(ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self, _message: &[u8], _cert: &rustls::pki_types::CertificateDer<'_>,
            _dss: &rustls::DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn verify_tls13_signature(
            &self, _message: &[u8], _cert: &rustls::pki_types::CertificateDer<'_>,
            _dss: &rustls::DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            Ok(HandshakeSignatureValid::assertion())
        }

        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            self.0.supported_schemes()
        }
    }

    let provider = rustls::crypto::CryptoProvider::get_default()
        .cloned()
        .unwrap_or_else(|| Arc::new(rustls::crypto::ring::default_provider()));
    let verify_algorithms = provider.signature_verification_algorithms;

    Arc::new(
        rustls::ClientConfig::builder_with_provider(provider)
            .with_safe_default_protocol_versions()
            .expect("default protocol versions are supported by the provider")
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(AcceptAnyServerCert(verify_algorithms)))
            .with_no_client_auth(),
    )
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
