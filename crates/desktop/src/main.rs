// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use tracing::info;
use wsrx_desktop::{logging, main_window};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger.
    let (console_guard, file_guard) = logging::setup()?;
    // Set the platform backend to winit.
    info!("WSRX Desktop is initializing...");
    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()))?;

    // Create the main window.
    let window = main_window::setup()?;

    drop(console_guard);
    drop(file_guard);
    drop(window);

    Ok(())
}
