pub mod settings;
pub mod system_info;
pub mod ui_state;
pub mod window_control;

use crate::ui::MainWindow;

pub fn setup(window: &MainWindow) {
    window_control::setup(window);
    system_info::setup(window);
    ui_state::setup(window);
}
