use std::{env, fs, path::Path};

fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_bundled_translations("ui/i18n")
        .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config("ui/main.slint", config).expect("Slint build failed");

    println!("cargo::rerun-if-changed=ui/i18n");
    println!("cargo::rerun-if-env-changed=WSRX_GIT_VERSION");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let git_v = if env::var("WSRX_GIT_VERSION").is_ok() {
        env::var("WSRX_GIT_VERSION").unwrap().to_uppercase()
    } else {
        git_version::git_version!(
            args = ["--abbrev=8", "--always", "--dirty=*", "--match=''"],
            fallback = "unknown"
        )
        .to_uppercase()
    };
    let version = format!(
        "{}-{git_v}-{}",
        env!("CARGO_PKG_VERSION"),
        rustc_version::version().unwrap()
    );
    let full_version = format!(
        "{version}-{}-{}-{}",
        build_target::target_arch().unwrap(),
        build_target::target_os().unwrap(),
        build_target::target_env().unwrap(),
    );
    fs::write(
        dest_path,
        format!("pub const WSRX_VERSION: &str = \"{version}\";\npub const WSRX_FULL_VERSION: &str = \"{full_version}\";\n"),
    )
    .unwrap();
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("ui/assets/logo.ico");
        res.compile().expect("Failed to set icon");
    }
}
