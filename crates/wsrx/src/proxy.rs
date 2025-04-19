//! The main proxy module for WebSocket Reflector X.

use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(feature = "server")]
use axum::extract::ws::{Message as AxMessage, WebSocket};
use futures_util::{StreamExt, sink::Sink, stream::Stream};
use thiserror::Error;
use tokio::net::TcpStream;
#[cfg(feature = "client")]
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{Error as TgError, Message as TgMessage},
};
use tokio_util::{
    bytes::{BufMut, Bytes, BytesMut},
    codec::{Decoder, Encoder, Framed},
};

/// An error type for WebSocket Reflector X.
#[derive(Error, Debug)]
pub enum Error {
    /// An IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// A WebSocket error from tungstenite.
    #[cfg(feature = "client")]
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] TgError),
    /// A WebSocket error from axum.
    #[cfg(feature = "server")]
    #[error("Axum error: {0}")]
    Axum(#[from] axum::Error),
}

/// A enum for different type of WebSocket message.
///
/// Just Binary message will be tunneled, other type of websocket message will
/// just be discarded.
pub enum Message {
    Binary(Vec<u8>),
    Others,
}

/// A enum for different type of WebSocket message.
#[cfg(feature = "client")]
impl From<TgMessage> for Message {
    /// Converts a `TgMessage` to a `Message`.
    fn from(msg: TgMessage) -> Self {
        match msg {
            TgMessage::Binary(data) => Message::Binary(data.into()),
            TgMessage::Text(data) => Message::Binary(data.as_bytes().to_vec()),
            _ => Message::Others,
        }
    }
}

/// Converts a `Message` to a `TgMessage`.
#[cfg(feature = "server")]
impl From<AxMessage> for Message {
    /// Converts a `AxMessage` to a `Message`.
    fn from(msg: AxMessage) -> Self {
        match msg {
            AxMessage::Binary(data) => Message::Binary(data.into()),
            AxMessage::Text(data) => Message::Binary(data.as_bytes().to_vec()),
            _ => Message::Others,
        }
    }
}

/// A enum for different type of WebSocket stream.
///
/// honestly, this is a bit of a hack, but it works.
/// The WebSocketStream in axum is derived from tungstenite, but axum does not
/// expose the tungstenite stream.
pub enum WsStream {
    /// Tungstenite WebSocket stream.
    #[cfg(feature = "client")]
    Tungstenite(WebSocketStream<MaybeTlsStream<TcpStream>>),
    /// Axum WebSocket stream.
    #[cfg(feature = "server")]
    AxumWebsocket(WebSocket),
}

/// A wrapper around WebSocket stream.
pub struct WrappedWsStream {
    /// The WebSocket stream.
    stream: WsStream,
}

#[cfg(feature = "client")]
impl From<WebSocketStream<MaybeTlsStream<TcpStream>>> for WrappedWsStream {
    /// Creates a new `WrappedWsStream` from tungstenite's WebSocket stream.
    fn from(stream: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Self {
        WrappedWsStream {
            stream: WsStream::Tungstenite(stream),
        }
    }
}

#[cfg(feature = "server")]
impl From<WebSocket> for WrappedWsStream {
    /// Creates a new `WrappedWsStream` from axum's WebSocket stream.
    fn from(stream: WebSocket) -> Self {
        WrappedWsStream {
            stream: WsStream::AxumWebsocket(stream),
        }
    }
}

/// A wrapper around WebSocket stream that implements `Stream` trait.
impl Stream for WrappedWsStream {
    type Item = Result<Message, Error>;

    /// Polls the next message from the WebSocket stream.
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut self.stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => {
                match futures_util::ready!(Pin::new(stream).poll_next(_cx)) {
                    Some(Ok(msg)) => Poll::Ready(Some(Ok(msg.into()))),
                    Some(Err(e)) => Poll::Ready(Some(Err(e.into()))),
                    None => Poll::Ready(None),
                }
            }
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => {
                match futures_util::ready!(Pin::new(stream).poll_next(_cx)) {
                    Some(Ok(msg)) => Poll::Ready(Some(Ok(msg.into()))),
                    Some(Err(e)) => Poll::Ready(Some(Err(e.into()))),
                    None => Poll::Ready(None),
                }
            }
            #[allow(unreachable_patterns)]
            _ => Poll::Ready(None),
        }
    }
}

