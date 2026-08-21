use std::time::Duration;

use axum::{
    Json,
    body::Body,
    extract::{FromRequest, Request, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tower_http::{
    cors::{AllowOrigin, Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{Span, debug, info, warn};

use super::{ServerState, UiEvent, launch_instance};
use crate::{launcher, models::{FeatureFlags, InstanceData, ScopeData}};

pub async fn serve(state: ServerState) {
    let listener = match tokio::net::TcpListener::bind(&format!("{}:{}", "127.0.0.1", 3307)).await {
        Ok(listener) => listener,
        Err(e) => {
            warn!("Failed to bind to port 3307: {e}");
            info!("Falling back to a random port...");
            tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("failed to bind port")
        }
    };

    let port = listener.local_addr().unwrap().port();

    launcher::write_lock_file(port);

    state.events.send(UiEvent::Online { port }).await.ok();

    info!(
        "API server is listening on [[ {} ]]",
        listener.local_addr().expect("failed to bind port")
    );
    axum::serve(listener, router(state))
        .await
        .expect("failed to launch server");
}

pub fn router(state: ServerState) -> axum::Router {
    let cors_state = state.clone();
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(AllowOrigin::async_predicate(
            |origin, _request_parts| async move {
                let scopes = cors_state.scopes.read().await;
                for scope in scopes.iter() {
                    if origin.to_str().unwrap_or("").ends_with(&scope.host) {
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
                        .post(launch)
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

impl From<&crate::daemon::ProxyInstance> for InstanceResponse {
    #[allow(deprecated)]
    fn from(instance: &crate::daemon::ProxyInstance) -> Self {
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
    let scope = origin(&headers);
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

async fn launch(
    State(state): State<ServerState>, headers: HeaderMap,
    axum::Json(instance_data): axum::Json<InstanceData>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let scope = origin(&headers).to_owned();

    let mut instance_data = instance_data;
    instance_data.scope_host = scope;

    match launch_instance(&state, &instance_data).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(err),
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
    let scope = origin(&headers).to_owned();

    let local = req.local.clone();
    let instances = state.instances.write().await;

    let Some(tunnel) = instances.iter().find(|i| i.local.as_str() == local) else {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Tunnel {} not found", local),
        ));
    };

    if tunnel.scope_host.as_str() != scope {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Tunnel {} not found in scope {}", local, scope),
        ));
    }

    drop(instances);
    super::remove_instance(&state, &local).await;

    Ok(StatusCode::OK)
}

async fn get_control_status(
    State(state): State<ServerState>, headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let req_scope = origin(&headers).to_owned();
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
    let req_scope = origin(&headers).to_owned();
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
    drop(scopes);

    debug!("Scope {} requested control", req_scope);
    state.events.send(UiEvent::Refresh).await.ok();

    Ok(StatusCode::OK)
}

async fn update_website_info(
    State(state): State<ServerState>, headers: HeaderMap,
    axum::Json(scope_data): axum::Json<ScopeData>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let req_scope = origin(&headers).to_owned();
    let mut scopes = state.scopes.write().await;
    if let Some(scope) = scopes.iter_mut().find(|s| s.host == req_scope) {
        scope.name = scope_data.name.clone();
        scope.features = scope_data.features;
        scope.settings = scope_data.settings.clone();
        drop(scopes);

        state.events.send(UiEvent::Refresh).await.ok();

        Ok(StatusCode::OK)
    } else {
        Err((
            StatusCode::FORBIDDEN,
            format!("Scope {req_scope} not found"),
        ))
    }
}

async fn popup_window(State(state): State<ServerState>) -> impl IntoResponse {
    state.events.send(UiEvent::Popup).await.ok();
    StatusCode::OK
}

fn origin(headers: &HeaderMap) -> &str {
    headers
        .get("Origin")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default()
}
