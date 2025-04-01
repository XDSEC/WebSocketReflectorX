pub mod storage;
pub mod system_info;
pub mod window_control;

use crate::ui::MainWindow;

pub fn setup(window: &MainWindow) {
    window_control::setup(window);
    system_info::setup(window);
    storage::setup(window);
}
