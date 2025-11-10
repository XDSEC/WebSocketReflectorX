// Button component - Reusable button with consistent styling
use gpui::{Context, Render, SharedString, Window, div, prelude::*};

use crate::styles::colors;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

pub struct Button {
    label: String,
    variant: ButtonVariant,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&mut Window, &mut Context<Self>) + Send + Sync>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Primary,
            disabled: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    fn bg_color(&self) -> gpui::Rgba {
        match self.variant {
            ButtonVariant::Primary => colors::accent(),
            ButtonVariant::Secondary => gpui::rgba(0x444444FF),
            ButtonVariant::Danger => colors::error(),
        }
    }

    fn hover_color(&self) -> gpui::Rgba {
        match self.variant {
            ButtonVariant::Primary => gpui::rgba(0x0088DDFF),
            ButtonVariant::Secondary => gpui::rgba(0x555555FF),
            ButtonVariant::Danger => gpui::rgba(0xFF6655FF),
        }
    }
}

impl Render for Button {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let id = SharedString::from(format!("button-{}", self.label));
        let label = self.label.clone();
        let disabled = self.disabled;

        div()
            .id(id)
            .px_4()
            .py_2()
            .rounded_md()
            .cursor_pointer()
            .when(!disabled, |div| {
                div.bg(self.bg_color())
                    .hover(|div| div.bg(self.hover_color()))
                    .on_click(cx.listener(|this, _event, window, cx| {
                        if let Some(ref callback) = this.on_click {
                            callback(window, cx);
                        }
                    }))
            })
            .when(disabled, |div| {
                div.bg(gpui::rgba(0x333333FF))
                    .text_color(gpui::rgba(0x666666FF))
            })
            .text_color(colors::foreground())
            .child(label)
    }
}
