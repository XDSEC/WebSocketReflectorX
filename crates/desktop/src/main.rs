// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::ComponentHandle;
use wsrx_desktop::main_window;

fn main() -> Result<(), Box<dyn Error>> {
    // Set the platform backend to winit.
    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap()))?;

    // Create the main window.
    let window = main_window::setup()?;
    window.run()?;

    Ok(())
}
