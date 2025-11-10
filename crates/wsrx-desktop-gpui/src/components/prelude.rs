// Component prelude - Common imports for all components
// Following Zed's pattern from crates/ui/src/component_prelude.rs

pub use gpui::prelude::*;
pub use gpui::{
    div, svg, AnyElement, AnyView, App, AppContext, Context, ElementId, Entity, InteractiveElement,
    IntoElement, MouseButton, ParentElement, Render, SharedString, StatefulInteractiveElement,
    Styled, View, ViewContext, VisualContext, WeakEntity, Window, WindowContext,
};

pub use crate::styles;
