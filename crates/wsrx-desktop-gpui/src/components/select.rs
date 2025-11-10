// Select component - Dropdown selection component
// Following Zed's pattern with traits for reusability

use gpui::{Context, Render, SharedString, Window, div, prelude::*};

use super::traits::{Disableable, Selectable};
use crate::styles::colors;

pub struct Select<T> {
    id: SharedString,
    placeholder: String,
    options: Vec<T>,
    selected_index: Option<usize>,
    disabled: bool,
    on_select: Option<Box<dyn Fn(&mut Window, &mut Context<Self>, T) + Send + Sync>>,
}

impl<T> Select<T>
where
    T: Clone + ToString + 'static,
{
    pub fn new(id: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            placeholder: "Select an option...".to_string(),
            options: Vec::new(),
            selected_index: None,
            disabled: false,
            on_select: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn options(mut self, options: Vec<T>) -> Self {
        self.options = options;
        self
    }

    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_index = index;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<T> Selectable for Select<T>
where
    T: Clone + ToString + 'static,
{
    type Item = T;

    fn selected(mut self, item: Self::Item) -> Self {
        if let Some(index) = self
            .options
            .iter()
            .position(|opt| opt.to_string() == item.to_string())
        {
            self.selected_index = Some(index);
        }
        self
    }

    fn on_select<F>(mut self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>, Self::Item) + Send + Sync + 'static,
    {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl<T> Disableable for Select<T>
where
    T: Clone + ToString + 'static,
{
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<T> Render for Select<T>
where
    T: Clone + ToString + 'static,
{
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_text = self
            .selected_index
            .and_then(|idx| self.options.get(idx))
            .map(|item| item.to_string())
            .unwrap_or_else(|| self.placeholder.clone());

        let disabled = self.disabled;
        let options = self.options.clone();

        div()
            .id(self.id.clone())
            .relative()
            .w_full()
            .px_3()
            .py_2()
            .rounded_md()
            .bg(colors::background())
            .text_color(colors::foreground())
            .cursor_pointer()
            .when(!disabled, |div| {
                div.hover(|div| div.bg(gpui::rgba(0x2A2A2AFF)))
                    .on_click(cx.listener(move |this, _event, window, cx| {
                        // For now, just cycle through options on click
                        // TODO: Implement proper dropdown with overlay
                        let next_index = this
                            .selected_index
                            .map(|idx| (idx + 1) % this.options.len())
                            .unwrap_or(0);
                        this.selected_index = Some(next_index);

                        if let (Some(index), Some(callback)) =
                            (this.selected_index, &this.on_select)
                        {
                            if let Some(item) = this.options.get(index).cloned() {
                                callback(window, cx, item);
                            }
                        }

                        cx.notify();
                    }))
            })
            .when(disabled, |div| {
                div.bg(gpui::rgba(0x1A1A1AFF))
                    .text_color(gpui::rgba(0x666666FF))
            })
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(selected_text)
                    .child("▼"), // Simple dropdown arrow
            )
    }
}
