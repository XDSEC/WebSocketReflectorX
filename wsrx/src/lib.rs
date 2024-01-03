//! WebSocket Reflector X
//!
//! A simple crate that proxies pure TCP connections to WebSocket connections and vice versa.

pub mod proxy;

pub use proxy::{proxy, Error, Message, WrappedWsStream};
