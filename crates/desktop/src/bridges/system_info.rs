use slint::ComponentHandle;

use crate::ui::{MainWindow, SystemInfoBridge};

pub fn setup(window: &MainWindow) {
    let bridge = window.global::<SystemInfoBridge>();
    #[cfg(target_os = "linux")]
    bridge.set_os("linux".into());
    #[cfg(target_os = "windows")]
    bridge.set_os("windows".into());
    #[cfg(target_os = "macos")]
    bridge.set_os("macos".into());
}
