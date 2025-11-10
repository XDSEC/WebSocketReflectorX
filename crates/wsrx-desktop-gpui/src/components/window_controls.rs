// Window controls component (minimize, maximize, close buttons)
use gpui::{prelude::FluentBuilder, *};

use crate::styles;

pub struct WindowControls {
    window: AnyWindowHandle,
}

impl WindowControls {
    pub fn new(window: AnyWindowHandle) -> Self {
        Self { window }
    }
}

impl Render for WindowControls {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window = self.window.clone();
        let is_macos = cfg!(target_os = "macos");

        div()
            .flex()
            .flex_row()
            .gap(styles::spacing::s_md())
            .when(!is_macos, |this| {
                this
                    // Minimize button
                    .child(
                        div()
                            .id("minimize-btn")
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(styles::heights::h_md())
                            .bg(styles::colors::layer_1())
                            .hover(|this| this.bg(styles::colors::layer_2()))
                            .cursor_pointer()
                            .on_click({
                                let window = window.clone();
                                cx.listener(move |_this, _event, _window, cx| {
                                    window
                                        .update(cx, |_view, window, _cx| {
                                            window.minimize_window();
                                        })
                                        .ok();
                                })
                            })
                            .child(
                                svg()
                                    .path("icons/subtract.svg")
                                    .size(styles::sizes::icon_sm())
                                    .text_color(styles::colors::window_fg()),
                            ),
                    )
                    // Maximize button
                    .child(
                        div()
                            .id("maximize-btn")
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(styles::heights::h_md())
                            .bg(styles::colors::layer_1())
                            .hover(|this| this.bg(styles::colors::layer_2()))
                            .cursor_pointer()
                            .on_click({
                                let window = window.clone();
                                cx.listener(move |_this, _event, _window, cx| {
                                    window
                                        .update(cx, |_view, window, _cx| {
                                            window.zoom_window();
                                        })
                                        .ok();
                                })
                            })
                            .child(
                                svg()
                                    .path("icons/maximize.svg")
                                    .size(styles::sizes::icon_sm())
                                    .text_color(styles::colors::window_fg()),
                            ),
                    )
                    // Close button
                    .child(
                        div()
                            .id("close-btn")
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(styles::heights::h_md())
                            .bg(styles::colors::layer_1())
                            .hover(|this| this.bg(styles::colors::error_bg()))
                            .cursor_pointer()
                            .on_click({
                                let window = window.clone();
                                cx.listener(move |_this, _event, _window, cx| {
                                    window
                                        .update(cx, |_view, window, _cx| {
                                            window.remove_window();
                                        })
                                        .ok();
                                })
                            })
                            .child(
                                svg()
                                    .path("icons/dismiss.svg")
                                    .size(styles::sizes::icon_sm())
                                    .text_color(styles::colors::window_fg()),
                            ),
                    )
            })
    }
}
