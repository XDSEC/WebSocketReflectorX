// Input component - Text input field with consistent styling
use gpui::{Context, Render, Window, div, prelude::*, SharedString};
use crate::styles::colors;

pub struct Input {
    id: String,
    placeholder: String,
    value: String,
    disabled: bool,
    on_change: Option<Box<dyn Fn(String, &mut Window, &mut Context<Self>) + Send + Sync>>,
}

impl Input {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            placeholder: String::new(),
            value: String::new(),
            disabled: false,
            on_change: None,
        }
    }
    
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(String, &mut Window, &mut Context<Self>) + Send + Sync + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }
}

impl Render for Input {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let id = SharedString::from(format!("input-{}", self.id));
        let placeholder = if self.value.is_empty() {
            self.placeholder.clone()
        } else {
            String::new()
        };
        let value = self.value.clone();
        let disabled = self.disabled;
        
        div()
            .id(id)
            .flex()
            .items_center()
            .px_3()
            .py_2()
            .rounded_md()
            .border_1()
            .when(!disabled, |div| {
                div.bg(gpui::rgba(0x2a2a2aff))
                    .border_color(gpui::rgba(0x444444ff))
                    .hover(|div| div.border_color(colors::accent()))
            })
            .when(disabled, |div| {
                div.bg(gpui::rgba(0x1a1a1aff))
                    .border_color(gpui::rgba(0x333333ff))
                    .text_color(gpui::rgba(0x666666ff))
            })
            .text_color(colors::foreground())
            .child(if value.is_empty() && !placeholder.is_empty() {
                div()
                    .text_color(gpui::rgba(0x888888ff))
                    .child(placeholder)
            } else {
                div().child(value)
            })
    }
}
