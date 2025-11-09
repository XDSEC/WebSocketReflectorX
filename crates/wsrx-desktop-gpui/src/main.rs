#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use gpui::{App, Application, WindowOptions, WindowBounds, Bounds, Size};

mod logging;

fn main() -> Result<()> {
    // Initialize logging
    let (_console_guard, _file_guard) = logging::setup()?;

    // Create and run the GPUI application
    Application::new().run(|_cx: &mut App| {
        // TODO: Initialize main window and root view
        // This will be implemented in the next migration phase
    });

    Ok(())
}
