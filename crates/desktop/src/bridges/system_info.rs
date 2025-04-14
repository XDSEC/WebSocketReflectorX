use async_compat::Compat;
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use serde::Deserialize;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use std::rc::Rc;
use sysinfo::System;
use tokio::{fs, io::AsyncBufReadExt, time::timeout};
use tracing::{debug, error, warn};

use crate::{
    WSRX_FULL_VERSION,
    ui::{Log, MainWindow, SystemInfoBridge},
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

    let logs_model: Rc<VecModel<Log>> = Rc::new(VecModel::default());
    let logs = ModelRc::from(logs_model.clone());
    bridge.set_logs(logs);

    bridge.on_refresh_interfaces(move || {
        let model = network_interfaces_model.clone();
        refresh_network_interfaces(model);
    });

    bridge.on_open_link(move |url| {
        open::that_detached(&url).unwrap_or_else(|_| {
            tracing::error!("Failed to open link {url} in default browser.");
        });
    });

    bridge.on_open_logs(move || {
        let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
            Some(dirs) => dirs,
            None => {
                error!("Unable to find project config directories");
                return;
            }
        };
        let log_dir = proj_dirs.data_local_dir().join("logs");
        open::that_detached(&log_dir).unwrap_or_else(|_| {
            tracing::error!("Failed to open logs directory.");
        });
    });

    check_for_updates(window);
    stream_logs(&window.as_weak());
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
            .user_agent(format!("wsrx/{}", env!("CARGO_PKG_VERSION")))
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
                warn!("Update available: {}", version);
            } else {
                bridge.set_has_updates(false);
                debug!("No update available.");
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

#[derive(Clone, Debug, Deserialize, Default)]
struct LogEntryFields {
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Default)]
struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub target: String,
    pub fields: LogEntryFields,
}

fn stream_logs(window: &slint::Weak<MainWindow>) {
    let window = window.clone();

    slint::spawn_local(Compat::new(async move {
        let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
            Some(dirs) => dirs,
            None => {
                eprintln!("Unable to find project config directories");
                return;
            }
        };
        let log_file = proj_dirs.data_local_dir().join("logs").join("wsrx.log");
        let mut timer = tokio::time::interval(tokio::time::Duration::from_secs(1));
        let mut lines = fs::File::open(&log_file)
            .await
            .map(tokio::io::BufReader::new)
            .map(tokio::io::BufReader::lines)
            .unwrap();
        let interval = tokio::time::Duration::from_secs(5);
        loop {
            timer.tick().await;
            while let Ok(log) = timeout(interval, lines.next_line()).await {
                let log = match log {
                    Ok(Some(log)) => log,
                    Ok(None) => break,
                    Err(e) => {
                        error!("failed to read log: {:?}", e);
                        break;
                    }
                };
                let log_entry = serde_json::from_str::<LogEntry>(&log).unwrap_or_else(|_| {
                    error!("failed to parse log: {}", log);
                    LogEntry::default()
                });
                let window = window.clone();

                slint::invoke_from_event_loop(move || {
                    let window = window.upgrade().unwrap();
                    let system_info_bridge = window.global::<SystemInfoBridge>();
                    let local_time = log_entry.timestamp.with_timezone(&chrono::Local);
                    let log = Log {
                        timestamp: format!("{}", local_time.format("%Y-%m-%d %H:%M:%S")).into(),
                        level: log_entry.level.into(),
                        target: log_entry.target.into(),
                        message: log_entry.fields.message.into(),
                    };
                    let logs = system_info_bridge.get_logs();
                    let logs = logs.as_any().downcast_ref::<VecModel<Log>>().unwrap();
                    logs.push(log.clone());
                })
                .ok();
            }
        }
    }))
    .ok();
}
