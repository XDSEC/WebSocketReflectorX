// Prevent console window in addition to Slint window in Windows release builds
// when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::ComponentHandle;
use wsrx_desktop::{launcher, logging};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger.
    let (console_guard, file_guard) = logging::setup()?;

    // Set the platform backend to winit.
    #[cfg(not(target_os = "macos"))]
    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()))?;

    #[cfg(target_os = "macos")]
    {
        use winit::platform::macos::WindowAttributesExtMacOS;

        let mut backend = i_slint_backend_winit::Backend::new().unwrap();
        backend.window_attributes_hook = Some(Box::new(|attr| {
            attr.with_fullsize_content_view(true)
                .with_title_hidden(true)
                .with_titlebar_transparent(true)
        }));

        slint::platform::set_platform(Box::new(backend))?;
    }

    // Create the main window.
    let ui = launcher::setup()?;
    ui.run().ok();
    drop(console_guard);
    drop(file_guard);
    drop(ui);

    Ok(())
}
