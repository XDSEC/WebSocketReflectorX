//! Desktop app assets (the logo), served alongside woocraft's built-in
//! assets through a combined asset source.

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct DesktopAssets;

/// Builds the app-wide asset source: woocraft's built-in assets first, then
/// the desktop app's own embedded assets (e.g. `logo.png`).
pub fn asset_source() -> woocraft::CombinedSource {
    woocraft::CombinedSource::new()
        .with(woocraft::Assets)
        .with(woocraft::EmbeddedSource::<DesktopAssets>::new())
}
