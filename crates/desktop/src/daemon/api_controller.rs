use std::time::Duration;

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
use slint::{ComponentHandle, Model, ToSharedString, VecModel};
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{Span, debug};
use wsrx::utils::create_tcp_listener;

use super::latency_worker::update_instance_latency;
use crate::{
    bridges::ui_state::sync_scoped_instance,
    daemon::{
        latency_worker::update_instance_state,
        model::{FeatureFlags, InstanceData, ProxyInstance, ScopeData, ServerState},
    },
    ui::{Instance, InstanceBridge, Scope, ScopeBridge},
};

pub fn router(state: ServerState) -> axum::Router {
    let cors_state = state.clone();
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
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
                        .patch(update_website_info),
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
    #[deprecated]
    from: String,
    #[deprecated]
    to: String,
    latency: i32,
}

impl From<&ProxyInstance> for InstanceResponse {
    #[allow(deprecated)]
    fn from(instance: &ProxyInstance) -> Self {
        InstanceResponse {
            label: instance.label.clone(),
            remote: instance.remote.clone(),
            local: instance.local.clone(),
            from: instance.local.clone(),
            to: instance.remote.clone(),
            latency: instance.latency,
        }
    }
}

async fn get_instances(
    State(state): State<ServerState>, headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let scope = headers
        .get("Origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();
    let instances = state.instances.read().await;
    let instances: Vec<InstanceResponse> = instances
        .iter()
        .filter_map(|instance| {
            if instance.scope_host.as_str() == scope {
                Some(instance.into())
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

    let listener = create_tcp_listener(&instance_data.local).await?;

    let local = listener
        .local_addr()
        .expect("failed to bind port")
        .to_string();

    let mut instances = state.instances.write().await;
    if instances.iter().any(|i| i.local.as_str() == local) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("The local address {local} is already taken by another instance"),
        ));
    }

    if let Some(instance) = instances
        .iter_mut()
        .find(|i| i.remote == instance_data.remote && i.scope_host == scope)
    {
        // test instance config changes
        if instance.label != instance_data.label {
            instance.label = instance_data.label.clone();
        }

        return Ok(Json(instance.data.clone()));
    }

    let instance = ProxyInstance::new(
        instance_data.label.clone(),
        scope.clone(),
        listener,
        instance_data.remote.clone(),
    );

    let instance_resp: InstanceData = (&instance).into();
    instances.push(instance);
    drop(instances);

    let state_clone = state.clone();
    let instance = instance_resp.clone();

    tokio::spawn(async move {
        let client = reqwest::Client::new();
        match update_instance_latency(&instance, &client).await {
            Ok(elapsed) => update_instance_state(state_clone, &instance, elapsed).await,
            Err(_) => update_instance_state(state_clone, &instance, -1).await,
        };
    });

    match slint::invoke_from_event_loop(move || {
        let ui_handle = state.ui.upgrade().unwrap();
        let instance_bridge = ui_handle.global::<InstanceBridge>();
        let instances = instance_bridge.get_instances();
        let instances = instances
            .as_any()
            .downcast_ref::<VecModel<Instance>>()
            .unwrap();
        let instance = Instance {
            label: instance_data.label.as_str().into(),
            remote: instance_data.remote.as_str().into(),
            local: local.as_str().into(),
            latency: -1,
            scope_host: scope.as_str().into(),
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

    let Some(tunnel) = instances.iter().find(|i| i.local.as_str() == req.local) else {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Tunnel {} not found", req.local),
        ));
    };

    if tunnel.scope_host.as_str() != scope {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Tunnel {} not found in scope {}", req.local, scope),
        ));
    }

    instances.retain(|i| i.local.as_str() != req.local);

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
            format!("Scope {req_scope} not found"),
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
    let (scope_name, scope_features, scope_settings) = if let Some(Json(ScopeData {
        name,
        features,
        settings,
        ..
    })) = json_body
    {
        (name, features, settings)
    } else {
        (req_scope.clone(), FeatureFlags::Basic, Default::default())
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
        features: scope_features,
        settings: scope_settings.clone(),
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
            features: scope_features.to_shared_string(),
            settings: serde_json::to_string(&scope_settings)
                .unwrap_or("{}".to_string())
                .into(),
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
        scope.features = scope_data.features;
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
                features: scope_data.features.to_shared_string(),
                settings: serde_json::to_string(&scope_data.settings)
                    .unwrap_or("{}".to_string())
                    .into(),
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
            format!("Scope {req_scope} not found"),
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
