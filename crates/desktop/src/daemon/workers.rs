use std::time::Duration;

use reqwest::Method;
use thiserror::Error;
use tokio::io::AsyncBufReadExt;
use tracing::{debug, error, warn};

use super::{ServerState, UiEvent, remove_instance};
use crate::{launcher, models::{FeatureFlags, InstanceData, LogEntry, PingFallSettings}};

/// Periodically pings every instance and updates its latency in the UI.
pub async fn latency_loop(state: ServerState) {
    let client = reqwest::Client::new();
    loop {
        let instances = state.instances.read().await;
        let instances_pure = instances
            .iter()
            .map(|instance| instance.into())
            .collect::<Vec<InstanceData>>();
        drop(instances);

        let mut changed = false;
        for instance in instances_pure {
            let instance = instance.clone();
            let client = client.clone();
            let state = state.clone();
            let result = update_instance_latency(&instance, &client).await;
            let elapsed = match result {
                Ok(elapsed) => elapsed,
                Err(_) => -1,
            };
            if update_instance_state(&state, &instance, elapsed).await {
                changed = true;
            }
            if let Err(e) = result {
                pingfall(state.clone(), instance.clone(), e).await;
            }
        }

        if changed {
            state.events.send(UiEvent::Refresh).await.ok();
        }

        // Sleep for 5 seconds
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

#[derive(Debug, Error)]
pub enum LatencyError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Non-success status code")]
    NonSuccessStatus(u16),
}

pub async fn update_instance_latency(
    instance: &InstanceData, client: &reqwest::Client,
) -> Result<i32, LatencyError> {
    let req = client
        .request(Method::OPTIONS, instance.remote.replace("ws", "http"))
        .header("User-Agent", format!("wsrx/{}", env!("CARGO_PKG_VERSION")))
        .build()?;

    let start_time = std::time::Instant::now();

    let resp = client.execute(req).await?;

    let elapsed = if resp.status().is_success() {
        // always > 0
        start_time.elapsed().as_millis() as i32 / 2
    } else {
        debug!("Failed to ping instance: {}", resp.status());
        return Err(LatencyError::NonSuccessStatus(resp.status().as_u16()));
    };

    Ok(elapsed)
}

/// Updates the latency of the instance with a matching remote address.
/// Returns `true` when the value changed.
pub async fn update_instance_state(
    state: &ServerState, instance: &InstanceData, elapsed: i32,
) -> bool {
    let mut changed = false;
    for proxy_instance in state.instances.write().await.iter_mut() {
        if instance.remote != proxy_instance.remote {
            continue;
        }

        if proxy_instance.latency != elapsed {
            proxy_instance.latency = elapsed;
            changed = true;
        }
        break;
    }
    changed
}

async fn pingfall(state: ServerState, instance: InstanceData, err: LatencyError) {
    warn!(
        "Pingfall triggered for instance {} due to error: {err:?}",
        instance.local
    );
    let scopes = state.scopes.read().await;

    let scope = scopes
        .iter()
        .find(|scope| scope.host == instance.scope_host.as_str());
    debug!("Pingfall settings: {:?}", scope);
    if let Some(scope) = scope
        && scope.features.contains(FeatureFlags::PingFall)
    {
        let settings = scope.settings.get("pingfall");
        if let Some(settings) = settings {
            let pingfall_settings: PingFallSettings =
                serde_json::from_value(settings.to_owned()).unwrap_or_default();

            match err {
                LatencyError::NonSuccessStatus(code) => {
                    if pingfall_settings.status.contains(&code)
                        || pingfall_settings.status.is_empty()
                    {
                        remove_instance(&state, &instance.local).await;
                    }
                }
                LatencyError::Request(_) => {
                    if pingfall_settings.drop_unknown {
                        remove_instance(&state, &instance.local).await;
                    }
                }
            }
        }
    }
}

/// Streams JSON log lines from `logs/wsrx.log` into the UI.
pub async fn stream_logs(state: ServerState) {
    let proj_dirs = launcher::project_dirs();
    let log_file = proj_dirs.data_local_dir().join("logs").join("wsrx.log");

    // Wait until the log file exists (the logger creates it lazily).
    let mut lines = loop {
        match tokio::fs::File::open(&log_file)
            .await
            .map(tokio::io::BufReader::new)
            .map(tokio::io::BufReader::lines)
        {
            Ok(lines) => break lines,
            Err(_) => tokio::time::sleep(Duration::from_secs(1)).await,
        }
    };

    let mut timer = tokio::time::interval(Duration::from_secs(1));
    let read_timeout = Duration::from_secs(5);
    loop {
        timer.tick().await;
        while let Ok(log) = tokio::time::timeout(read_timeout, lines.next_line()).await {
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
            state.events.send(UiEvent::Log(log_entry)).await.ok();
        }
    }
}

/// Checks the GitHub releases API for a newer version and notifies the UI.
pub async fn check_for_updates(state: ServerState) {
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
            state.events.send(UiEvent::HasUpdates(true)).await.ok();
            warn!("Update available: {}", version);
        } else {
            state.events.send(UiEvent::HasUpdates(false)).await.ok();
            debug!("No update available.");
        }
    } else {
        error!(
            "Failed to fetch the latest version: {} {:?}",
            response.status(),
            response.text().await
        );
    }
}

/// Returns a human-readable system info string for the settings page.
pub fn system_info() -> String {
    format!(
        "System    : {}\nLocale    : {}\nCPU       : {}\nKernel    : {}\nWSRX      : {}",
        sysinfo::System::name().unwrap_or_else(|| "Unknown".into()),
        sys_locale::get_locale().unwrap_or_else(|| "Unknown".into()),
        sysinfo::System::cpu_arch(),
        sysinfo::System::kernel_long_version(),
        crate::WSRX_FULL_VERSION,
    )
}
