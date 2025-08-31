use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, task::JoinHandle};
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

                tokio::spawn(async move {
                    use tokio_tungstenite::connect_async;

                    let ws = match connect_async(proxy_config.remote.as_str()).await {
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
