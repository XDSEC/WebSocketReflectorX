
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[cfg(feature="cli")]
use tokio_tungstenite::MaybeTlsStream;
#[cfg(feature="cli")]
use tokio_tungstenite::{tungstenite::Message as TungsteniteMessage, WebSocketStream};
#[cfg(feature="cli")]
use futures_util::{SinkExt, StreamExt};

#[cfg(feature="web")]
use axum::extract::ws::{Message as AxumMessage, WebSocket};

#[cfg(feature="web")]
/// Proxy a axum websocket connection to a tcp connection
///
/// # Arguments
/// - `ws` - The websocket connection
/// - `tcp` - The tcp connection
///
/// # Returns
/// 
/// A `Result` with the error if any
pub async fn proxy_axum_ws(mut ws: WebSocket, mut tcp: TcpStream) -> anyhow::Result<()> {
    let mut buf = [0u8; 16384]; // the max ssl record should be 16384 by default

    loop {
        tokio::select! {
            res  = ws.recv() => {
                if let Some(msg) = res {
                    // TODO: add pcap tcp stream dump features here
                    if let Ok(AxumMessage::Binary(msg)) = msg {
                        let _ = tcp.write_all(&msg).await;
                    }
                } else {
                    return Ok(());
                }
            },
            res  = tcp.read(&mut buf) => {
                // TODO: add pcap tcp stream dump features here
                let res = res?;
                if 0 != res {
                    let _ = ws.send(AxumMessage::Binary(buf[..res].to_vec())).await;
                } else {
                    return Ok(());
                }
            },
        }
    }
}

#[cfg(feature="cli")]
/// Proxy a tungstenite websocket connection to a tcp connection
///
/// # Arguments
/// - `ws` - The websocket connection
/// - `tcp` - The tcp connection
///
/// # Returns
/// 
/// A `Result` with the error if any
pub async fn proxy_ws(
    mut ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    mut tcp: TcpStream,
) -> anyhow::Result<()> {
    let mut buf = [0u8; 16384]; // the max ssl record should be 16384 by default

    loop {
        tokio::select! {
            res  = ws.next() => {
                if let Some(msg) = res {
                    // TODO: add pcap tcp stream dump features here
                    if let Ok(TungsteniteMessage::Binary(msg)) = msg {
                        let _ = tcp.write_all(&msg).await;
                    }
                } else {
                    return Ok(());
                }
            },
            res  = tcp.read(&mut buf) => {
                // TODO: add pcap tcp stream dump features here
                let res = res?;
                if 0 != res {
                    let _ = ws.send(TungsteniteMessage::Binary(buf[..res].to_vec())).await;
                } else {
                    return Ok(());
                }
            },
        }
    }
}
