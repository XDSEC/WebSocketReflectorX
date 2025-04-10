use slint::{ComponentHandle, Model, VecModel};
use tracing::{debug, info};

use crate::{
    bridges::ui_state::sync_scoped_instance,
    ui::{Instance, InstanceBridge, MainWindow, Scope, ScopeBridge},
};

use super::model::ServerState;

pub async fn on_scope_allow(state: &ServerState, ui: slint::Weak<MainWindow>, scope_host: &str) {
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

pub async fn on_scope_del(state: &ServerState, ui: slint::Weak<MainWindow>, scope_host: &str) {
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
            latency: i.latency,
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
