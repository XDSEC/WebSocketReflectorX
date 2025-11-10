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
        mut self, handler: impl Fn(bool, &mut Window, &mut App) + Send + Sync + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Checkbox {
    type Element = gpui::AnyElement;

    fn into_element(self) -> Self::Element {
        let checked = self.checked;
        let disabled = self.disabled;
        let id = self.id.clone();
        let label = self.label.to_string();

        let mut root_div = div()
            .id(id)
            .flex()
            .flex_row()
            .items_center()
            .gap(sizes::icon_sm());

        if !disabled {
            root_div = root_div.cursor_pointer();
        }

        if disabled {
            root_div = root_div.cursor_not_allowed().opacity(0.5);
        }

        if let Some(on_change) = self.on_change {
            root_div = root_div.on_click(move |_event, window, cx| {
                if !disabled {
                    on_change(!checked, window, cx);
                }
            });
        }

        let checkbox_box = {
            let mut box_div = div()
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
                    gpui::rgba(0x00000000)
                });

            if checked {
                box_div = box_div.child(
                    // Checkmark icon
                    svg()
                        .path("icons/checkmark.svg")
                        .size(sizes::icon_xs())
                        .text_color(colors::window_bg()),
                );
            }

            box_div
        };

        root_div
            .child(checkbox_box)
            .child(
                // Label
                div().text_color(colors::window_fg()).child(label),
            )
            .into_any_element()
    }
}
