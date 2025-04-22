use reqwest::Method;
use slint::{ComponentHandle, Model, VecModel};
use tracing::debug;

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
                if update_instance_latency(state.clone(), instance.clone(), &client)
                    .await
                    .is_none()
                {
                    pingfall(state, instance).await;
                }
            });
        }
        sync_scoped_instance(state.ui.clone());

        // Sleep for 5 seconds
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn update_instance_latency(
    state: ServerState, instance: InstanceData, client: &reqwest::Client,
) -> Option<i32> {
    let req = client
        .request(Method::OPTIONS, instance.remote.replace("ws", "http"))
        .header("User-Agent", format!("wsrx/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .ok()?;

    let start_time = std::time::Instant::now();

    let resp = client.execute(req).await.ok()?;

    let elapsed = if resp.status().is_success() {
        // always > 0
        start_time.elapsed().as_millis() as i32 / 2
    } else {
        debug!("Failed to ping instance: {}", resp.status());
        return None;
    };

    for proxy_instance in state.instances.write().await.iter_mut() {
        if instance.remote != proxy_instance.remote {
            continue;
        }

        proxy_instance.latency = elapsed;
        let window = state.ui.clone();

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

    Some(elapsed)
}

async fn pingfall(state: ServerState, instance: InstanceData) {
    let scopes = state.scopes.read().await;

    let scope = scopes
        .iter()
        .find(|scope| scope.host == instance.scope_host.as_str());

    if let Some(scope) = scope {
        if scope.features.contains(FeatureFlags::PingFall) {
            on_instance_del(&state, &instance.local).await;
        }
    }
}
