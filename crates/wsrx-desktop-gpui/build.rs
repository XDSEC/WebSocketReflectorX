// Build script for wsrx-desktop-gpui
// Generates platform-specific constants and resources

use std::env;

fn main() {
    // Generate constants
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = std::path::Path::new(&out_dir).join("constants.rs");

    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
    let target = build_target::target();

    let constants = format!(
        r#"
/// Application version
pub const WSRX_VERSION: &str = "{version}";

/// Platform information
pub const TARGET_OS: &str = "{os}";
pub const TARGET_ARCH: &str = "{arch}";
"#,
        version = version,
        os = target.os,
        arch = target.arch,
    );

    std::fs::write(&out_path, constants).expect("Failed to write constants.rs");

    // Platform-specific build setup
    #[cfg(target_os = "windows")]
    {
        // Setup Windows resources (icon, version info)
        if let Ok(_) = env::var("CARGO_CFG_WINDOWS") {
            let mut res = winres::WindowsResource::new();
            res.set_icon("../../arts/logo.png");
            if let Err(e) = res.compile() {
                eprintln!("Warning: Failed to compile Windows resources: {}", e);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-fapple-link-runtime");
    }
}
