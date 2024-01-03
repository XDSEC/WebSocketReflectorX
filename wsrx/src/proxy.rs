use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{sink::Sink, stream::Stream, StreamExt};
use thiserror::Error;
use tokio_util::{
    bytes::{BufMut, Bytes, BytesMut},
    codec::{Decoder, Encoder},
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[cfg(feature = "client")]
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[cfg(feature = "server")]
    #[error("Axum error: {0}")]
    Axum(#[from] axum::Error),
}

pub enum Message {
    Binary(Vec<u8>),
    Others,
}

#[cfg(feature = "client")]
impl From<tokio_tungstenite::tungstenite::Message> for Message {
    fn from(msg: tokio_tungstenite::tungstenite::Message) -> Self {
        match msg {
            tokio_tungstenite::tungstenite::Message::Binary(data) => Message::Binary(data),
            tokio_tungstenite::tungstenite::Message::Text(data) => {
                Message::Binary(data.into_bytes())
            }
            _ => Message::Others,
        }
    }
}

#[cfg(feature = "server")]
impl From<axum::extract::ws::Message> for Message {
    fn from(msg: axum::extract::ws::Message) -> Self {
        match msg {
            axum::extract::ws::Message::Binary(data) => Message::Binary(data),
            axum::extract::ws::Message::Text(data) => Message::Binary(data.into_bytes()),
            _ => Message::Others,
        }
    }
}

pub enum WsStream {
    #[cfg(feature = "client")]
    Tungstenite(
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    ),
    #[cfg(feature = "server")]
    AxumWebsocket(axum::extract::ws::WebSocket),
}

pub struct WrappedWsStream {
    stream: WsStream,
}

#[cfg(feature = "client")]
impl From<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>> for WrappedWsStream {
    fn from(stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>) -> Self {
        WrappedWsStream {
            stream: WsStream::Tungstenite(stream),
        }
    }
}

#[cfg(feature = "server")]
impl From<axum::extract::ws::WebSocket> for WrappedWsStream {
    fn from(stream: axum::extract::ws::WebSocket) -> Self {
        WrappedWsStream {
            stream: WsStream::AxumWebsocket(stream),
        }
    }
}

impl Stream for WrappedWsStream {
    type Item = Result<Message, Error>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut self.stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => {
                match futures_util::ready!(Pin::new(stream).poll_next(_cx)) {
                    Some(Ok(msg)) => return Poll::Ready(Some(Ok(msg.into()))),
                    Some(Err(e)) => return Poll::Ready(Some(Err(e.into()))),
                    None => return Poll::Ready(None),
                }
            }
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => {
                match futures_util::ready!(Pin::new(stream).poll_next(_cx)) {
                    Some(Ok(msg)) => return Poll::Ready(Some(Ok(msg.into()))),
                    Some(Err(e)) => return Poll::Ready(Some(Err(e.into()))),
                    None => return Poll::Ready(None),
                }
            }
            #[allow(unreachable_patterns)]
            _ => return Poll::Ready(None),
        }
    }
}

impl Sink<Message> for WrappedWsStream {
    type Error = Error;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match &mut self.get_mut().stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => std::pin::Pin::new(stream)
                .poll_ready(_cx)
                .map_err(|e| e.into()),
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => std::pin::Pin::new(stream)
                .poll_ready(_cx)
                .map_err(|e| e.into()),
            #[allow(unreachable_patterns)]
            _ => Poll::Ready(Ok(())),
        }
    }

    fn start_send(self: std::pin::Pin<&mut Self>, _item: Message) -> Result<(), Self::Error> {
        match &mut self.get_mut().stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => match _item {
                Message::Binary(data) => std::pin::Pin::new(stream)
                    .start_send(tokio_tungstenite::tungstenite::Message::Binary(data))
                    .map_err(|e| e.into()),
                Message::Others => Ok(()),
            },
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => match _item {
                Message::Binary(data) => std::pin::Pin::new(stream)
                    .start_send(axum::extract::ws::Message::Binary(data))
                    .map_err(|e| e.into()),
                Message::Others => Ok(()),
            },
            #[allow(unreachable_patterns)]
            _ => Ok(()),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match &mut self.get_mut().stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => std::pin::Pin::new(stream)
                .poll_flush(_cx)
                .map_err(|e| e.into()),
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => std::pin::Pin::new(stream)
                .poll_flush(_cx)
                .map_err(|e| e.into()),
            #[allow(unreachable_patterns)]
            _ => Poll::Ready(Ok(())),
        }
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        match &mut self.get_mut().stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => std::pin::Pin::new(stream)
                .poll_close(_cx)
                .map_err(|e| e.into()),
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => std::pin::Pin::new(stream)
                .poll_close(_cx)
                .map_err(|e| e.into()),
            #[allow(unreachable_patterns)]
            _ => Poll::Ready(Ok(())),
        }
    }
}

pub async fn proxy_stream<S, T>(s1: S, s2: T) -> Result<(), Error>
where
    S: Sink<Message, Error = Error> + Stream<Item = Result<Message, Error>> + Unpin,
    T: Sink<Message, Error = Error> + Stream<Item = Result<Message, Error>> + Unpin,
{
    let (s1sink, s1stream) = s1.split();
    let (s2sink, s2stream) = s2.split();
    let f1 = s1stream.forward(s2sink);
    let f2 = s2stream.forward(s1sink);
    tokio::try_join!(f1, f2)?;
    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct MessageCodec(());

impl MessageCodec {
    /// Creates a new `MessageCodec` for shipping around raw bytes.
    pub fn new() -> MessageCodec {
        MessageCodec(())
    }
}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Message>, Error> {
        if !buf.is_empty() {
            let len = buf.len();
            Ok(Some(Message::Binary(buf.split_to(len).to_vec())))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<Message> for MessageCodec {
    type Error = Error;

    fn encode(&mut self, data: Message, buf: &mut BytesMut) -> Result<(), Error> {
        match data {
            Message::Binary(data) => {
                buf.reserve(data.len());
                buf.put(Bytes::from(data));
                Ok(())
            }
            Message::Others => Ok(()),
        }
    }
}

pub async fn proxy(ws: WrappedWsStream, tcp: tokio::net::TcpStream) -> Result<(), Error> {
    let framed_tcp_stream = tokio_util::codec::Framed::new(tcp, MessageCodec::new());
    proxy_stream(ws, framed_tcp_stream).await?;
    Ok(())
}
