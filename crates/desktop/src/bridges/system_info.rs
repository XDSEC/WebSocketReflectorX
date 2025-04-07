use async_compat::Compat;
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::rc::Rc;
use sysinfo::System;
use tracing::{debug, error, info};

use crate::{
    WSRX_FULL_VERSION,
    ui::{MainWindow, SystemInfoBridge},
};
use local_ip_address::list_afinet_netifas;

pub fn setup(window: &MainWindow) {
    let bridge = window.global::<SystemInfoBridge>();
    #[cfg(target_os = "linux")]
    bridge.set_os("linux".into());
    #[cfg(target_os = "windows")]
    bridge.set_os("windows".into());
    #[cfg(target_os = "macos")]
    bridge.set_os("macos".into());

    bridge.set_info(
        format!(
            "System    : {}\nCPU       : {}\nKernel    : {}\nWSRX      : {}",
            System::name().unwrap_or_else(|| "Unknown".into()),
            System::cpu_arch(),
            System::kernel_long_version(),
            WSRX_FULL_VERSION,
        )
        .into(),
    );

    bridge.set_version(env!("CARGO_PKG_VERSION").into());

    let network_interfaces_model: Rc<VecModel<SharedString>> =
        Rc::new(VecModel::from(vec!["127.0.0.1".into(), "0.0.0.0".into()]));
    let network_interfaces = ModelRc::from(network_interfaces_model.clone());
    bridge.set_interfaces(network_interfaces);

    bridge.on_refresh_interfaces(move || {
        let model = network_interfaces_model.clone();
        refresh_network_interfaces(model);
    });

    bridge.on_goto_support(move || {
        open::that_detached("https://github.com/XDSEC/WebSocketReflectorX/issues").unwrap_or_else(
            |_| {
                tracing::error!("Failed to open the support page.");
            },
        );
    });

    check_for_updates(window);
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

fn check_for_updates(window: &MainWindow) {
    let window_weak = window.as_weak();
    let _ = slint::spawn_local(Compat::new(async move {
        let window = window_weak.unwrap();
        let bridge = window.global::<SystemInfoBridge>();
        debug!("Checking for updates...");
        let client = reqwest::Client::builder()
            .user_agent("WebSocketReflectorX/0.4")
            .build()
            .unwrap();
        let response = match client
            .get("https://api.github.com/repos/XDSEC/WebSocketReflectorX/releases/latest")
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                error!("Failed to fetch the latest version: {}", e);
                return;
            }
        };

        if response.status().is_success() {
            let json: serde_json::Value = match response.json().await {
                Ok(json) => json,
                Err(e) => {
                    error!("Failed to parse the response: {}", e);
                    return;
                }
            };
            let version = json["tag_name"].as_str().unwrap_or("0.0.0");
            let current_version = env!("CARGO_PKG_VERSION");
            if version != current_version {
                bridge.set_has_updates(true);
                info!("Update available: {}", version);
            } else {
                bridge.set_has_updates(false);
                info!("No update available.");
            }
        } else {
            error!(
                "Failed to fetch the latest version: {} {:?}",
                response.status(),
                response.text().await
            );
        }
    }));
}
