use std::time::Duration;

use axum::{body::Body, extract::FromRef, http::Request, response::Response};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, Span};

use crate::cli::logger::init_logger;

pub async fn launch(host: Option<String>, port: Option<u16>, secret: Option<String>) {
    init_logger();
    let router = build_router(secret);
    let listener = TcpListener::bind(&format!(
        "{}:{}",
        host.unwrap_or(String::from("127.0.0.1")),
        port.unwrap_or(0)
    ))
    .await
    .expect("failed to bind port");
    info!(
        "wsrx server is listening on {}",
        listener.local_addr().expect("failed to bind port")
    );
    info!(
        "you can access manage api at http://{}/pool",
        listener.local_addr().expect("failed to bind port")
    );
    axum::serve(listener, router)
        .await
        .expect("failed to launch server");
}

#[derive(Clone, FromRef)]
pub struct GlobalState {
    pub secret: Option<String>,
}

fn build_router(secret: Option<String>) -> axum::Router {
    let state = GlobalState { secret };
    axum::Router::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    tracing::info_span!(
                            "http",
                            method = %request.method(),
                            uri = %request.uri().path(),
                    )
                })
                .on_request(())
                .on_response(|response: &Response, latency: Duration, _span: &Span| {
                    info!("[{}] in {}ms", response.status(), latency.as_millis());
                }),
        )
        .with_state::<()>(state)
}
