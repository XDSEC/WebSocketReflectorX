//! WebSocket Reflector X
//!
//! A simple crate that proxies pure TCP connections to WebSocket connections
//! and vice versa.

pub mod proxy;

#[cfg(feature = "client")]
pub mod utils;

#[cfg(feature = "client")]
pub mod tunnel;

pub use proxy::{proxy, Error, Message, WrappedWsStream};
