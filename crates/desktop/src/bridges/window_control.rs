use std::collections::HashMap;

use i_slint_backend_winit::{WinitWindowAccessor, WinitWindowEventResult};
use slint::ComponentHandle;
use winit::window::ResizeDirection;

use crate::{
    launcher,
    ui::{MainWindow, WindowControlBridge},
};

pub fn setup(window: &MainWindow) {
    let mut resize_map = HashMap::new();
    resize_map.insert("r".to_string(), ResizeDirection::East);
    resize_map.insert("t".to_string(), ResizeDirection::North);
    resize_map.insert("tr".to_string(), ResizeDirection::NorthEast);
    resize_map.insert("tl".to_string(), ResizeDirection::NorthWest);
    resize_map.insert("b".to_string(), ResizeDirection::South);
    resize_map.insert("br".to_string(), ResizeDirection::SouthEast);
    resize_map.insert("bl".to_string(), ResizeDirection::SouthWest);
    resize_map.insert("l".to_string(), ResizeDirection::West);
    let window_weak = window.as_weak();
    window.window().on_winit_window_event(move |w, e| {
        // println!("{:?}", e);
        match e {
            winit::event::WindowEvent::RedrawRequested => {
                let window = window_weak.unwrap();
                if window.get_main_window_maximized() != w.is_maximized() {
                    window.set_main_window_maximized(w.is_maximized());
                }
                if window.get_main_window_minimized() != w.is_minimized() {
                    window.set_main_window_minimized(w.is_minimized());
                }
                WinitWindowEventResult::Propagate
            }
            winit::event::WindowEvent::CloseRequested => {
                launcher::shutdown(&window_weak);
                WinitWindowEventResult::PreventDefault
            }
            _ => WinitWindowEventResult::Propagate,
        }
    });

    let window_weak = window.as_weak();
    window.on_main_window_resize(move |resize_direction_str| {
        let direction = resize_map
            .get(&resize_direction_str.to_lowercase())
            .unwrap();
        let app_clone = window_weak.unwrap();
        app_clone.window().with_winit_window(|winit_window| {
            let _ = winit_window.drag_resize_window(*direction);
        });
    });

    let window_control_bridge = window.global::<WindowControlBridge>();
    let window_clone_pin = window.as_weak();
    window_control_bridge.on_start_drag(move || {
        let window_clone = window_clone_pin.unwrap();
        window_clone.window().with_winit_window(|winit_window| {
            winit_window.drag_window().ok();
        });
    });
    let window_clone_pin = window.as_weak();
    window_control_bridge.on_close(move || {
        // TODO: system tray implementation
        launcher::shutdown(&window_clone_pin);
    });
    let window_clone_pin = window.as_weak();
    window_control_bridge.on_maximize(move || {
        let window_clone = window_clone_pin.unwrap();
        window_clone.window().with_winit_window(|winit_window| {
            winit_window.set_maximized(!winit_window.is_maximized());
        });
    });
    let window_clone_pin = window.as_weak();
    window_control_bridge.on_minimize(move || {
        let window_clone = window_clone_pin.unwrap();
        window_clone.window().with_winit_window(|winit_window| {
            winit_window.set_minimized(!winit_window.is_minimized().unwrap_or(false));
        });
    });
}
