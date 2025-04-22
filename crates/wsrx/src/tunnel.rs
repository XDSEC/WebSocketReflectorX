use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use super::proxy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    #[serde(alias = "from")]
    pub local: Arc<String>,
    #[serde(alias = "to")]
    pub remote: Arc<String>,
}

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
    pub fn new(remote: Arc<String>, listener: TcpListener) -> Self {
        let local = Arc::new(
            listener
                .local_addr()
                .expect("failed to bind port")
                .to_string(),
        );

        info!("CREATE tcp server: {} <-wsrx-> {}", local, remote);

        let token = CancellationToken::new();

        let config = TunnelConfig { local, remote };

        let loop_config = config.clone();
        let loop_token = token.clone();
        let handle = tokio::task::spawn(async move {
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
                    let ws = match tokio_tungstenite::connect_async(proxy_config.remote.as_str())
                        .await
                    {
                        Ok((ws, _)) => ws,
                        Err(e) => {
                            error!("Failed to connect to {}: {}", proxy_config.remote, e);
                            proxy_token.cancel();
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
