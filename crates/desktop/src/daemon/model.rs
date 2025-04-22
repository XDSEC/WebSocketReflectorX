use std::{fmt::Display, sync::Arc};

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;

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
    pub latency: i32,
    #[serde(default)]
    pub scope_host: String,
    #[serde(skip)]
    pub handle: Option<CancellationToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceDataPure {
    pub label: String,
    pub remote: String,
    pub local: String,
    pub latency: i32,
    pub scope_host: String,
}

impl From<&InstanceData> for InstanceDataPure {
    fn from(data: &InstanceData) -> Self {
        InstanceDataPure {
            label: data.label.clone(),
            remote: data.remote.clone(),
            local: data.local.clone(),
            latency: data.latency,
            scope_host: data.scope_host.clone(),
        }
    }
}

pub fn default_label() -> String {
    format!("inst-{:06x}", rand::random::<u32>())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScopeData {
    pub host: String,
    pub name: String,
    pub state: String,
    pub features: FeatureFlags,
}

#[derive(Clone)]
pub struct ServerState {
    pub ui: slint::Weak<MainWindow>,
    pub instances: Arc<RwLock<Vec<InstanceData>>>,
    pub scopes: Arc<RwLock<Vec<ScopeData>>>,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FeatureFlags: u32 {
        const Basic    = 0b00000001;
        const PingFall = 0b00000010;
    }
}

const FEATURE_MAP: &[(&str, FeatureFlags)] = &[
    ("basic", FeatureFlags::Basic),
    ("pingfall", FeatureFlags::PingFall),
];

impl FeatureFlags {
    pub fn as_feature_vec(&self) -> Vec<&'static str> {
        let mut flags = Vec::new();
        for (flag_str, flag) in FEATURE_MAP {
            if self.contains(*flag) {
                flags.push(*flag_str);
            }
        }
        flags
    }
}

impl Display for FeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_feature_vec().join(","))
    }
}

impl Serialize for FeatureFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_feature_vec().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FeatureFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Vec::<String>::deserialize(deserializer)?.into_iter().into())
    }
}

impl<T> From<T> for FeatureFlags
where
    T: Iterator<Item = String>,
{
    fn from(flags: T) -> Self {
        let mut feature_flags = FeatureFlags::empty();
        for flag in flags {
            for (flag_str, flag_value) in FEATURE_MAP {
                if flag == *flag_str {
                    feature_flags.insert(*flag_value);
                }
            }
        }
        feature_flags
    }
}
