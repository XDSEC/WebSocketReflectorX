// i18n - Internationalization support using rust-i18n
// Provides multi-language support with YAML locale files
// NOTE: The i18n! macro is initialized in lib.rs at crate root

// Re-export functions for convenience
pub use rust_i18n::{t, set_locale, locale};

/// Set application language
pub fn set_language(locale_str: &str) {
    set_locale(locale_str);
}

/// Get current language
pub fn current_language() -> String {
    locale().to_string()
}

/// Detect system locale and set it
pub fn init_locale() {
    if let Some(locale) = sys_locale::get_locale() {
        // Map system locale to supported locales
        let locale = match locale.as_str() {
            l if l.starts_with("zh") => "zh-CN",
            _ => "en",
        };
        set_language(locale);
    }
}

