// Settings bridge - Application settings persistence
use std::path::PathBuf;

use anyhow::Result;
use directories::ProjectDirs;

use crate::models::Settings;

/// Bridge for managing application settings persistence
pub struct SettingsBridge {
    /// Path to settings file
    settings_path: PathBuf,
}

impl SettingsBridge {
    /// Create a new settings bridge
    pub fn new() -> Result<Self> {
        let settings_path = Self::get_settings_path()?;
        Ok(Self { settings_path })
    }

    /// Get the settings file path
    fn get_settings_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("org", "xdsec", "wsrx-desktop-gpui")
            .ok_or_else(|| anyhow::anyhow!("Could not determine settings directory"))?;

        let config_dir = proj_dirs.config_dir();
        std::fs::create_dir_all(config_dir)?;

        Ok(config_dir.join("settings.toml"))
    }

    /// Load settings from file
    pub fn load(&self) -> Result<Settings> {
        if !self.settings_path.exists() {
            return Ok(Settings::default());
        }

        let content = std::fs::read_to_string(&self.settings_path)?;
        let settings: Settings = toml::from_str(&content)?;
        Ok(settings)
    }

    /// Save settings to file
    pub fn save(&self, settings: &Settings) -> Result<()> {
        let content = toml::to_string_pretty(settings)?;
        std::fs::write(&self.settings_path, content)?;
        Ok(())
    }
}

impl Default for SettingsBridge {
    fn default() -> Self {
        Self::new().expect("Failed to initialize settings bridge")
    }
}