/// A wrapper around WebSocket stream that implements `Sink` trait.
impl Sink<Message> for WrappedWsStream {
    type Error = Error;

    /// Polls the WebSocket stream if it is ready to send a message.
    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut self.get_mut().stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => Pin::new(stream).poll_ready(_cx).map_err(|e| e.into()),
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => {
                Pin::new(stream).poll_ready(_cx).map_err(|e| e.into())
            }
            #[allow(unreachable_patterns)]
            _ => Poll::Ready(Ok(())),
        }
    }

    /// Sends a message to the WebSocket stream.
    fn start_send(self: Pin<&mut Self>, _item: Message) -> Result<(), Self::Error> {
        match &mut self.get_mut().stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => match _item {
                Message::Binary(data) => Pin::new(stream)
                    .start_send(TgMessage::Binary(data.into()))
                    .map_err(|e| e.into()),
                Message::Others => Ok(()),
            },
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => match _item {
                Message::Binary(data) => Pin::new(stream)
                    .start_send(AxMessage::Binary(data.into()))
                    .map_err(|e| e.into()),
                Message::Others => Ok(()),
            },
            #[allow(unreachable_patterns)]
            _ => Ok(()),
        }
    }

    /// Polls the WebSocket stream to flush the message.
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut self.get_mut().stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => Pin::new(stream).poll_flush(_cx).map_err(|e| e.into()),
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => {
                Pin::new(stream).poll_flush(_cx).map_err(|e| e.into())
            }
            #[allow(unreachable_patterns)]
            _ => Poll::Ready(Ok(())),
        }
    }

    /// Polls the WebSocket stream to close the connection.
    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut self.get_mut().stream {
            #[cfg(feature = "client")]
            WsStream::Tungstenite(stream) => Pin::new(stream).poll_close(_cx).map_err(|e| e.into()),
            #[cfg(feature = "server")]
            WsStream::AxumWebsocket(stream) => {
                Pin::new(stream).poll_close(_cx).map_err(|e| e.into())
            }
            #[allow(unreachable_patterns)]
            _ => Poll::Ready(Ok(())),
        }
    }
}

/// Proxies two streams.
///
/// * `s1` - The first stream.
/// * `s2` - The second stream.
pub async fn proxy_stream<S, T>(s1: S, s2: T) -> Result<(), Error>
where
    S: Sink<Message, Error = Error> + Stream<Item = Result<Message, Error>> + Unpin,
    T: Sink<Message, Error = Error> + Stream<Item = Result<Message, Error>> + Unpin, {
    let (s1sink, s1stream) = s1.split();
    let (s2sink, s2stream) = s2.split();
    let f1 = s1stream.forward(s2sink);
    let f2 = s2stream.forward(s1sink);
    tokio::try_join!(f1, f2)?;
    Ok(())
}

/// A codec for WebSocket messages.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct MessageCodec;

impl MessageCodec {
    /// Creates a new `MessageCodec` for shipping around raw bytes.
    pub fn new() -> MessageCodec {
        Self {}
    }
}

/// A codec for WebSocket messages.
impl Decoder for MessageCodec {
    type Item = Message;
    type Error = Error;

    /// Decodes a WebSocket message from the buffer.
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Message>, Error> {
        if !buf.is_empty() {
            let len = buf.len();
            Ok(Some(Message::Binary(buf.split_to(len).to_vec())))
        } else {
            Ok(None)
        }
    }
}

/// A codec for WebSocket messages.
impl Encoder<Message> for MessageCodec {
    type Error = Error;

    /// Encodes a WebSocket message to the buffer.
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

/// Proxies a WebSocket stream with a TCP stream.
///
/// * `ws` - The WebSocket stream, either axum's stream or tungstenite stream
///   are supported.
/// * `tcp` - The TCP stream.
pub async fn proxy(ws: WrappedWsStream, tcp: TcpStream) -> Result<(), Error> {
    let framed_tcp_stream = Framed::new(tcp, MessageCodec::new());
    proxy_stream(ws, framed_tcp_stream).await?;
    Ok(())
}
