use tokio::{net::TcpListener, task::JoinHandle};
use tracing::info;

use crate::cli::logger::init_logger;

#[derive(Debug, Clone)]
enum Direction {
    Ws2Tcp,
    Tcp2Ws,
}

#[derive(Debug)]
struct Connection {
    ws: String,
    tcp: String,
    c_type: Direction,
    handle: JoinHandle<()>,
}

pub async fn launch_daemon(host: Option<String>, port: Option<u16>) {
    init_logger();
    let router = build_router();
    let listener = TcpListener::bind(&format!(
        "{}:{}",
        host.unwrap_or(String::from("127.0.0.1")),
        port.unwrap_or(0)
    ))
    .await
    .expect("failed to bind port");
    info!(
        "wsrx daemon is listening on {}",
        listener.local_addr().expect("failed to bind port")
    );
    info!(
        "you can access manage api at http://{}/pool",
        listener.local_addr().expect("failed to bind port")
    );
    info!(
        "new proxy tokens can be connected at http://{}/traffic/<id>",
        listener.local_addr().expect("failed to bind port")
    );
    axum::serve(listener, router)
        .await
        .expect("failed to launch server");
}

fn build_router() -> axum::Router {
    axum::Router::new()
}
