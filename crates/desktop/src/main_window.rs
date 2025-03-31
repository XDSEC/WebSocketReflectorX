use slint::{ComponentHandle, PlatformError};

use crate::{bridges, ui::MainWindow};

pub fn setup() -> Result<MainWindow, PlatformError> {
    let ui = MainWindow::new()?;

    bridges::setup(&ui);

    ui.run()?;
    Ok(ui)
}
