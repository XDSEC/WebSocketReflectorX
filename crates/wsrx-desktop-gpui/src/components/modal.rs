// Modal component - Modal dialog overlay
use gpui::{AnyElement, Context, Render, Window, div, prelude::*};

use crate::styles::colors;

pub struct Modal {
    title: String,
    content: Option<AnyElement>,
    show_close: bool,
    on_close: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) + Send + Sync>>,
}

impl Modal {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: None,
            show_close: true,
            on_close: None,
        }
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn show_close(mut self, show: bool) -> Self {
        self.show_close = show;
        self
    }

    pub fn on_close<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) + Send + Sync + 'static,
    {
        self.on_close = Some(Box::new(callback));
        self
    }
}

impl Render for Modal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title.clone();
        let show_close = self.show_close;

        div()
            .flex()
            .absolute()
            .top_0()
            .left_0()
            .w_full()
            .h_full()
            .items_center()
            .justify_center()
            .bg(gpui::rgba(0x00000099)) // Semi-transparent overlay
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(gpui::relative(0.8))
                    .max_w(gpui::px(600.0))
                    .bg(gpui::rgba(0x2A2A2AFF))
                    .rounded_lg()
                    .shadow_lg()
                    .child(
                        // Header
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_6()
                            .py_4()
                            .border_b_1()
                            .border_color(gpui::rgba(0x444444FF))
                            .child(
                                div()
                                    .text_xl()
                                    .text_color(colors::foreground())
                                    .child(title),
                            )
                            .when(show_close, |container| {
                                container.child(
                                    div()
                                        .id("modal-close")
                                        .px_2()
                                        .py_1()
                                        .cursor_pointer()
                                        .hover(|div| div.bg(gpui::rgba(0x444444FF)))
                                        .rounded_md()
                                        .on_click(cx.listener(|this, _event, window, cx| {
                                            if let Some(ref callback) = this.on_close {
                                                callback(window, cx);
                                            }
                                        }))
                                        .child("✕"),
                                )
                            }),
                    )
                    .child(
                        // Content
                        div().px_6().py_4().children(self.content.take()),
                    ),
            )
    }
}
