use std::net::ToSocketAddrs;

use slint::{ComponentHandle, Model, VecModel};
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};

use crate::{
    bridges::ui_state::sync_scoped_instance,
    daemon::model::InstanceData,
    ui::{Instance, InstanceBridge, MainWindow, Scope, ScopeBridge},
};

use super::model::ServerState;

pub async fn on_instance_add(state: &ServerState, remote: &str, local: &str) {
    let mut tcp_addr_obj = match local.to_string().to_socket_addrs() {
        Ok(tcp_addr_obj) => tcp_addr_obj,
        Err(err) => {
            error!("Failed to parse from address: {err}");
            return;
        }
    };
    let tcp_addr_obj = match tcp_addr_obj.next() {
        Some(tcp_addr_obj) => tcp_addr_obj,
        None => {
            error!("Failed to parse from address");
            return;
        }
    };
    let listener = match TcpListener::bind(tcp_addr_obj).await {
        Ok(listener) => listener,
        Err(err) => {
            error!("Failed to bind to address: {err}");
            return;
        }
    };
    let local = listener
        .local_addr()
        .expect("failed to bind port")
        .to_string();
    info!("CREATE tcp server: {local} <- wsrx -> {remote}",);

    let mut instances = state.instances.write().await;
    if instances.iter().any(|i| i.local == local) {
        warn!("Instance already exists: {local}");
        return;
    }
    let remote = remote.to_string();
    let remote_clone = remote.clone();

    let instance = InstanceData {
        label: format!("inst-{:06x}", rand::random::<u32>()),
        remote: remote.clone(),
        local: local.clone(),
        latency: -1,
        scope_host: "default-scope".to_string(),
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
    let label = instance.label.clone();
    instances.push(instance);
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
            label: label.into(),
            remote: remote_clone.into(),
            local: local.into(),
            latency: -1,
            scope_host: "default-scope".into(),
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
    let mut instances = state.instances.write().await;

    if let Some(tunnel) = instances.iter().find(|v| v.local == local) {
        info!("CLOSE tcp server: {} <- wsrx -> {}", local, tunnel.remote);
        if let Some(handle) = tunnel.handle.as_ref() {
            handle.abort();
        }

        instances.retain(|i| i.local != local);
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
}

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
