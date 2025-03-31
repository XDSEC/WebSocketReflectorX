fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_bundled_translations("ui/i18n")
        .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config("ui/main.slint", config).expect("Slint build failed");
}
