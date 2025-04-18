use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use slint::ComponentHandle;
use tracing::{debug, error, warn};

use crate::ui::{MainWindow, SettingsBridge};

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

// handle "en-US" / "en" / "zh" / "zh-CN" / "zh-Hans-CN" / "zh-Hant-TW"
// into one of "en_US" / "zh_CN" / "zh_TW"
fn normalize_language(locale: String) -> String {
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
            warn!("Unsupported language: {}, defaulting to en_US", locale);
            "en_US".to_string()
        }
    }
}

fn default_language() -> String {
    sys_locale::get_locale()
        .map(normalize_language)
        .unwrap_or_else(|| {
            warn!("Failed to get system locale, defaulting to en_US");
            "en_US".to_string()
        })
}

fn default_theme() -> String {
    "dark".to_string()
}

const fn default_running_in_tray() -> bool {
    false
}

pub fn load_config(window: &MainWindow) {
    let bridge = window.global::<SettingsBridge>();
    let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
        Some(dirs) => dirs,
        None => {
            error!("Unable to find project config directories");
            return;
        }
    };
    let config_file = proj_dirs.config_dir().join("config.toml");
    let config = match std::fs::read_to_string(&config_file) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to read config file: {}", e);
            "".to_string()
        }
    };
    let config: WsrxDesktopConfig = toml::from_str(&config).unwrap_or_default();
    debug!("Loaded config: {:?}", config);
    bridge.set_theme(config.theme.into());
    slint::select_bundled_translation(&config.language).ok();
    bridge.set_language(config.language.into());
    bridge.set_running_in_tray(config.running_in_tray);

    let window_clone = window.as_weak();
    bridge.on_change_language(move |lang| {
        let window = window_clone.upgrade().unwrap();
        let bridge = window.global::<SettingsBridge>();
        bridge.set_language(lang.clone());

        slint::select_bundled_translation(lang.as_str()).ok();
    });
}

pub fn save_config(window: &MainWindow) {
    let bridge = window.global::<SettingsBridge>();
    let proj_dirs = match ProjectDirs::from("org", "xdsec", "wsrx") {
        Some(dirs) => dirs,
        None => {
            error!("Unable to find project config directories");
            return;
        }
    };
    let config_file = proj_dirs.config_dir().join("config.toml");
    let config = WsrxDesktopConfig {
        theme: bridge.get_theme().into(),
        running_in_tray: bridge.get_running_in_tray(),
        language: bridge.get_language().into(),
    };
    debug!("Saving config: {:?}", config);
    let config = toml::to_string(&config).unwrap_or_else(|e| {
        error!("Failed to serialize config: {}", e);
        String::new()
    });
    if let Err(e) = std::fs::create_dir_all(proj_dirs.config_dir()) {
        error!("Failed to create config directory: {}", e);
        return;
    }
    if let Err(e) = std::fs::write(&config_file, config) {
        error!("Failed to write config file: {}", e);
    }
}
