// Icon-only button component
// Based on Zed's IconButton pattern

use super::prelude::*;
use crate::styles::{border_radius, colors, heights, sizes};

#[derive(Clone, Copy, PartialEq)]
pub enum IconButtonStyle {
    Subtle,      // Default - transparent with hover
    Filled,      // Solid background
    Danger,      // Red-themed for destructive actions
}

pub struct IconButton {
    icon_path: &'static str,
    style: IconButtonStyle,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&mut Window, &mut App) + Send + Sync>>,
}

impl IconButton {
    pub fn new(icon_path: &'static str) -> Self {
        Self {
            icon_path,
            style: IconButtonStyle::Subtle,
            disabled: false,
            on_click: None,
        }
    }

    pub fn style(mut self, style: IconButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn background_color(&self) -> gpui::Hsla {
        match self.style {
            IconButtonStyle::Subtle => gpui::transparent_black(),
            IconButtonStyle::Filled => colors::layer_2(),
            IconButtonStyle::Danger => colors::layer_1(),
        }
    }

    fn hover_background_color(&self) -> gpui::Hsla {
        match self.style {
            IconButtonStyle::Subtle => colors::layer_1(),
            IconButtonStyle::Filled => colors::layer_3(),
            IconButtonStyle::Danger => colors::error_bg(),
        }
    }

    fn icon_color(&self) -> gpui::Hsla {
        if self.disabled {
            colors::window_fg().opacity(0.3)
        } else {
            match self.style {
                IconButtonStyle::Danger => colors::error_fg(),
                _ => colors::window_fg(),
            }
        }
    }
}

impl IntoElement for IconButton {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        let id = SharedString::from(format!("icon-button-{}", self.icon_path));

        div()
            .id(id)
            .flex()
            .items_center()
            .justify_center()
            .size(heights::h_md())
            .bg(self.background_color())
            .rounded(border_radius::r_sm())
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|this| this.bg(self.hover_background_color()))
            })
            .when(self.disabled, |this| this.cursor_not_allowed().opacity(0.5))
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |_event, window, cx| {
                    on_click(window, cx);
                })
            })
            .child(
                svg()
                    .path(self.icon_path)
                    .size(sizes::icon_sm())
                    .text_color(self.icon_color()),
            )
    }
}
