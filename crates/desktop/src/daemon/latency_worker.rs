use reqwest::Method;
use slint::{ComponentHandle, Model, VecModel};
use tracing::{debug, error};

use super::{
    model::{InstanceDataPure, ServerState},
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
            .collect::<Vec<InstanceDataPure>>();
        drop(instances);
        let client = reqwest::Client::new();
        for instance in instances_pure {
            let instance = instance.clone();
            let client = client.clone();
            let state = state.clone();
            tokio::spawn(async move {
                if let Err(e) = update_instance_latency(state.clone(), instance, &client).await {
                    error!("Failed to update latency: {:?}", e);
                }
                pingfall(state).await;
            });
        }
        sync_scoped_instance(state.ui.clone());
        // Sleep for 5 seconds
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn update_instance_latency(
    state: ServerState, instance: InstanceDataPure, client: &reqwest::Client,
) -> Result<i32, reqwest::Error> {
    let start_time = std::time::Instant::now();
    let resp = client
        .request(Method::OPTIONS, instance.remote.replace("ws", "http"))
        .header("User-Agent", format!("wsrx/{}", env!("CARGO_PKG_VERSION")))
        .send()
        .await?;
    let elapsed = start_time.elapsed();
    let mut instances = state.instances.write().await;
    for instance_d in instances.iter_mut() {
        if instance.remote == instance_d.remote {
            let mut latency = elapsed.as_millis() as i32 / 2;
            if resp.status().is_success() {
                instance_d.latency = latency;
            } else {
                debug!(
                    "Failed to get latency for link {}: {:?}",
                    instance_d.remote,
                    resp.status()
                );
                instance_d.latency = -1;
                latency = -1;
            }
            let window = state.ui.clone();
            slint::invoke_from_event_loop(move || {
                let window = window.upgrade().unwrap();
                let instance_bridge = window.global::<InstanceBridge>();
                let instances_rc = instance_bridge.get_instances();
                let instances_rc = instances_rc
                    .as_any()
                    .downcast_ref::<VecModel<Instance>>()
                    .unwrap();
                let mut index = 0;
                for i in instances_rc.iter() {
                    if i.local == instance.local {
                        break;
                    }
                    index += 1;
                }
                instances_rc.set_row_data(
                    index,
                    Instance {
                        local: instance.local.clone().into(),
                        remote: instance.remote.clone().into(),
                        latency,
                        label: instance.label.clone().into(),
                        scope_host: instance.scope_host.clone().into(),
                    },
                );
            })
            .ok();
            break;
        }
    }

    Ok(0)
}

async fn pingfall(state: ServerState) {
    let instances = state.instances.read().await;
    let instances_pure = instances
        .iter()
        .map(|instance| instance.into())
        .collect::<Vec<InstanceDataPure>>();
    drop(instances);
    let scopes = state.scopes.read().await;
    for instance in instances_pure.iter() {
        let scope = scopes
            .iter()
            .find(|scope| scope.name == instance.scope_host);
        if let Some(scope) = scope {
            if scope.features.contains(&"pingfall".to_owned()) {
                on_instance_del(&state, &instance.local).await;
            }
        }
    }
}
