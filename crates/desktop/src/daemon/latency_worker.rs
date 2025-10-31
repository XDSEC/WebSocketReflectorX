use reqwest::Method;
use slint::{ComponentHandle, Model, VecModel};
use thiserror::Error;
use tracing::{debug, warn};

use super::{
    model::{FeatureFlags, InstanceData, ServerState},
    ui_controller::on_instance_del,
};
use crate::{
    bridges::ui_state::sync_scoped_instance,
    ui::{Instance, InstanceBridge},
};

pub async fn start(state: ServerState) {
    loop {
        let instances = state.instances.read().await;
        let instances_pure = instances
            .iter()
            .map(|instance| instance.into())
            .collect::<Vec<InstanceData>>();
        drop(instances);
        let client = reqwest::Client::new();
        for instance in instances_pure {
            let instance = instance.clone();
            let client = client.clone();
            let state = state.clone();
            tokio::spawn(async move {
                let result = update_instance_latency(&instance, &client).await;
                if let Ok(elapsed) = result {
                    update_instance_state(state.clone(), &instance, elapsed).await;
                } else {
                    update_instance_state(state.clone(), &instance, -1).await;
                }
                if let Err(e) = result {
                    pingfall(state.clone(), instance.clone(), e).await;
                }
            });
        }
        sync_scoped_instance(state.ui.clone());

        // Sleep for 5 seconds
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

#[derive(Debug, Error)]
pub enum LatencyError {
    #[error("Request error: {0}")]
    Rewqest(#[from] reqwest::Error),
    #[error("Non-success status code")]
    NonSuccessStatus(u16),
}

pub async fn update_instance_latency(
    instance: &InstanceData, client: &reqwest::Client,
) -> Result<i32, LatencyError> {
    let req = client
        .request(Method::OPTIONS, instance.remote.replace("ws", "http"))
        .header("User-Agent", format!("wsrx/{}", env!("CARGO_PKG_VERSION")))
        .build()?;

    let start_time = std::time::Instant::now();

    let resp = client.execute(req).await?;

    let elapsed = if resp.status().is_success() {
        // always > 0
        start_time.elapsed().as_millis() as i32 / 2
    } else {
        debug!("Failed to ping instance: {}", resp.status());
        return Err(LatencyError::NonSuccessStatus(resp.status().as_u16()));
    };

    Ok(elapsed)
}

pub async fn update_instance_state(state: ServerState, instance: &InstanceData, elapsed: i32) {
    for proxy_instance in state.instances.write().await.iter_mut() {
        if instance.remote != proxy_instance.remote {
            continue;
        }

        proxy_instance.latency = elapsed;
        let window = state.ui.clone();
        let instance = instance.clone();

        let _ = slint::invoke_from_event_loop(move || {
            let window = window.upgrade().unwrap();
            let instance_bridge = window.global::<InstanceBridge>();
            let instances_rc = instance_bridge.get_instances();
            let instances_rc = instances_rc
                .as_any()
                .downcast_ref::<VecModel<Instance>>()
                .unwrap();

            if let Some(index) = instances_rc
                .iter()
                .position(|i| i.local == instance.local.as_str())
            {
                instances_rc.set_row_data(
                    index,
                    Instance {
                        local: instance.local.as_str().into(),
                        remote: instance.remote.as_str().into(),
                        latency: elapsed,
                        label: instance.label.as_str().into(),
                        scope_host: instance.scope_host.as_str().into(),
                    },
                );
            }
        });

        break;
    }
}

async fn pingfall(state: ServerState, instance: InstanceData, err: LatencyError) {
    warn!(
        "Pingfall triggered for instance {} due to error: {err:?}",
        instance.local
    );
    let scopes = state.scopes.read().await;

    let scope = scopes
        .iter()
        .find(|scope| scope.host == instance.scope_host.as_str());
    debug!("Pingfall settings: {:?}", scope);
    if let Some(scope) = scope
        && scope.features.contains(FeatureFlags::PingFall)
    {
        let settings = scope.settings.get("pingfall");
        if let Some(settings) = settings {
            let pingfall_settings: super::model::PingFallSettings =
                serde_json::from_value(settings.to_owned()).unwrap_or_default();

            match err {
                LatencyError::NonSuccessStatus(code) => {
                    if pingfall_settings.fail_status.contains(&code)
                        || pingfall_settings.fail_status.is_empty()
                    {
                        on_instance_del(&state, &instance.local).await;
                    }
                }
                LatencyError::Rewqest(_) => {
                    if pingfall_settings.drop_unknown {
                        on_instance_del(&state, &instance.local).await;
                    }
                }
            }
        }
    }
}
