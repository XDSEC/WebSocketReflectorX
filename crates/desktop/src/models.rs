use std::{collections::HashMap, fmt::Display};

use bitflags::bitflags;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A single proxied tunnel instance shown in the UI.
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

/// A scope (a website origin) that requested / obtained control of wsrx.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScopeData {
    pub host: String,
    pub name: String,
    pub state: String,
    pub features: FeatureFlags,
    #[serde(default)]
    pub settings: HashMap<String, Value>,
}

/// Desktop application configuration, persisted to `config.toml`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsrxDesktopConfig {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_running_in_tray")]
    pub running_in_tray: bool,
    #[serde(default = "default_language")]
    pub language: String,
}

impl Default for WsrxDesktopConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            running_in_tray: default_running_in_tray(),
            language: default_language(),
        }
    }
}

/// A log entry streamed from the JSON log file into the UI.
#[derive(Clone, Debug, Deserialize, Default)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub target: String,
    pub fields: LogEntryFields,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct LogEntryFields {
    pub message: String,
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
    pub status: Vec<u16>,
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

fn default_label() -> String {
    format!("inst-{:06x}", rand::random::<u32>())
}

// handle "en-US" / "en" / "zh" / "zh-CN" / "zh-Hans-CN" / "zh-Hant-TW"
// into one of "en_US" / "zh_CN" / "zh_TW"
pub fn normalize_language(locale: String) -> String {
    let mut parts = locale.split('-');
    let lang = parts.next().unwrap_or("en");
    let region = parts.next().map(|s| match s {
        "CN" => "CN",
        "TW" => "TW",
        "HK" => "TW",
        "Hans" => "CN",
        "Hant" => "TW",
        _ => "US",
    });

    match lang {
        "en" => format!("en_{}", region.unwrap_or("US")),
        "zh" => format!("zh_{}", region.unwrap_or("CN")),
        _ => {
            tracing::warn!("Unsupported language: {}, defaulting to en_US", locale);
            "en_US".to_string()
        }
    }
}

pub fn default_language() -> String {
    sys_locale::get_locale()
        .map(normalize_language)
        .unwrap_or_else(|| {
            tracing::warn!("Failed to get system locale, defaulting to en_US");
            "en_US".to_string()
        })
}

fn default_theme() -> String {
    "dark".to_string()
}

const fn default_running_in_tray() -> bool {
    false
}
