use std::{collections::HashMap, net::ToSocketAddrs, sync::Arc, time::Duration};

use axum::{
    body::Body,
    extract::{FromRef, Request as ExtractRequest, State},
    http::{header::CONTENT_TYPE, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::RwLock, task::JoinHandle};
use tower_http::trace::TraceLayer;
use tracing::{error, info, Span};
use wsrx::proxy;

use crate::cli::logger::init_logger;

pub async fn launch(host: Option<String>, port: Option<u16>, secret: Option<String>, log_json: Option<bool>) {
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
        "wsrx daemon is listening on {}",
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
    pub connections: Arc<RwLock<HashMap<String, Tunnel>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tunnel {
    pub from: String,
    pub to: String,
    #[serde(skip)]
    pub handle: Option<JoinHandle<()>>,
}

fn build_router(secret: Option<String>) -> axum::Router {
    let state = GlobalState {
        secret,
        connections: Arc::new(RwLock::new(HashMap::new())),
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
                    if let Some(auth) = req.headers().get("authorization") {
                        if auth.to_str().map_err(|_| StatusCode::UNAUTHORIZED)? == secret {
                            return Ok(next.run(req).await);
                        }
                    }
                    return Err(StatusCode::UNAUTHORIZED);
                }
                Ok(next.run(req).await)
            },
        ))
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

#[derive(Deserialize)]
struct TunnelRequest {
    pub from: String,
    pub to: String,
}

async fn launch_tunnel(
    State(connections): State<Arc<RwLock<HashMap<String, Tunnel>>>>,
    axum::Json(req): axum::Json<TunnelRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut pool = connections.write().await;
    // pool.insert(req.from, req.to);
    let mut tcp_addr_obj = req.from.to_socket_addrs().map_err(|err| {
        error!("Failed to parse from address: {err}");
        (StatusCode::BAD_REQUEST, "failed to parse from address")
    })?;
    let tcp_addr_obj = tcp_addr_obj
        .next()
        .ok_or((StatusCode::BAD_REQUEST, "failed to get socket addr"))?;
    let tunnel = Tunnel {
        from: req.from.clone(),
        to: req.to.clone(),
        handle: Some(tokio::task::spawn(async move {
            let listener = TcpListener::bind(tcp_addr_obj)
                .await
                .expect("failed to bind port");
            info!(
                "CREATE tcp server: {} <--wsrx--> {}",
                listener.local_addr().expect("failed to bind port"),
                req.to
            );
            loop {
                let Ok((tcp, _)) = listener.accept().await else {
                    error!("Failed to accept tcp connection, exiting.");
                    return;
                };
                let url = req.to.clone();
                let peer_addr = tcp.peer_addr().unwrap();
                info!("CREATE remote <-wsrx-> {}", peer_addr);
                tokio::spawn(async move {
                    let (ws, _) = match tokio_tungstenite::connect_async(&url).await {
                        Ok(ws) => ws,
                        Err(e) => {
                            error!("Failed to connect to {url}: {e}");
                            return;
                        }
                    };
                    match proxy(ws.into(), tcp).await {
                        Ok(_) => {}
                        Err(e) => {
                            info!("REMOVE {url} <-wsrx-> {peer_addr}, {e}");
                        }
                    }
                });
            }
        })),
    };
    let resp = serde_json::to_string(&tunnel).map_err(|e| {
        error!("Failed to serialize tunnel: {e:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to serialize tunnel",
        )
    });
    pool.insert(req.from.clone(), tunnel);
    resp
}

async fn get_tunnels(
    State(connections): State<Arc<RwLock<HashMap<String, Tunnel>>>>,
) -> impl IntoResponse {
    let pool = connections.read().await;
    let resp = serde_json::to_string::<HashMap<String, Tunnel>>(&pool).map_err(|e| {
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

#[derive(Deserialize)]
struct CloseTunnelRequest {
    pub key: String,
}

async fn close_tunnel(
    State(connections): State<Arc<RwLock<HashMap<String, Tunnel>>>>,
    axum::Json(req): axum::Json<CloseTunnelRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut pool = connections.write().await;
    let tunnel = pool.get(&req.key);
    if let Some(tunnel) = tunnel {
        if let Some(handle) = tunnel.handle.as_ref() {
            handle.abort();
        }
        pool.remove(&req.key);
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, "not found"))
    }
}
