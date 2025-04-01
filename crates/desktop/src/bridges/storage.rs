use slint::ComponentHandle;

use crate::ui::{InstanceBridge, MainWindow, ScopeBridge};

pub fn setup(window: &MainWindow) {
    let _instance_bridge = window.global::<InstanceBridge>();
    let _scope_bridge = window.global::<ScopeBridge>();
}
