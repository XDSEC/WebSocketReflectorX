use std::sync::Arc;

use slint::{ComponentHandle, Model, ToSharedString, VecModel};
use tracing::{debug, info, warn};
use wsrx::utils::create_tcp_listener;

use super::{default_label, model::ServerState};
use crate::{
    bridges::ui_state::sync_scoped_instance,
    daemon::proxy_instance::ProxyInstance,
    ui::{Instance, InstanceBridge, MainWindow, Scope, ScopeBridge},
};

pub async fn on_instance_add(state: &ServerState, remote: &str, local: &str) {
    let listener = match create_tcp_listener(local).await {
        Ok(listener) => listener,
        Err(_) => return,
    };

    let local = listener
        .local_addr()
        .expect("failed to bind port")
        .to_string();

    if state
        .instances
        .read()
        .await
        .iter()
        .any(|i| i.local.as_str() == local)
    {
        warn!("Instance already exists: {local}");
        return;
    }

    let remote = Arc::new(remote.to_string());
    let scope = Arc::new("default-scope".to_string());

    let instance = ProxyInstance::new(default_label(), scope.clone(), listener, remote.clone());

    let label = instance.label.clone();
    state.instances.write().await.push(instance);

    let state = state.clone();

    match slint::invoke_from_event_loop(move || {
        let ui_handle = state.ui.upgrade().unwrap();
        let instance_bridge = ui_handle.global::<InstanceBridge>();
        let instances = instance_bridge.get_instances();
        let instances = instances
            .as_any()
            .downcast_ref::<VecModel<Instance>>()
            .unwrap();
        let instance = Instance {
            label: label.as_str().into(),
            remote: remote.as_str().into(),
            local: local.as_str().into(),
            latency: -1,
            scope_host: scope.as_str().into(),
        };
        instances.push(instance);
        sync_scoped_instance(ui_handle.as_weak());
    }) {
        Ok(_) => {
            debug!("Added instance to UI");
        }
        Err(e) => {
            debug!("Failed to update UI: {e}");
        }
    }
}

pub async fn on_instance_del(state: &ServerState, local: &str) {
    state
        .instances
        .write()
        .await
        .retain(|instance| instance.local.as_str() != local);

    let state = state.clone();
    let local = local.to_string();

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
            if i.local == local {
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
        }
    }
}

pub async fn on_scope_allow(state: &ServerState, ui: slint::Weak<MainWindow>, scope_host: &str) {
    let mut scopes = state.scopes.write().await;
    let scope_name;
    let scope_features;

    if let Some(scope) = scopes.iter_mut().find(|s| s.host == scope_host) {
        scope.state = "allowed".to_string();
        scope_name = scope.name.clone();
        scope_features = scope.features;
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
                    features: scope_features.to_shared_string(),
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
    let removed_scope = {
        let mut scopes = state.scopes.write().await;
        scopes
            .iter()
            .position(|s| s.host == scope_host)
            .map(|index| scopes.remove(index))
    };

    match removed_scope {
        Some(scope) => {
            state
                .instances
                .write()
                .await
                .retain(|i| i.scope_host.as_str() != scope.host);

            info!("Scope {} removed", scope.host);
        }
        None => return,
    };

    let scope_host = scope_host.to_string();
    let state = state.clone();

    let instances: Vec<Instance> = state
        .instances
        .read()
        .await
        .iter()
        .filter(|i| i.scope_host.as_str() == scope_host)
        .map(Into::into)
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
