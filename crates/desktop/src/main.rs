// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::ComponentHandle;
use std::error::Error;

use wsrx_desktop::{launcher, logging};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger.
    let (console_guard, file_guard) = logging::setup()?;
    // Set the platform backend to winit.
    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()))?;

    // Create the main window.
    let ui = launcher::setup()?;
    ui.run().ok();
    drop(console_guard);
    drop(file_guard);
    drop(ui);

    Ok(())
}
