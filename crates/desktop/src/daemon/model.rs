use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, task::JoinHandle};

use crate::ui::MainWindow;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceData {
    #[serde(default = "default_label")]
    pub label: String,
    #[serde(alias = "to")]
    pub remote: String,
    #[serde(alias = "from")]
    pub local: String,
    #[serde(default)]
    pub latency: String,
    #[serde(default)]
    pub scope_host: String,
    #[serde(skip)]
    pub handle: Option<JoinHandle<()>>,
}

pub fn default_label() -> String {
    format!("inst-{:06x}", rand::random::<u32>())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeData {
    pub host: String,
    pub name: String,
    pub state: String,
    pub features: Vec<String>,
}

#[derive(Clone)]
pub struct ServerState {
    pub ui: slint::Weak<MainWindow>,
    pub instances: Arc<RwLock<Vec<InstanceData>>>,
    pub scopes: Arc<RwLock<Vec<ScopeData>>>,
}
