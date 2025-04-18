use std::{net::ToSocketAddrs, time::Duration};

use axum::{
    Json,
    body::Body,
    extract::{FromRequest, Request, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use i_slint_backend_winit::WinitWindowAccessor;
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Model, VecModel};
use tokio::net::TcpListener;
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{Span, debug, error, info};

use super::model::{InstanceData, ScopeData, ServerState};
use crate::{
    bridges::ui_state::sync_scoped_instance,
    daemon::model::InstanceDataPure,
    ui::{Instance, InstanceBridge, Scope, ScopeBridge},
};

pub fn router(state: ServerState) -> axum::Router {
    let cors_state = state.clone();
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(AllowOrigin::async_predicate(
            |origin, _request_parts| async move {
                let scopes = cors_state.scopes.read().await;
                let allowed_origin = scopes
                    .iter()
                    .map(|scope| scope.host.to_string())
                    .collect::<Vec<_>>();
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
                    get(get_instances)
                        .post(launch_instance)
                        .delete(close_instance),
                )
                .route("/popup", post(popup_window))
                .layer(cors_layer)
                .with_state(state.clone()),
        )
        .merge(
            axum::Router::new()
                .route(
                    "/connect",
                    get(get_control_status)
                        .post(request_control)
                        .put(update_website_info),
                )
                .route(
                    "/version",
                    get(|| async { Json(env!("CARGO_PKG_VERSION")) }),
                )
                .layer(any_origin_layer)
                .with_state(state.clone()),
        )
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

#[derive(Serialize)]
struct InstanceResponse {
    label: String,
    remote: String,
    local: String,
    /// deprecated
    from: String,
    /// deprecated
    to: String,
    latency: i32,
}

async fn get_instances(
    State(state): State<ServerState>, headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let scope = headers
        .get("Origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();
    let instances = state.instances.read().await;
    let instances = instances
        .iter()
        .filter_map(|instance| {
            if instance.scope_host == scope {
                Some(InstanceResponse {
                    label: instance.label.clone(),
                    remote: instance.remote.clone(),
                    local: instance.local.clone(),
                    from: instance.local.clone(),
                    to: instance.remote.clone(),
                    latency: instance.latency,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(Json(instances))
}

async fn launch_instance(
    State(state): State<ServerState>, headers: HeaderMap,
    axum::Json(instance_data): axum::Json<InstanceData>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let scope = headers
        .get("Origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
        .to_owned();

    let mut tcp_addr_obj = instance_data.local.to_socket_addrs().map_err(|err| {
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
    let local = listener
        .local_addr()
        .expect("failed to bind port")
        .to_string();
    let remote = instance_data.remote.clone();
    info!("CREATE tcp server: {local} <- wsrx -> {remote}",);

    let mut instances = state.instances.write().await;
    if instances.iter().any(|i| i.local == local) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Instance {} already exists", local),
        ));
    }

    let instance = InstanceData {
        label: instance_data.label.clone(),
        remote: instance_data.remote.clone(),
        local: local.clone(),
        latency: -1,
        scope_host: scope.clone(),
        handle: Some(tokio::task::spawn(async move {
            loop {
                let remote = remote.clone();
                let Ok((tcp, _)) = listener.accept().await else {
                    error!("Failed to accept tcp connection, exiting.");
                    return;
                };
                let peer_addr = tcp.peer_addr().unwrap();
                tokio::spawn(async move {
                    info!("LINK {remote} <- wsrx -> {peer_addr}");
                    let (ws, _) = match tokio_tungstenite::connect_async(&remote).await {
                        Ok(ws) => ws,
                        Err(e) => {
                            error!("Failed to connect to {remote}: {e}");
                            return;
                        }
                    };
                    match wsrx::proxy(ws.into(), tcp).await {
                        Ok(_) => {}
                        Err(e) => {
                            info!("REMOVE {remote} <- wsrx -> {peer_addr}: {e}");
                        }
                    }
                });
            }
        })),
    };
    let instance_resp: InstanceDataPure = (&instance).into();
    instances.push(instance);

    match slint::invoke_from_event_loop(move || {
        let ui_handle = state.ui.upgrade().unwrap();
        let instance_bridge = ui_handle.global::<InstanceBridge>();
        let instances = instance_bridge.get_instances();
        let instances = instances
            .as_any()
            .downcast_ref::<VecModel<Instance>>()
            .unwrap();
        let instance = Instance {
            label: instance_data.label.into(),
            remote: instance_data.remote.into(),
            local: local.into(),
            latency: -1,
            scope_host: scope.into(),
        };
        instances.push(instance);
        sync_scoped_instance(ui_handle.as_weak());
    }) {
        Ok(_) => {
            debug!("Added instance to UI");
            Ok(Json(instance_resp))
        }
        Err(e) => {
            debug!("Failed to update UI: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to update UI".to_owned(),
            ))
        }
    }
}

#[derive(Deserialize)]
struct CloseInstanceRequest {
    #[serde(alias = "key")]
    pub local: String,
}

async fn close_instance(
    State(state): State<ServerState>, headers: HeaderMap,
    axum::Json(req): axum::Json<CloseInstanceRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let scope = headers
        .get("Origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    let mut instances = state.instances.write().await;

    if let Some(tunnel) = instances.iter().find(|v| v.local == req.local) {
        if tunnel.scope_host != scope {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Tunnel {} not found in scope {}", req.local, scope),
            ));
        }
        info!(
            "CLOSE tcp server: {} <- wsrx -> {}",
            req.local, tunnel.remote
        );
        if let Some(handle) = tunnel.handle.as_ref() {
            handle.abort();
        }

        instances.retain(|i| i.local != req.local);

        match slint::invoke_from_event_loop(move || {
            let ui_handle = state.ui.upgrade().unwrap();
            let instance_bridge = ui_handle.global::<InstanceBridge>();
            let instances = instance_bridge.get_instances();
            let instances = instances
                .as_any()
                .downcast_ref::<VecModel<Instance>>()
                .unwrap();
            let mut index = 0;
            for i in instances.iter() {
                if i.local == req.local {
                    break;
                }
                index += 1;
            }
            instances.remove(index);
            sync_scoped_instance(ui_handle.as_weak());
        }) {
            Ok(_) => {
                debug!("Removed instance from UI");
            }
            Err(e) => {
                debug!("Failed to sync state: {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to update UI".to_owned(),
                ));
            }
        }
        Ok(StatusCode::OK)
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            format!("Tunnel {} not found", req.local),
        ))
    }
}

async fn get_control_status(
    State(state): State<ServerState>, headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let req_scope = headers
        .get("Origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    let scopes = state.scopes.read().await;
    let scope = scopes.iter().find(|s| s.host == req_scope);
    if let Some(scope) = scope {
        if scope.state == "pending" {
            Ok(StatusCode::CREATED)
        } else {
            Ok(StatusCode::ACCEPTED)
        }
    } else {
        Err((
            StatusCode::FORBIDDEN,
            format!("Scope {} not found", req_scope),
        ))
    }
}

async fn request_control(
    State(state): State<ServerState>, headers: HeaderMap, req: Request<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let req_scope = headers
        .get("Origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    let json_body = axum::Json::<ScopeData>::from_request(req, &state)
        .await
        .ok();
    let (scope_name, scope_features) = if let Some(json_body) = json_body {
        (json_body.name.clone(), json_body.features.clone())
    } else {
        (req_scope.clone(), vec!["basic".to_string()])
    };

    let mut scopes = state.scopes.write().await;
    if scopes.iter().any(|scope| scope.host == req_scope) {
        return Ok(StatusCode::ACCEPTED);
    }
    let scope_name = if scope_name.is_empty() {
        req_scope.clone()
    } else {
        scope_name.clone()
    };
    let scope = ScopeData {
        name: scope_name.clone(),
        host: req_scope.clone(),
        state: "pending".to_string(),
        features: scope_features.clone(),
    };
    scopes.push(scope);

    match slint::invoke_from_event_loop(move || {
        let ui_handle = state.ui.upgrade().unwrap();
        let scope_bridge = ui_handle.global::<ScopeBridge>();
        let scopes = scope_bridge.get_scopes();
        let scopes = scopes.as_any().downcast_ref::<VecModel<Scope>>().unwrap();
        if scopes.iter().any(|scope| scope.host == req_scope) {
            return;
        }
        let scope = Scope {
            host: req_scope.clone().into(),
            name: scope_name.into(),
            state: "pending".into(),
            features: scope_features.join(",").into(),
        };
        scopes.push(scope);
    }) {
        Ok(_) => {
            debug!("Added scope to UI");
            Ok(StatusCode::OK)
        }
        Err(e) => {
            debug!("Failed to update UI: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to sync state".to_owned(),
            ))
        }
    }
}

async fn update_website_info(
    State(state): State<ServerState>, headers: HeaderMap,
    axum::Json(scope_data): axum::Json<ScopeData>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let req_scope = headers
        .get("Origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    let mut scopes = state.scopes.write().await;
    if let Some(scope) = scopes.iter_mut().find(|s| s.host == req_scope) {
        scope.name = scope_data.name.clone();
        scope.features = scope_data.features.clone();
        let state_d = scope.state.clone();

        match slint::invoke_from_event_loop(move || {
            let ui_handle = state.ui.upgrade().unwrap();
            let scope_bridge = ui_handle.global::<ScopeBridge>();
            let scopes = scope_bridge.get_scopes();
            let scopes = scopes.as_any().downcast_ref::<VecModel<Scope>>().unwrap();
            let mut index = 0;
            for i in scopes.iter() {
                if i.host == req_scope {
                    break;
                }
                index += 1;
            }
            let scope = Scope {
                host: req_scope.clone().into(),
                name: scope_data.name.clone().into(),
                state: state_d.into(),
                features: scope_data.features.join(",").into(),
            };
            scopes.set_row_data(index, scope);
        }) {
            Ok(_) => {
                debug!("Updated scope in UI");
            }
            Err(e) => {
                debug!("Failed to update UI: {e}");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to update UI".to_owned(),
                ));
            }
        }

        Ok(StatusCode::OK)
    } else {
        Err((
            StatusCode::FORBIDDEN,
            format!("Scope {} not found", req_scope),
        ))
    }
}

async fn popup_window(
    State(state): State<ServerState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    slint::invoke_from_event_loop(move || {
        let ui_handle = state.ui.upgrade().unwrap();
        ui_handle.show().ok();
        ui_handle.window().with_winit_window(|winit_window| {
            winit_window.set_minimized(false);
        });
    })
    .ok();
    Ok(StatusCode::OK)
}
