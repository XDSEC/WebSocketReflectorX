// Title bar component
use gpui::{prelude::FluentBuilder, *};

use crate::{components::WindowControls, styles};

pub struct TitleBar {
    window: AnyWindowHandle,
    show_sidebar_callback: Option<Box<dyn Fn(&mut App) + Send + Sync>>,
}

impl TitleBar {
    pub fn new(window: AnyWindowHandle) -> Self {
        Self {
            window,
            show_sidebar_callback: None,
        }
    }

    pub fn set_show_sidebar_callback(&mut self, callback: Box<dyn Fn(&mut App) + Send + Sync>) {
        self.show_sidebar_callback = Some(callback);
    }
}

impl Render for TitleBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window = self.window;
        let is_macos = cfg!(target_os = "macos");

        div()
            .id("title-bar")
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .h(styles::heights::h_md() + styles::padding::p_md() * 2.0)
            .px(styles::padding::p_md())
            .py(styles::padding::p_md())
            .gap(styles::spacing::s_md())
            .bg(gpui::transparent_black())
            // Drag area
            .on_mouse_down(MouseButton::Left, {
                cx.listener(move |_this, _event: &MouseDownEvent, _window, cx| {
                    window
                        .update(cx, |_view, window, _cx| {
                            window.start_window_move();
                        })
                        .ok();
                })
            })
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(styles::spacing::s_md())
                    .when(!is_macos, |this| {
                        this.child(
                            div()
                                .id("toggle-sidebar-btn")
                                .flex()
                                .items_center()
                                .justify_center()
                                .size(styles::heights::h_md())
                                .bg(styles::colors::layer_1())
                                .hover(|this| this.bg(styles::colors::layer_2()))
                                .rounded(styles::border_radius::r_sm())
                                .cursor_pointer()
                                .on_click(cx.listener(|this, _event, _window, cx| {
                                    if let Some(ref callback) = this.show_sidebar_callback {
                                        callback(cx);
                                    }
                                }))
                                .child(
                                    svg()
                                        .path("navigation")
                                        .size(styles::sizes::icon_sm())
                                        .text_color(styles::colors::window_fg()),
                                ),
                        )
                    }),
            )
            .child(
                // Center spacer
                div().flex_1(),
            )
            .child(cx.new(|_cx| WindowControls::new(window)))
    }
}
