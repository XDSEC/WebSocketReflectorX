use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::rc::Rc;

use crate::ui::{MainWindow, SystemInfoBridge};
use local_ip_address::list_afinet_netifas;

pub fn setup(window: &MainWindow) {
    let bridge = window.global::<SystemInfoBridge>();
    #[cfg(target_os = "linux")]
    bridge.set_os("linux".into());
    #[cfg(target_os = "windows")]
    bridge.set_os("windows".into());
    #[cfg(target_os = "macos")]
    bridge.set_os("macos".into());

    let network_interfaces_model: Rc<VecModel<SharedString>> =
        Rc::new(VecModel::from(vec!["127.0.0.1".into(), "0.0.0.0".into()]));
    let network_interfaces = ModelRc::from(network_interfaces_model.clone());
    bridge.set_interfaces(network_interfaces);

    bridge.on_refresh_interfaces(move || {
        let model = network_interfaces_model.clone();
        refresh_network_interfaces(model);
    });
}

pub fn refresh_network_interfaces(model: Rc<VecModel<SharedString>>) {
    let interfaces = list_afinet_netifas().unwrap_or_default();
    model.clear();
    for (_, addr) in interfaces {
        if addr.is_ipv6() {
            continue;
        }
        let ip = addr.to_string();
        model.push(SharedString::from(ip));
    }
    model.push("0.0.0.0".into());
}
