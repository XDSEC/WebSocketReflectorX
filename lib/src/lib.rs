use axum::extract::ws::{Message as AxumMessage, WebSocket};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{tungstenite::Message as TungsteniteMessage, WebSocketStream};

pub async fn proxy_axum_ws(mut ws: WebSocket, mut tcp: TcpStream) {
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
                    return;
                }
            },
            res  = tcp.read(&mut buf) => {
                match res {
                    // TODO: add pcap tcp stream dump features here
                    Ok(n) => {
                        if 0 != n {
                            let _ = ws.send(AxumMessage::Binary(buf[..n].to_vec())).await;
                        } else {
                            return ;
                        }
                    },
                    Err(_err) => {
                        return;
                    }
                }
            },
        }
    }
}

pub async fn proxy_ws(mut ws: WebSocketStream<MaybeTlsStream<TcpStream>>, mut tcp: TcpStream) {
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
                    return;
                }
            },
            res  = tcp.read(&mut buf) => {
                // TODO: add pcap tcp stream dump features here
                match res {
                    Ok(n) => {
                        if 0 != n {
                            let _ = ws.send(TungsteniteMessage::Binary(buf[..n].to_vec())).await;
                        } else {
                            return ;
                        }
                    },
                    Err(_err) => {
                        return;
                    }
                }
            },
        }
    }
}
