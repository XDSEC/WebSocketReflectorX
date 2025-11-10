// Component traits - Common interfaces for UI components
// Following Zed's pattern for reusable component behavior

#![allow(dead_code)] // Traits defined for future use

use gpui::{Context, Window};

/// Trait for components that can be clicked
pub trait Clickable {
    fn on_click<F>(self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) + Send + Sync + 'static;
}

/// Trait for components that can be disabled
pub trait Disableable {
    fn disabled(self, disabled: bool) -> Self;
}

/// Trait for components with styled variants
pub trait Styleable {
    type Style;
    fn style(self, style: Self::Style) -> Self;
}

/// Trait for components that can be selected from a list
pub trait Selectable {
    type Item;
    fn selected(self, item: Self::Item) -> Self;
    fn on_select<F>(self, handler: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>, Self::Item) + Send + Sync + 'static;
}
