use std::{collections::HashMap, sync::Arc, time::Duration};

use axum::{
    body::Body,
    extract::{FromRef, Path, Request as ExtractRequest, State, WebSocketUpgrade},
    http::{Request, StatusCode, header::CONTENT_TYPE},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Deserialize;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
};
use tokio_util::sync::CancellationToken;
use tower_http::trace::TraceLayer;
use tracing::{Span, error, info};
use wsrx::proxy;

use crate::cli::logger::init_logger;

/// Launch the server with the given host, port, and secret.
pub async fn launch(
    host: Option<String>, port: Option<u16>, secret: Option<String>, log_json: Option<bool>,
) {
    let log_json = log_json.unwrap_or(false);
    init_logger(log_json);
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

type ConnectionMap = Arc<RwLock<HashMap<String, String>>>;

/// The global state of the server.
#[derive(Clone, FromRef)]
pub struct GlobalState {
    pub secret: Option<String>,
    pub connections: ConnectionMap,
}

/// Build the router with the given secret.
fn build_router(secret: Option<String>) -> axum::Router {
    let state = GlobalState {
        secret,
        connections: Default::default(),
    };
    axum::Router::new()
        .route(
            "/pool",
            get(get_tunnels).post(launch_tunnel).delete(close_tunnel),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            |State(secret): State<Option<String>>, req: ExtractRequest, next: Next| async move {
                if let Some(secret) = secret {
                    if let Some(auth) = req.headers().get("authorization")
                        && auth.to_str().map_err(|_| StatusCode::UNAUTHORIZED)? == secret
                    {
                        return Ok(next.run(req).await);
                    }
                    return Err(StatusCode::UNAUTHORIZED);
                }
                Ok(next.run(req).await)
            },
        ))
        .route("/traffic/{*key}", get(process_traffic).options(ping))
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

/// The request body for launching a tunnel.
#[derive(Deserialize)]
struct TunnelRequest {
    pub from: String,
    pub to: String,
}

/// Launch a tunnel from the given address to the given address.
async fn launch_tunnel(
    State(connections): State<ConnectionMap>, axum::Json(req): axum::Json<TunnelRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut pool = connections.write().await;
    pool.insert(req.from, req.to);
    Ok(StatusCode::CREATED)
}

/// Get the list of tunnels.
async fn get_tunnels(State(connections): State<ConnectionMap>) -> impl IntoResponse {
    let pool = connections.read().await;
    let resp = serde_json::to_string::<HashMap<String, String>>(&pool).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to serialize pool: {e}"),
        )
    });
    axum::response::Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(resp.unwrap())
        .unwrap()
}

/// The request body for closing a tunnel.
#[derive(Deserialize)]
struct CloseTunnelRequest {
    pub key: String,
}

/// Close a tunnel with the given key.
async fn close_tunnel(
    State(connections): State<ConnectionMap>, axum::Json(req): axum::Json<CloseTunnelRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut pool = connections.write().await;
    if pool.remove(&req.key).is_some() {
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, "not found"))
    }
}

/// Process the traffic between the WebSocket and TCP connection.
async fn process_traffic(
    State(connections): State<ConnectionMap>, Path(key): Path<String>, ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let pool = connections.read().await;
    if let Some(conn) = pool.get(&key) {
        let tcp_addr = conn.to_owned();
        Ok(ws.on_upgrade(move |socket| async move {
            let tcp = TcpStream::connect(&tcp_addr).await;
            if tcp.is_err() {
                error!("failed to connect to tcp server: {}", tcp.unwrap_err());
                return;
            }
            let tcp = tcp.unwrap();
            proxy(socket.into(), tcp, CancellationToken::new())
                .await
                .ok();
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "not found"))
    }
}

/// Ping the server to check if the connection is alive.
async fn ping(
    State(connections): State<ConnectionMap>, Path(key): Path<String>,
) -> impl IntoResponse {
    let pool = connections.read().await;
    if pool.get(&key).is_some() {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
