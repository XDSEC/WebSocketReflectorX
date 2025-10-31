use std::{collections::HashMap, fmt::Display, sync::Arc};

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{net::TcpListener, sync::RwLock};
use wsrx::tunnel::Tunnel;

use super::default_label;
use crate::ui::{Instance, MainWindow};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScopeData {
    pub host: String,
    pub name: String,
    pub state: String,
    pub features: FeatureFlags,
    #[serde(default)]
    pub settings: HashMap<String, Value>,
}

#[derive(Clone)]
pub struct ServerState {
    pub ui: slint::Weak<MainWindow>,
    pub instances: Arc<RwLock<Vec<ProxyInstance>>>,
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

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicSettings {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PingFallSettings {
    pub fail_status: Vec<u16>,
    pub drop_unknown: bool,
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

pub struct ProxyInstance {
    pub data: InstanceData,
    _tunnel: Tunnel,
}

impl ProxyInstance {
    pub fn new(
        label: impl AsRef<str>, scope_host: impl AsRef<str>, listener: TcpListener,
        remote: impl AsRef<str>,
    ) -> Self {
        let tunnel = Tunnel::new(remote.as_ref(), listener);

        Self {
            data: InstanceData {
                label: label.as_ref().to_string(),
                remote: remote.as_ref().to_string(),
                local: tunnel.local.clone(),
                latency: -1,
                scope_host: scope_host.as_ref().to_string(),
            },
            _tunnel: tunnel,
        }
    }
}

impl From<&ProxyInstance> for InstanceData {
    fn from(value: &ProxyInstance) -> Self {
        value.data.clone()
    }
}

impl From<&ProxyInstance> for Instance {
    fn from(value: &ProxyInstance) -> Self {
        Instance {
            label: value.label.as_str().into(),
            remote: value.remote.as_str().into(),
            local: value.local.as_str().into(),
            latency: value.latency,
            scope_host: value.scope_host.as_str().into(),
        }
    }
}

impl std::ops::Deref for ProxyInstance {
    type Target = InstanceData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for ProxyInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
