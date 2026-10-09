//! App translations, injected into woocraft's i18n store.
//!
//! Translations live as flat TOML tables in `locales/<locale>.toml` (locale
//! tags use woocraft's canonical names, e.g. `zh-hans.toml`) and are embedded
//! into the binary. [`init`] parses them and merges each file into
//! woocraft's locale store via [`woocraft::load_locale`], so app strings and
//! woocraft's own component strings resolve through one pipeline
//! ([`woocraft::translate`]).
//!
//! The app ships translations for `zh-hans` and `zh-hant` only; `en-us`
//! needs no file because missing keys fall back to the (English) key itself.
//! The title bar's built-in language menu is restricted to
//! [`SUPPORTED_LOCALES`], and the persisted setting stores the same tags
//! (legacy `en_US` / `zh_CN` / `zh_TW` values are accepted and normalized).

use std::collections::HashMap;

use rust_embed::RustEmbed;

/// The locales offered by the title bar's language menu and accepted by
/// [`set_locale`], in menu order.
pub const SUPPORTED_LOCALES: [&str; 3] = ["en-us", "zh-hans", "zh-hant"];

/// Embedded `locales/*.toml` translation tables.
#[derive(RustEmbed)]
#[folder = "locales"]
struct LocaleAssets;

/// Loads every embedded `locales/*.toml` into woocraft's i18n store. Call
/// once at startup, before any string is resolved.
pub fn init() {
    for file in LocaleAssets::iter() {
        let Some(bytes) = LocaleAssets::get(&file) else {
            continue;
        };
        let locale = file.trim_end_matches(".toml");
        match toml::from_slice::<HashMap<String, String>>(bytes.data.as_ref()) {
            Ok(translations) => {
                woocraft::load_locale(locale, translations);
            }
            Err(err) => {
                tracing::error!("failed to parse locale file {file}: {err}");
            }
        }
    }
}

/// Maps `locale` onto one of [`SUPPORTED_LOCALES`]: woocraft-normalized
/// (so `en_US`, `zh` and `zh-Hans-CN` all work), then defaulted to `en-us`
/// when the app ships no translation for it.
pub fn normalize(locale: &str) -> &'static str {
    let normalized = woocraft::normalize_locale(locale);
    SUPPORTED_LOCALES
        .iter()
        .find(|supported| **supported == normalized)
        .copied()
        .unwrap_or(SUPPORTED_LOCALES[0])
}

/// Sets the active locale (clamped to [`SUPPORTED_LOCALES`]).
pub fn set_locale(locale: &str) {
    woocraft::set_locale(normalize(locale));
}

/// Translates `key` in the active locale, falling back to the key itself
/// (i.e. the English source string) when no translation exists.
pub fn t(key: &str) -> String {
    woocraft::translate(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_translations_into_woocraft() {
        init();
        assert_eq!(
            woocraft::translate_in_locale("zh-hans", "Get Started"),
            "开始使用"
        );
        assert_eq!(
            woocraft::translate_in_locale("zh-hant", "Get Started"),
            "開始使用"
        );
        // en-us ships no file: missing keys fall back to the key itself.
        assert_eq!(
            woocraft::translate_in_locale("en-us", "Get Started"),
            "Get Started"
        );
        // Keys the app does not translate fall back verbatim.
        assert_eq!(
            woocraft::translate_in_locale("zh-hans", "unknown key"),
            "unknown key"
        );
    }

    #[test]
    fn normalizes_locale_tags() {
        assert_eq!(normalize("zh_CN"), "zh-hans");
        assert_eq!(normalize("zh-Hant-TW"), "zh-hant");
        assert_eq!(normalize("en"), "en-us");
        // Locales the app does not ship default to en-us.
        assert_eq!(normalize("fr-FR"), "en-us");
    }
}
