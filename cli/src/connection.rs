use tokio::net::TcpListener;
use tokio::net::TcpStream;
use url::Url;

pub async fn connect(url: impl AsRef<str>, port: Option<u16>) -> anyhow::Result<()> {
    let port = port.unwrap_or(0);
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    let port = listener.local_addr()?.port();
    let url = Url::parse(url.as_ref())?;
    if url.scheme() != "ws" && url.scheme() != "wss" {
        return Err(anyhow::anyhow!("invalid scheme: {}", url.scheme()));
    }
    let url = url.as_ref().to_string();
    log::info!("Hi, I am not RX, RX is here -> 127.0.0.1:{port}");
    loop {
        let (tcp, _) = listener.accept().await.expect("Failed to accept tcp connection");
        let url = url.clone();
        tokio::spawn(async move {
            proxy_ws_addr(url, tcp).await.expect("Failed to proxy tcp connection, exiting.");
        });
    }
}

async fn proxy_ws_addr(addr: impl AsRef<str>, tcp: TcpStream) -> anyhow::Result<()> {
    let (ws, _) = tokio_tungstenite::connect_async(addr.as_ref()).await?;
    wsrx::proxy_ws(ws, tcp).await?;
    Ok(())
}
