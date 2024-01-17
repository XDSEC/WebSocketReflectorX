use std::{collections::HashMap, sync::Arc, time::Duration, net::ToSocketAddrs};

use axum::{
    body::Body,
    extract::{FromRef, Path, Request as ERequest, State, WebSocketUpgrade},
    http::{header::CONTENT_TYPE, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::RwLock,
    task::JoinHandle,
};
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, Span};
use wsrx::proxy;

use crate::cli::logger::init_logger;

#[derive(Debug, Clone, Serialize)]
enum Direction {
    Ws2Tcp,
    Tcp2Ws,
}

#[derive(Debug, Serialize)]
struct Connection {
    ws: String,
    tcp: String,
    direction: Direction,
    #[serde(skip)]
    handle: Option<JoinHandle<()>>,
}

#[derive(Debug, Serialize)]
struct Pool {
    connections: HashMap<String, Connection>,
}

#[derive(Error, Debug)]
enum WsrxCliError {
    #[error("wsrx error: {0}")]
    WsrxError(#[from] wsrx::Error),
    #[error("url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("invalid socket address")]
    InvalidSocketAddr,
}

static POOL: Lazy<Arc<RwLock<Pool>>> = Lazy::new(|| Arc::new(RwLock::new(Pool::new())));

async fn proxy_ws_addr(addr: impl AsRef<str>, tcp: TcpStream) -> Result<(), wsrx::Error> {
    let peer_addr = tcp.peer_addr().unwrap();
    let (ws, _) = tokio_tungstenite::connect_async(addr.as_ref()).await?;
    proxy(ws.into(), tcp).await?;
    info!("REMOVE remote <-wsrx-> {}", peer_addr);
    Ok(())
}

impl Pool {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    async fn create_ws_tunnel(&mut self, ws_key: String, to: String) -> Result<(), WsrxCliError> {
        let _tcp_addr = to.to_socket_addrs().map_err(|_| WsrxCliError::InvalidSocketAddr)?;
        self.connections.insert(
            ws_key.clone(),
            Connection {
                ws: ws_key.clone(),
                tcp: to.clone(),
                direction: Direction::Ws2Tcp,
                handle: None,
            },
        );
        Ok(())
    }

    async fn create_tcp_tunnel(
        &mut self,
        tcp_addr: String,
        to: String,
    ) -> Result<(), WsrxCliError> {
        let mut tcp_addr_obj = tcp_addr
            .to_socket_addrs()
            .map_err(|_| WsrxCliError::InvalidSocketAddr)?;
        let tcp_addr_obj = tcp_addr_obj.next().ok_or(WsrxCliError::InvalidSocketAddr)?;
        self.connections.insert(
            tcp_addr.clone(),
            Connection {
                ws: to.clone(),
                tcp: tcp_addr.clone(),
                direction: Direction::Tcp2Ws,
                handle: Some(tokio::task::spawn(async move {
                    let listener = TcpListener::bind(tcp_addr_obj)
                        .await
                        .expect("failed to bind port");
                    info!(
                        "CREATE tcp server: {} <--wsrx--> {}",
                        listener.local_addr().expect("failed to bind port"),
                        to
                    );
                    loop {
                        let Ok((tcp, _)) = listener.accept().await else {
                            error!("Failed to accept tcp connection, exiting.");
                            return;
                        };
                        let url = to.clone();
                        let peer_addr = tcp.peer_addr().unwrap();
                        info!("CREATE remote <-wsrx-> {}", peer_addr);
                        tokio::spawn(async move {
                            match proxy_ws_addr(url, tcp).await {
                                Ok(_) => {}
                                Err(e) => {
                                    info!("REMOVE remote <-wsrx-> {} with error", peer_addr);
                                    debug!("TCP connection closed: {}", e);
                                }
                            }
                        });
                    }
                })),
            },
        );
        Ok(())
    }
}

pub async fn launch_daemon(host: Option<String>, port: Option<u16>, secret: Option<String>) {
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

#[derive(Clone, FromRef)]
pub struct GlobalState {
    pub secret: Option<String>,
}

fn build_router(secret: Option<String>) -> axum::Router {
    let state = GlobalState { secret };
    axum::Router::new()
        .route(
            "/pool",
            post(launch_new_tunnel)
                .get(get_tunnels)
                .delete(close_tunnel),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            |State(secret): State<Option<String>>, req: ERequest, next: Next| async move {
                if let Some(secret) = secret {
                    if let Some(auth) = req.headers().get("authorization") {
                        if auth.to_str().map_err(|_| StatusCode::UNAUTHORIZED)? == &secret {
                            return Ok(next.run(req).await);
                        }
                    }
                    return Err(StatusCode::UNAUTHORIZED);
                }
                Ok(next.run(req).await)
            },
        ))
        .route("/traffic/*key", get(process_traffic).options(ping))
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
    pub direction: String,
    pub from: String,
    pub to: String,
}

async fn launch_new_tunnel(
    axum::Json(req): axum::Json<TunnelRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    match req.direction.as_str() {
        "ws2tcp" => {
            POOL.write()
                .await
                .create_ws_tunnel(req.from, req.to)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "failed to create tunnel"))?;
        }
        "tcp2ws" => {
            POOL.write()
                .await
                .create_tcp_tunnel(req.from, req.to)
                .await
                .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "failed to create tunnel"))?;
        }
        _ => return Err((StatusCode::BAD_REQUEST, "invalid direction")),
    }
    Ok(StatusCode::CREATED)
}

async fn get_tunnels() -> impl IntoResponse {
    let pool = POOL.read().await;
    let resp = serde_json::to_string(&pool.connections).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to serialize pool: {}", e),
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
    axum::Json(req): axum::Json<CloseTunnelRequest>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let mut pool = POOL.write().await;
    if let Some(conn) = pool.connections.remove(&req.key) {
        if let Some(handle) = conn.handle {
            handle.abort();
        }
        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, "not found"))
    }
}

async fn process_traffic(
    Path(key): Path<String>,
    ws: Option<WebSocketUpgrade>,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    let pool = POOL.read().await;
    if let Some(conn) = pool.connections.get(&key) {
        let tcp_addr = conn.tcp.clone();
        if let Some(ws) = ws {
            Ok(ws.on_upgrade(move |socket| async move {
                let tcp = TcpStream::connect(&tcp_addr).await;
                if tcp.is_err() {
                    error!("failed to connect to tcp server: {}", tcp.unwrap_err());
                    return;
                }
                let tcp = tcp.unwrap();
                proxy(socket.into(), tcp).await.ok();
            }))
        } else {
            Err((StatusCode::NO_CONTENT, ""))
        }
    } else {
        Err((StatusCode::NOT_FOUND, "not found"))
    }
}

async fn ping(Path(key): Path<String>) -> impl IntoResponse {
    let pool = POOL.read().await;
    if let Some(_) = pool.connections.get(&key) {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
