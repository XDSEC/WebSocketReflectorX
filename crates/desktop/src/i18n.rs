//! App translations, injected into woocraft's i18n store.
//!
//! Translations live as flat TOML tables in `locales/<locale>.toml` (locale
//! tags use woocraft's canonical names, e.g. `zh-hans.toml`) and are embedded
//! into the binary. [`init`] parses them and merges each file into
//! woocraft's locale store via [`woocraft::load_locale`], so app strings and
//! woocraft's own component strings resolve through one pipeline
//! ([`woocraft::translate`]).
//!
//! All three [`SUPPORTED_LOCALES`] ship a file — including `en-us`, whose
//! identity mapping is required because rust-i18n's missing-key fallback
//! renders `"<locale>.<key>"`, which would prefix every untranslated string
//! with `en-us.`. The title bar's built-in language menu is restricted to
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

/// Translates `key` in the active locale. Every shipped locale file must
/// cover every key: a missing entry renders rust-i18n's
/// `"<locale>.<key>"` fallback instead of the string.
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
        // The en-us identity file resolves without hitting the fallback.
        assert_eq!(
            woocraft::translate_in_locale("en-us", "Get Started"),
            "Get Started"
        );
        // Keys missing from every locale file render rust-i18n's
        // "<locale>.<key>" fallback — which is why each shipped locale
        // must cover every key.
        assert_eq!(
            woocraft::translate_in_locale("zh-hans", "unknown key"),
            "zh-hans.unknown key"
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

    #[test]
    fn locale_files_cover_the_same_keys() {
        let mut files: Vec<(String, std::collections::HashSet<String>)> = vec![];
        for file in LocaleAssets::iter() {
            let Some(bytes) = LocaleAssets::get(&file) else {
                continue;
            };
            let translations: HashMap<String, String> =
                toml::from_slice(bytes.data.as_ref()).expect("valid locale toml");
            files.push((
                file.trim_end_matches(".toml").to_string(),
                translations.into_keys().collect(),
            ));
        }

        let reference = files
            .iter()
            .find(|(locale, _)| locale == "en-us")
            .expect("en-us.toml must exist (its identity mapping prevents the \
                     locale-prefixed missing-key fallback)");
        for (locale, keys) in &files {
            if locale == "en-us" {
                continue;
            }
            let missing: Vec<_> = reference
                .1
                .difference(keys)
                .map(String::as_str)
                .collect();
            let extra: Vec<_> = keys
                .difference(&reference.1)
                .map(String::as_str)
                .collect();
            assert!(
                missing.is_empty() && extra.is_empty(),
                "{locale} keys diverge from en-us.toml: missing {missing:?}, extra {extra:?}"
            );
        }
    }
}
