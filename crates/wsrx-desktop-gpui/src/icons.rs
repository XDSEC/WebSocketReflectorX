// Embedded SVG icons
// All SVG files are embedded directly into the binary at compile time

pub const HOME: &str = include_str!("../icons/home.svg");
pub const CODE: &str = include_str!("../icons/code.svg");
pub const SETTINGS: &str = include_str!("../icons/settings.svg");
pub const GLOBE_STAR: &str = include_str!("../icons/globe-star.svg");
pub const NAVIGATION: &str = include_str!("../icons/navigation.svg");
pub const LOGO: &str = include_str!("../icons/logo.svg");
pub const LOGO_STROKED: &str = include_str!("../icons/logo-stroked.svg");
pub const WARNING: &str = include_str!("../icons/warning.svg");
pub const DISMISS: &str = include_str!("../icons/dismiss.svg");
pub const MAXIMIZE: &str = include_str!("../icons/maximize.svg");
pub const SUBTRACT: &str = include_str!("../icons/subtract.svg");
pub const LOCK_CLOSED: &str = include_str!("../icons/lock-closed.svg");
pub const CHECKMARK: &str = include_str!("../icons/checkmark.svg");
pub const CHECKBOX_UNCHECKED: &str = include_str!("../icons/checkbox-unchecked.svg");
pub const ARROW_UP_RIGHT: &str = include_str!("../icons/arrow-up-right.svg");
pub const ARROW_SYNC_OFF: &str = include_str!("../icons/arrow-sync-off.svg");

/// Get icon SVG content by name
pub fn get_icon(name: &str) -> Option<&'static str> {
    match name {
        "home" => Some(HOME),
        "code" => Some(CODE),
        "settings" => Some(SETTINGS),
        "globe-star" => Some(GLOBE_STAR),
        "navigation" => Some(NAVIGATION),
        "logo" => Some(LOGO),
        "logo-stroked" => Some(LOGO_STROKED),
        "warning" => Some(WARNING),
        "dismiss" => Some(DISMISS),
        "maximize" => Some(MAXIMIZE),
        "subtract" => Some(SUBTRACT),
        "lock-closed" => Some(LOCK_CLOSED),
        "checkmark" => Some(CHECKMARK),
        "checkbox-unchecked" => Some(CHECKBOX_UNCHECKED),
        "arrow-up-right" => Some(ARROW_UP_RIGHT),
        "arrow-sync-off" => Some(ARROW_SYNC_OFF),
        _ => None,
    }
}
