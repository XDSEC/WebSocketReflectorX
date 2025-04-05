use slint::{ComponentHandle, PlatformError};
use tracing::info;

use crate::{bridges, ui::MainWindow};

pub fn setup() -> Result<MainWindow, PlatformError> {
    let ui = MainWindow::new()?;

    bridges::setup(&ui);

    info!("Initialization is finished.");
    info!("高性能ですから! (∠・ω< )⌒☆");
    ui.run()?;
    Ok(ui)
}
