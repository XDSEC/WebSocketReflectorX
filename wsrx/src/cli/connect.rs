use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn, debug};
use url::Url;
use wsrx::proxy;

use crate::cli::logger::init_logger;


pub async fn launch_client(address: String, host: Option<String>, port: Option<u16>) {
  let port = port.unwrap_or(0);
  let host = host.unwrap_or(String::from("127.0.0.1"));
  init_logger();
  let listener = TcpListener::bind(format!("{host}:{port}"))
      .await
      .expect("failed to bind port");
  let Ok(url) = Url::parse(&address) else {
      error!("Invalid url, please check your input.");
      return;
  };
  if url.scheme() != "ws" && url.scheme() != "wss" {
      error!("Invalid url scheme, only `ws` and `wss` are supported.");
      return;
  }
  let url = url.as_ref().to_string();
  info!(
      "Hi, I am not RX, RX is here -> {}",
      listener.local_addr().unwrap()
  );
  warn!("wsrx will not report non-critical errors by default, you can set `RUST_LOG=wsrx=debug` to see more details.");
  loop {
      let Ok((tcp, _)) = listener.accept().await else {
          error!("Failed to accept tcp connection, exiting.");
          return;
      };
      let url = url.clone();
      let peer_addr = tcp.peer_addr().unwrap();
      info!("CREATE remote <-wsrx-> {}", peer_addr);
      tokio::spawn(async move {
          match proxy_ws_addr(url, tcp)
              .await {
                  Ok(_) => {}
                  Err(e) => {
                      info!("REMOVE remote <-wsrx-> {} with error", peer_addr);
                      debug!("TCP connection closed: {}", e);
                  }
              }
      });
  }
}

async fn proxy_ws_addr(addr: impl AsRef<str>, tcp: TcpStream) -> Result<(), wsrx::Error> {
  let peer_addr = tcp.peer_addr().unwrap();
  let (ws, _) = tokio_tungstenite::connect_async(addr.as_ref()).await?;
  proxy(ws.into(), tcp).await?;
  info!("REMOVE remote <-wsrx-> {}", peer_addr);
  Ok(())
}
