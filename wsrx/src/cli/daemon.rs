use std::{
    collections::HashMap,
    net::ToSocketAddrs,
    sync::{Arc, RwLock as SyncRwLock},
    time::Duration,
};

use axum::{
    body::Body,
    extract::{FromRef, Request as ExtractRequest, State},
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
    Json,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::RwLock, task::JoinHandle};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use tracing::{debug, error, info, Span};
use wsrx::proxy;

use crate::cli::logger::init_logger;

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

static ALLOWED_ORIGINS: Lazy<Arc<SyncRwLock<Vec<String>>>> =
    Lazy::new(|| Arc::new(SyncRwLock::new(Vec::new())));

static PENDING_ORIGINS: Lazy<Arc<SyncRwLock<Vec<String>>>> =
    Lazy::new(|| Arc::new(SyncRwLock::new(Vec::new())));

fn build_router(secret: Option<String>) -> axum::Router {
    let state = GlobalState {
        secret,
        connections: Arc::new(RwLock::new(HashMap::new())),
    };
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(AllowOrigin::predicate(
            |origin: &HeaderValue, _request_parts: &_| {
                let allowed_origin = ALLOWED_ORIGINS.read().unwrap();
                for o in allowed_origin.iter() {
                    if origin.to_str().unwrap_or("").ends_with(o) {
                        return true;
                    }
                }
                false
            },
        ));
    let any_origin_layer = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(AllowOrigin::any());
    axum::Router::new()
        .nest(
            "/",
            axum::Router::new()
                .route(
                    "/pool",
                    get(get_tunnels).post(launch_tunnel).delete(close_tunnel),
                )
                .route(
                    "/cors",
                    get(get_origins)
                        .post(add_allowed_origin)
                        .delete(remove_allowed_origin),
                )
                .layer(cors_layer)
                .with_state(state.clone()),
        )
        .nest(
            "/",
            axum::Router::new()
                .route(
                    "/connect",
                    get(get_cors_status)
                        .post(add_pending_origin)
                        .delete(remove_pending_origin),
                )
                .layer(any_origin_layer)
                .with_state(state.clone()),
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
                .on_failure(())
                .on_response(|response: &Response, latency: Duration, _span: &Span| {
                    debug!(
                        "API Request [{}] in {}ms",
                        response.status(),
                        latency.as_millis()
                    );
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
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut pool = connections.write().await;
    // pool.insert(req.from, req.to);
    let mut tcp_addr_obj = req.from.to_socket_addrs().map_err(|err| {
        error!("Failed to parse from address: {err}");
        (
            StatusCode::BAD_REQUEST,
            "failed to parse from address".to_owned(),
        )
    })?;
    let tcp_addr_obj = tcp_addr_obj.next().ok_or((
        StatusCode::BAD_REQUEST,
        "failed to get socket addr".to_owned(),
    ))?;
    let listener = TcpListener::bind(tcp_addr_obj).await.map_err(|err| {
        error!("Failed to bind tcp address {tcp_addr_obj:?}: {err}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to bind tcp address {tcp_addr_obj:?}: {err}"),
        )
    })?;
    info!(
        "CREATE tcp server: {} <--wsrx--> {}",
        listener.local_addr().expect("failed to bind port"),
        req.to
    );
    let tunnel = Tunnel {
        from: listener
            .local_addr()
            .expect("failed to bind port")
            .to_string(),
        to: req.to.clone(),
        handle: Some(tokio::task::spawn(async move {
            loop {
                let Ok((tcp, _)) = listener.accept().await else {
                    error!("Failed to accept tcp connection, exiting.");
                    return;
                };
                let url = req.to.clone();
                let peer_addr = tcp.peer_addr().unwrap();
                info!("LINK {url} <-wsrx-> {peer_addr}");
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
                            info!("REMOVE {url} <-wsrx-> {peer_addr}: {e}");
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
            format!("Failed to serialize tunnel: {e:?}"),
        )
    });
    pool.insert(tunnel.from.clone(), tunnel);
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
        info!("REMOVE {} <-wsrx-> {}", tunnel.from, tunnel.to);
        pool.remove(&req.key);
        Ok(StatusCode::OK)
    } else {
        error!("Tunnel does not exist: {}", req.key);
        Err((StatusCode::NOT_FOUND, "not found"))
    }
}

#[derive(Serialize)]
struct OriginResponse {
    pub allowed: Vec<String>,
    pub waitlist: Vec<String>,
}

async fn get_origins() -> Result<impl IntoResponse, (StatusCode, String)> {
    let allowed_origin = ALLOWED_ORIGINS.read().unwrap();
    let waitlist = PENDING_ORIGINS.read().unwrap();
    let resp = serde_json::to_string(&OriginResponse {
        allowed: allowed_origin.clone(),
        waitlist: waitlist.clone(),
    })
    .map_err(|e| {
        error!("Failed to serialize origin response: {e:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize origin response: {e:?}"),
        )
    })?;
    Ok(Json(resp))
}

async fn add_allowed_origin(
    axum::Json(req): axum::Json<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut allowed_origin = ALLOWED_ORIGINS.write().map_err(|_| {
        error!("Failed to lock allowed origin");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to lock allowed origin",
        )
    })?;
    allowed_origin.push(req);
    Ok(StatusCode::OK)
}

async fn remove_allowed_origin(
    axum::Json(req): axum::Json<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut allowed_origin = ALLOWED_ORIGINS.write().map_err(|_| {
        error!("Failed to lock allowed origin");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to lock allowed origin",
        )
    })?;
    allowed_origin.retain(|o| o != &req);
    Ok(StatusCode::OK)
}

async fn get_cors_status(headers: HeaderMap) -> impl IntoResponse {
    let allowed_origin = ALLOWED_ORIGINS
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let origin = headers.get("origin").map(|o| o.to_str().unwrap_or(""));
    let resp = match origin {
        Some(origin) => {
            if allowed_origin.contains(&origin.to_string()) {
                Ok(StatusCode::ACCEPTED)
            } else {
                Err(StatusCode::FORBIDDEN)
            }
        }
        None => Ok(StatusCode::ACCEPTED),
    };
    resp
}

async fn add_pending_origin(
    axum::Json(req): axum::Json<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut waitlist = PENDING_ORIGINS.write().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to lock origin waitlist",
        )
    })?;
    waitlist.push(req);
    Ok(StatusCode::OK)
}

async fn remove_pending_origin(
    axum::Json(req): axum::Json<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut waitlist = PENDING_ORIGINS.write().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to lock origin waitlist",
        )
    })?;
    waitlist.retain(|o| o != &req);
    Ok(StatusCode::OK)
}
