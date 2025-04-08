use std::{net::ToSocketAddrs, process, rc::Rc, sync::Arc, time::Duration};

use axum::{
    Json,
    body::Body,
    extract::{FromRequest, State},
    http::{HeaderMap, Method, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Model, VecModel};
use tokio::{net::TcpListener, sync::RwLock, task::JoinHandle};
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{Span, debug, error, info, warn};
use wsrx::proxy;

use crate::{
    bridges::ui_state::sync_scoped_instance,
    ui::{Instance, InstanceBridge, MainWindow, Scope, ScopeBridge},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceData {
    #[serde(default = "default_label")]
    pub label: String,
    #[serde(alias = "to")]
    pub remote: String,
    #[serde(alias = "from")]
    pub local: String,
    #[serde(default)]
    pub latency: String,
    #[serde(default)]
    pub scope_host: String,
    #[serde(skip)]
    pub handle: Option<JoinHandle<()>>,
}

fn default_label() -> String {
    format!("inst-{:06x}", rand::random::<u32>())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeData {
    host: String,
    name: String,
    state: String,
    features: Vec<String>,
}

#[derive(Clone)]
pub struct ServerState {
    pub ui: slint::Weak<MainWindow>,
    pub instances: Arc<RwLock<Vec<InstanceData>>>,
    pub scopes: Arc<RwLock<Vec<ScopeData>>>,
}

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
                .layer(cors_layer)
                .with_state(state.clone()),
        )
        .merge(
            axum::Router::new()
                .route("/connect", get(get_control_status).post(request_control))
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
        latency: "-- ms".to_string(),
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
                    match proxy(ws.into(), tcp).await {
                        Ok(_) => {}
                        Err(e) => {
                            info!("REMOVE {remote} <- wsrx -> {peer_addr}: {e}");
                        }
                    }
                });
            }
        })),
    };
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
            latency: "-- ms".into(),
            scope_host: scope.into(),
        };
        instances.push(instance);
        sync_scoped_instance(ui_handle.as_weak());
    }) {
        Ok(_) => {
            debug!("Added instance to UI");
            Ok(StatusCode::OK)
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
    pub key: String,
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

    if let Some(tunnel) = instances.iter().find(|v| v.local == req.key) {
        if tunnel.scope_host != scope {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Tunnel {} not found in scope {}", req.key, scope),
            ));
        }
        info!("CLOSE tcp server: {} <- wsrx -> {}", req.key, tunnel.remote);
        if let Some(handle) = tunnel.handle.as_ref() {
            handle.abort();
        }

        instances.retain(|i| i.local != req.key);

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
                if i.local == req.key {
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
            format!("Tunnel {} not found", req.key),
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
        (req_scope.clone(), vec!["v0".to_string()])
    };

    let mut scopes = state.scopes.write().await;
    if scopes.iter().any(|scope| scope.host == req_scope) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Scope {} already exists", req_scope),
        ));
    }
    let scope = ScopeData {
        host: if scope_name.is_empty() {
            req_scope.clone()
        } else {
            scope_name.clone()
        },
        name: req_scope.clone(),
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
            name: req_scope.into(),
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

pub fn setup(ui: &MainWindow) {
    use rustls::crypto;

    match crypto::aws_lc_rs::default_provider().install_default() {
        Ok(_) => info!("using `AWS Libcrypto` as default crypto backend."),
        Err(err) => {
            error!("`AWS Libcrypto` is not available: {:?}", err);
            warn!("try to use `ring` as default crypto backend.");
            crypto::ring::default_provider()
                .install_default()
                .inspect_err(|err| {
                    error!("`ring` is not available: {:?}", err);
                    error!("All crypto backend are not available, exiting...");
                    process::exit(1);
                })
                .ok();
            info!("using `ring` as default crypto backend.");
        }
    }

    let handle = ui.as_weak();
    // Initialize the global state
    let instances: Rc<VecModel<Instance>> = Rc::new(VecModel::default());
    let scopes: Rc<VecModel<Scope>> = Rc::new(VecModel::default());
    let scoped_instances: Rc<VecModel<Instance>> = Rc::new(VecModel::default());

    let instances_rc = slint::ModelRc::from(instances.clone());
    let scopes_rc = slint::ModelRc::from(scopes.clone());
    let scoped_instances_rc = slint::ModelRc::from(scoped_instances.clone());

    let instance_bridge = ui.global::<InstanceBridge>();
    instance_bridge.set_instances(instances_rc);
    instance_bridge.set_scoped_instances(scoped_instances_rc);

    let scope_bridge = ui.global::<ScopeBridge>();
    scope_bridge.set_scopes(scopes_rc);

    let state = ServerState {
        ui: handle.clone(),
        instances: Arc::new(RwLock::new(vec![])),
        scopes: Arc::new(RwLock::new(vec![])),
    };

    let state_cloned = state.clone();
    let handle_cloned = handle.clone();

    scope_bridge.on_allow(move |scope_host| {
        let state_cloned = state_cloned.clone();
        let handle_cloned = handle_cloned.clone();
        match slint::spawn_local(async_compat::Compat::new(async move {
            on_scope_allow(&state_cloned, handle_cloned.clone(), scope_host.as_str()).await;
        })) {
            Ok(_) => {}
            Err(e) => {
                debug!("Failed to update scope bridge: {e}");
            }
        }
    });

    let state_cloned = state.clone();
    let handle_cloned = handle.clone();

    scope_bridge.on_del(move |scope_host| {
        let state_cloned = state_cloned.clone();
        let handle_cloned = handle_cloned.clone();
        match slint::spawn_local(async_compat::Compat::new(async move {
            on_scope_del(&state_cloned, handle_cloned.clone(), scope_host.as_str()).await;
        })) {
            Ok(_) => {}
            Err(e) => {
                debug!("Failed to update scope bridge: {e}");
            }
        }
    });

    let router = router(state.clone());

    match slint::spawn_local(async_compat::Compat::new(async move {
        let listener = TcpListener::bind(&format!("{}:{}", "127.0.0.1", 3307))
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
    })) {
        Ok(_) => {
            info!("API server started");
        }
        Err(e) => {
            error!("Failed to start API server: {e}");
        }
    }
}

async fn on_scope_allow(state: &ServerState, ui: slint::Weak<MainWindow>, scope_host: &str) {
    let mut scopes = state.scopes.write().await;
    let scope_name;
    let scope_features;

    if let Some(scope) = scopes.iter_mut().find(|s| s.host == scope_host) {
        scope.state = "allowed".to_string();
        scope_name = scope.name.clone();
        scope_features = scope.features.clone();
    } else {
        return;
    }

    let scope_host = scope_host.to_string();

    match slint::invoke_from_event_loop(move || {
        let ui_handle = ui.upgrade().unwrap();
        let scope_bridge = ui_handle.global::<ScopeBridge>();
        let scopes = scope_bridge.get_scopes();
        let scopes = scopes.as_any().downcast_ref::<VecModel<Scope>>().unwrap();
        let mut index = 0;
        for s in scopes.iter() {
            if s.host == scope_host {
                break;
            }
            index += 1;
        }
        if index < scopes.row_count() {
            scopes.set_row_data(
                index,
                Scope {
                    host: scope_host.into(),
                    name: scope_name.into(),
                    state: "allowed".into(),
                    features: scope_features.join(",").into(),
                },
            );
        }
    }) {
        Ok(_) => {
            debug!("Updated scope state to allowed");
        }
        Err(e) => {
            debug!("Failed to update UI: {e}");
        }
    }
}

async fn on_scope_del(state: &ServerState, ui: slint::Weak<MainWindow>, scope_host: &str) {
    let mut scopes = state.scopes.write().await;
    let removed_scope = scopes
        .iter()
        .position(|s| s.host == scope_host)
        .map(|index| scopes.remove(index));

    let scope_host = scope_host.to_string();

    if let Some(scope) = removed_scope {
        info!("Scope {} removed", scope.host);

        let mut instances = state.instances.write().await;
        instances.retain(|i| {
            if let Some(handle) = i.handle.as_ref() {
                handle.abort();
            }
            i.scope_host != scope.host
        });
    } else {
        return;
    }

    let state = state.clone();
    let instances = state.instances.read().await;
    let instances = instances
        .iter()
        .filter(|i| i.scope_host == scope_host)
        .map(|i| Instance {
            label: i.label.clone().into(),
            remote: i.remote.clone().into(),
            local: i.local.clone().into(),
            latency: "-- ms".into(),
            scope_host: i.scope_host.clone().into(),
        })
        .collect::<Vec<_>>();

    match slint::invoke_from_event_loop(move || {
        let ui_handle = ui.upgrade().unwrap();
        let scope_bridge = ui_handle.global::<ScopeBridge>();
        let scopes = scope_bridge.get_scopes();
        let scopes = scopes.as_any().downcast_ref::<VecModel<Scope>>().unwrap();
        let mut index = 0;
        for s in scopes.iter() {
            if s.host == scope_host {
                break;
            }
            index += 1;
        }
        if index < scopes.row_count() {
            scopes.remove(index);
        }
        let instance_bridge = ui_handle.global::<InstanceBridge>();
        let instances_rc = instance_bridge.get_instances();
        let instances_rc = instances_rc
            .as_any()
            .downcast_ref::<VecModel<Instance>>()
            .unwrap();
        instances_rc.clear();
        for instance in instances.iter() {
            instances_rc.push(instance.clone());
        }
        sync_scoped_instance(ui_handle.as_weak());
    }) {
        Ok(_) => {
            debug!("Removed scope from UI");
        }
        Err(e) => {
            debug!("Failed to update UI: {e}");
        }
    }
}
