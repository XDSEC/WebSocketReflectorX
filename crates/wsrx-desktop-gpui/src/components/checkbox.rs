// Checkbox component
// Based on Zed's checkbox pattern

use super::prelude::*;
use crate::styles::{border_radius, colors, sizes};

pub struct Checkbox {
    id: SharedString,
    label: SharedString,
    checked: bool,
    disabled: bool,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + Send + Sync>>,
}

impl Checkbox {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            disabled: false,
            on_change: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Checkbox {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        let checked = self.checked;
        let disabled = self.disabled;

        div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .gap(sizes::icon_sm())
            .when(!disabled, |this| this.cursor_pointer())
            .when(disabled, |this| this.cursor_not_allowed().opacity(0.5))
            .when_some(self.on_change, |this, on_change| {
                this.on_click(move |_event, window, cx| {
                    if !disabled {
                        on_change(!checked, window, cx);
                    }
                })
            })
            .child(
                // Checkbox box
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(sizes::icon_md())
                    .rounded(border_radius::r_xs())
                    .border_1()
                    .border_color(if checked {
                        colors::primary_bg()
                    } else {
                        colors::element_border()
                    })
                    .bg(if checked {
                        colors::primary_bg()
                    } else {
                        gpui::transparent_black()
                    })
                    .when(checked, |this| {
                        this.child(
                            // Checkmark icon
                            svg()
                                .path("icons/checkmark.svg")
                                .size(sizes::icon_xs())
                                .text_color(colors::window_bg()),
                        )
                    }),
            )
            .child(
                // Label
                div()
                    .text_color(colors::window_fg())
                    .child(self.label.to_string()),
            )
    }
}
