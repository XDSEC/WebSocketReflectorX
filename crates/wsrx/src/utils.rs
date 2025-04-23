use std::net::ToSocketAddrs;

use axum::http::StatusCode;
use tokio::net::TcpListener;
use tracing::error;


/// Creates a TCP listener on the specified local address.
///
/// @param local The local address to bind the TCP listener to.
///
/// @returns A `Result` containing the `TcpListener` if successful, or an error tuple
pub async fn create_tcp_listener(local: &str) -> Result<TcpListener, (StatusCode, String)> {
    let mut tcp_addr_obj = local.to_socket_addrs().map_err(|err| {
        error!("Failed to parse from address: {err}");
        (
            StatusCode::BAD_REQUEST,
            "failed to parse from address".to_owned(),
        )
    })?;

    let tcp_addr_obj = tcp_addr_obj.next().ok_or_else(|| {
        error!("Failed to get socket addr");
        (
            StatusCode::BAD_REQUEST,
            "failed to get socket addr".to_owned(),
        )
    })?;

    TcpListener::bind(tcp_addr_obj).await.map_err(|err| {
        error!("Failed to bind tcp address {tcp_addr_obj:?}: {err}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to bind tcp address {tcp_addr_obj:?}: {err}"),
        )
    })
}
