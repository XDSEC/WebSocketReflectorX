// StatusIndicator component - Visual status indicator with color coding
use gpui::{Context, Render, Window, div, prelude::*};
use crate::styles::colors;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Success,
    Warning,
    Error,
    Info,
    Inactive,
}

pub struct StatusIndicator {
    status: Status,
    label: Option<String>,
    size: f32,
}

impl StatusIndicator {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            label: None,
            size: 8.0,
        }
    }
    
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    
    fn status_color(&self) -> gpui::Rgba {
        match self.status {
            Status::Success => colors::success(),
            Status::Warning => colors::warning(),
            Status::Error => colors::error(),
            Status::Info => colors::accent(),
            Status::Inactive => gpui::rgba(0x666666ff),
        }
    }
}

impl Render for StatusIndicator {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                div()
                    .w(gpui::px(self.size))
                    .h(gpui::px(self.size))
                    .rounded_full()
                    .bg(self.status_color())
            )
            .children(self.label.as_ref().map(|label| {
                div()
                    .text_sm()
                    .text_color(colors::foreground())
                    .child(label.clone())
            }))
    }
}
