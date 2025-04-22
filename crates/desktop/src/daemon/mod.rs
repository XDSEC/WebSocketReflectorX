use std::{process, rc::Rc, sync::Arc};

use api_controller::router;
use directories::ProjectDirs;
use model::{ScopeData, ServerState};
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Model, ToSharedString, VecModel};
use tokio::{net::TcpListener, sync::RwLock};
use tracing::{debug, error, info, warn};

use crate::ui::{Instance, InstanceBridge, MainWindow, Scope, ScopeBridge, SettingsBridge};

mod api_controller;
mod latency_worker;
mod model;
mod proxy_instance;
mod ui_controller;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ScopesConfig {
    scopes: Vec<ScopeData>,
}

pub fn setup(ui: &MainWindow) {
    use rustls::crypto;

    match crypto::aws_lc_rs::default_provider().install_default() {
        Ok(_) => info!("Using `AWS Libcrypto` as default crypto backend."),
        Err(err) => {
            error!("`AWS Libcrypto` is not available: {:?}", err);
            warn!("Try to use `ring` as default crypto backend.");
            crypto::ring::default_provider()
                .install_default()
                .inspect_err(|err| {
                    error!("`ring` is not available: {:?}", err);
                    error!("All crypto backend are not available, exiting...");
                    process::exit(1);
                })
                .ok();
            info!("Using `ring` as default crypto backend.");
        }
    }

    // read config scope
    let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
        Some(dirs) => dirs,
        None => {
            error!("Unable to find project config directories");
            return;
        }
    };
    let config_file = proj_dirs.config_dir().join("scopes.toml");
    let config = match std::fs::read_to_string(&config_file) {
        Ok(config) => config,
        Err(_) => "".to_owned(),
    };
    let scopes: ScopesConfig = match toml::from_str(&config) {
        Ok(scopes) => scopes,
        Err(e) => {
            error!("Failed to parse config file: {}", e);
            ScopesConfig { scopes: vec![] }
        }
    };
    debug!("Loaded scopes: {:?}", scopes);

    let handle = ui.as_weak();

    let state_d = ServerState {
        ui: handle.clone(),
        instances: Arc::new(RwLock::new(vec![])),
        scopes: Arc::new(RwLock::new(scopes.scopes.clone())),
    };
    // Initialize the global state
    let instances: Rc<VecModel<Instance>> = Rc::new(VecModel::default());
    let scopes_r: Rc<VecModel<Scope>> = Rc::new(VecModel::default());
    for scope in scopes.scopes.iter() {
        scopes_r.push(Scope {
            host: scope.host.clone().into(),
            name: scope.name.clone().into(),
            state: scope.state.clone().into(),
            features: scope.features.to_shared_string(),
        });
    }
    let scoped_instances: Rc<VecModel<Instance>> = Rc::new(VecModel::default());

    let instances_rc = slint::ModelRc::from(instances.clone());
    let scopes_rc = slint::ModelRc::from(scopes_r.clone());
    let scoped_instances_rc = slint::ModelRc::from(scoped_instances.clone());

    let instance_bridge = ui.global::<InstanceBridge>();
    instance_bridge.set_instances(instances_rc);
    instance_bridge.set_scoped_instances(scoped_instances_rc);

    let state = state_d.clone();

    instance_bridge.on_add(move |remote, local| {
        let state_cloned = state.clone();
        match slint::spawn_local(async_compat::Compat::new(async move {
            ui_controller::on_instance_add(&state_cloned, remote.as_str(), local.as_str()).await;
        })) {
            Ok(_) => {}
            Err(e) => {
                debug!("Failed to update instance bridge: {e}");
            }
        }
    });

    let state = state_d.clone();

    instance_bridge.on_del(move |local| {
        let state_cloned = state.clone();
        match slint::spawn_local(async_compat::Compat::new(async move {
            ui_controller::on_instance_del(&state_cloned, local.as_str()).await;
        })) {
            Ok(_) => {}
            Err(e) => {
                debug!("Failed to update instance bridge: {e}");
            }
        }
    });

    let scope_bridge = ui.global::<ScopeBridge>();
    scope_bridge.set_scopes(scopes_rc);

    let handle_cloned = handle.clone();
    let state = state_d.clone();

    scope_bridge.on_allow(move |scope_host| {
        let state_cloned = state.clone();
        let handle_cloned = handle_cloned.clone();
        match slint::spawn_local(async_compat::Compat::new(async move {
            ui_controller::on_scope_allow(
                &state_cloned,
                handle_cloned.clone(),
                scope_host.as_str(),
            )
            .await;
        })) {
            Ok(_) => {}
            Err(e) => {
                debug!("Failed to update scope bridge: {e}");
            }
        }
    });

    let state_cloned = state_d.clone();
    let handle_cloned = handle.clone();

    scope_bridge.on_del(move |scope_host| {
        let state_cloned = state_cloned.clone();
        let handle_cloned = handle_cloned.clone();
        match slint::spawn_local(async_compat::Compat::new(async move {
            ui_controller::on_scope_del(&state_cloned, handle_cloned.clone(), scope_host.as_str())
                .await;
        })) {
            Ok(_) => {}
            Err(e) => {
                debug!("Failed to update scope bridge: {e}");
            }
        }
    });

    let router = router(state_d.clone());

    match slint::spawn_local(async_compat::Compat::new(async move {
        let listener = match TcpListener::bind(&format!("{}:{}", "127.0.0.1", 3307)).await {
            Ok(listener) => listener,
            Err(e) => {
                warn!("Failed to bind to port 3307: {e}");
                // Fallback to a random port
                info!("Falling back to a random port...");
                // Bind to a random port
                TcpListener::bind("127.0.0.1:0")
                    .await
                    .expect("failed to bind port")
            }
        };

        let port = listener.local_addr().unwrap().port();

        slint::invoke_from_event_loop(move || {
            let ui = handle.upgrade().unwrap();
            let settings_bridge = ui.global::<SettingsBridge>();
            settings_bridge.set_api_port(port as i32);
            settings_bridge.set_online(true);
        })
        .ok();

        let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
            Some(dirs) => dirs,
            None => {
                error!("Unable to find project config directories");
                return;
            }
        };
        let lock_file = proj_dirs.data_local_dir().join(".rx.is.alive");
        tokio::fs::write(&lock_file, port.to_string())
            .await
            .unwrap_or_else(|_| {
                error!("Failed to write lock file");
                std::process::exit(1);
            });

        info!(
            "API server is listening on [[ {} ]]",
            listener.local_addr().expect("failed to bind port")
        );
        axum::serve(listener, router)
            .await
            .expect("failed to launch server");
    })) {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to start API server: {e}");
        }
    }

    let state = state_d.clone();
    match slint::spawn_local(async_compat::Compat::new(async move {
        latency_worker::start(state).await;
    })) {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to start latency worker: {e}");
        }
    }
}

