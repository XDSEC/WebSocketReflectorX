use slint::{ComponentHandle, Model, SharedString, VecModel};
use tracing::debug;

use crate::ui::{Instance, InstanceBridge, MainWindow, Scope, ScopeBridge, UiState};

pub fn setup(window: &MainWindow) {
    let bridge = window.global::<UiState>();
    let window_weak = window.as_weak();

    bridge.on_change_scope(move |scope: SharedString| {
        let window = window_weak.clone().upgrade().unwrap();
        let ui_state = window.global::<UiState>();
        ui_state.set_scope(Scope {
            host: "localhost".into(),
            name: "localhost".into(),
            state: "pending".into(),
            features: "".into(),
            settings: "{}".into(),
        });

        let window_weak = window_weak.clone();

        match slint::spawn_local(async move {
            let window = window_weak.clone().upgrade().unwrap();
            let ui_state = window.global::<UiState>();
            let scope_bridge = window.global::<ScopeBridge>();
            let scopes = scope_bridge.get_scopes();
            let scopes = scopes.as_any().downcast_ref::<VecModel<Scope>>().unwrap();
            let found_scope = scopes.iter().find(|s| s.host == scope);
            if let Some(found_scope) = found_scope {
                ui_state.set_scope(found_scope);
                debug!("Scope found: {scope}");
            } else {
                debug!("Scope not found: {scope}");
            }
            sync_scoped_instance(window_weak);
        }) {
            Ok(_) => {}
            Err(e) => {
                debug!("Failed to change scope: {e}");
            }
        }
    });
}

pub fn sync_scoped_instance(window: slint::Weak<MainWindow>) {
    let window = window.upgrade().unwrap();
    let ui_state = window.global::<UiState>();
    let instance_bridge = window.global::<InstanceBridge>();
    let instances = instance_bridge.get_instances();
    let instances = instances
        .as_any()
        .downcast_ref::<VecModel<Instance>>()
        .unwrap();
    let scoped_instances = instance_bridge.get_scoped_instances();
    let scoped_instances = scoped_instances
        .as_any()
        .downcast_ref::<VecModel<Instance>>()
        .unwrap();
    let current_scope = ui_state.get_page().to_string();
    scoped_instances.clear();
    for instance in instances.iter() {
        if instance.scope_host == current_scope {
            scoped_instances.push(instance.clone());
        }
    }
}
