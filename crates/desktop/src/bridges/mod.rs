pub mod window_control;

use crate::ui::MainWindow;

pub fn setup(window: &MainWindow) {
    window_control::setup(window);
}