pub fn save_scopes(ui: &slint::Weak<MainWindow>) {
    let window = ui.upgrade().unwrap();
    let scope_bridge = window.global::<ScopeBridge>();
    let scopes = scope_bridge.get_scopes();
    let scopes = scopes.as_any().downcast_ref::<VecModel<Scope>>().unwrap();
    let mut scopes_vec = vec![];
    for scope in scopes.iter() {
        scopes_vec.push(ScopeData {
            host: scope.host.to_string(),
            name: scope.name.to_string(),
            state: scope.state.to_string(),
            features: scope
                .features
                .split(",")
                .map(|s| s.trim().to_string())
                .into(),
        });
    }
    let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
        Some(dirs) => dirs,
        None => {
            error!("Unable to find project config directories");
            return;
        }
    };
    let config_file = proj_dirs.config_dir().join("scopes.toml");
    let config_obj = ScopesConfig { scopes: scopes_vec };
    let config = toml::to_string(&config_obj).unwrap_or_else(|e| {
        error!("Failed to serialize scopes: {}", e);
        String::new()
    });
    if let Err(e) = std::fs::create_dir_all(proj_dirs.config_dir()) {
        error!("Failed to create config directory: {}", e);
        return;
    }
    if let Err(e) = std::fs::write(&config_file, config) {
        error!("Failed to write config file: {}", e);
    }
    debug!("Saved scopes to: {:?}", config_file);
}

fn default_label() -> Arc<String> {
    Arc::new(format!("inst-{:06x}", rand::random::<u32>()))
}
