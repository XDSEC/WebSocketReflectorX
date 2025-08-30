use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, RwLock as SyncRwLock},
    time::Duration,
};

use axum::{
    Json,
    body::Body,
    extract::{FromRef, Request as ExtractRequest, State},
    http::{HeaderMap, HeaderValue, Method, Request, StatusCode, header::CONTENT_TYPE},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{Span, debug, error, info};
use wsrx::{
    tunnel::{Tunnel, TunnelConfig},
    utils::create_tcp_listener,
};

use crate::cli::logger::init_logger;

pub async fn launch(
    host: Option<String>, port: Option<u16>, secret: Option<String>, log_json: Option<bool>,
    heartbeat: Option<u64>,
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
    if let Some(interval) = heartbeat {
        tokio::spawn(heartbeat_watchdog(interval));
    }
    axum::serve(listener, router)
        .await
        .expect("failed to launch server");
}

type ConnectionMap = Arc<RwLock<HashMap<String, Tunnel>>>;

#[derive(Clone, FromRef)]
pub struct GlobalState {
    pub secret: Option<String>,
    pub connections: ConnectionMap,
}

static ALLOWED_ORIGINS: Lazy<Arc<SyncRwLock<Vec<String>>>> =
    Lazy::new(|| Arc::new(SyncRwLock::new(Vec::new())));

static PENDING_ORIGINS: Lazy<Arc<SyncRwLock<Vec<String>>>> =
    Lazy::new(|| Arc::new(SyncRwLock::new(Vec::new())));

static HEARTBEAT_TIME: Lazy<Arc<SyncRwLock<DateTime<Utc>>>> =
    Lazy::new(|| Arc::new(SyncRwLock::new(Utc::now())));

async fn heartbeat_watchdog(interval: u64) {
    loop {
        tokio::time::sleep(Duration::from_secs(interval)).await;
        let last_heartbeat = HEARTBEAT_TIME.read().ok();
        if last_heartbeat.is_none() {
            continue;
        }
        let last_heartbeat = last_heartbeat.unwrap();
        if Utc::now()
            .signed_duration_since(*last_heartbeat)
            .num_seconds()
            > interval as i64
        {
            error!("Heartbeat timeout, last active at {last_heartbeat}, exiting.");
            std::process::exit(0);
        } else {
            debug!("Heartbeat check passed, last active at {last_heartbeat}.");
        }
    }
}

fn build_router(secret: Option<String>) -> axum::Router {
    let state = GlobalState {
        secret,
        connections: Default::default(),
    };
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any)
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
        .allow_headers(Any)
        .allow_origin(Any);
    axum::Router::new()
        .merge(
            axum::Router::new()
                .route(
                    "/pool",
                    get(get_tunnels).post(launch_tunnel).delete(close_tunnel),
                )
                .route("/heartbeat", get(update_heartbeat))
                .route(
                    "/access",
                    get(get_origins)
                        .post(add_allowed_origin)
                        .delete(remove_allowed_origin),
                )
                .layer(cors_layer)
                .with_state(state.clone()),
        )
        .merge(
            axum::Router::new()
                .route("/connect", get(get_cors_status).post(add_pending_origin))
                .layer(any_origin_layer)
                .with_state(state.clone()),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            |State(secret): State<Option<String>>, req: ExtractRequest, next: Next| async move {
                if let Some(secret) = secret {
                    if let Some(auth) = req.headers().get("authorization")
                        && auth.to_str().map_err(|_| StatusCode::UNAUTHORIZED)? == secret {
                            return Ok(next.run(req).await);
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

async fn launch_tunnel(
    State(connections): State<ConnectionMap>, axum::Json(req): axum::Json<TunnelConfig>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut pool = connections.write().await;

    let listener = create_tcp_listener(req.local.as_str()).await?;

    let tunnel = Tunnel::new(req.remote, listener);

    let resp = serde_json::to_string(&tunnel).map_err(|e| {
        error!("Failed to serialize tunnel: {e:?}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize tunnel: {e:?}"),
        )
    });

    pool.insert(tunnel.local.clone(), tunnel);

    resp
}

async fn get_tunnels(State(connections): State<ConnectionMap>) -> impl IntoResponse {
    let pool = connections.read().await;
    let resp = serde_json::to_string(pool.deref()).map_err(|e| {
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
    State(connections): State<ConnectionMap>, axum::Json(req): axum::Json<CloseTunnelRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    if connections.write().await.remove(&req.key).is_some() {
        Ok(StatusCode::OK)
    } else {
        error!("Tunnel does not exist: {}", req.key);
        Err((StatusCode::NOT_FOUND, "not found"))
    }
}

#[derive(Serialize)]
struct OriginResponse {
    pub allowed: Vec<String>,
    pub pending: Vec<String>,
}

async fn get_origins() -> Result<impl IntoResponse, (StatusCode, String)> {
    let allowed_origin = ALLOWED_ORIGINS.read().unwrap();
    let pending = PENDING_ORIGINS.read().unwrap();
    let resp = OriginResponse {
        allowed: allowed_origin.clone(),
        pending: pending.clone(),
    };
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
    let mut waitlist = PENDING_ORIGINS.write().map_err(|_| {
        error!("Failed to lock origin waitlist");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to lock origin waitlist",
        )
    })?;
    if waitlist.contains(&req) {
        waitlist.retain(|o| o != &req);
    }
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
    let mut waitlist = PENDING_ORIGINS.write().map_err(|_| {
        error!("Failed to lock origin waitlist");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to lock origin waitlist",
        )
    })?;
    if waitlist.contains(&req) {
        waitlist.retain(|o| o != &req);
    }
    allowed_origin.retain(|o| o != &req);
    Ok(StatusCode::OK)
}

async fn get_cors_status(headers: HeaderMap) -> impl IntoResponse {
    let allowed_origins = ALLOWED_ORIGINS
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let pending_origins = PENDING_ORIGINS
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let origin = headers.get("origin").map(|o| o.to_str().unwrap_or(""));
    match origin {
        Some(origin) => {
            if allowed_origins.contains(&origin.to_string()) {
                Ok(StatusCode::ACCEPTED)
            } else if pending_origins.contains(&origin.to_string()) {
                Err(StatusCode::CREATED)
            } else {
                Err(StatusCode::FORBIDDEN)
            }
        }
        None => Ok(StatusCode::ACCEPTED),
    }
}

async fn add_pending_origin(
    axum::Json(req): axum::Json<String>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let allowed_origin = ALLOWED_ORIGINS.read().map_err(|_| {
        error!("Failed to lock allowed origin");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to lock allowed origin",
        )
    })?;
    if allowed_origin.contains(&req) {
        return Ok(StatusCode::ACCEPTED);
    }
    let mut waitlist = PENDING_ORIGINS.write().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "failed to lock origin waitlist",
        )
    })?;
    if waitlist.contains(&req) {
        return Ok(StatusCode::CREATED);
    }
    waitlist.push(req);
    Ok(StatusCode::CREATED)
}

async fn update_heartbeat() -> impl IntoResponse {
    let mut last_heartbeat = HEARTBEAT_TIME.write().unwrap();
    *last_heartbeat = Utc::now();
    StatusCode::OK
}
