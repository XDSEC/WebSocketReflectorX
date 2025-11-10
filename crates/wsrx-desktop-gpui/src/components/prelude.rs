// Component prelude - Common imports for all components
// Following Zed's pattern from crates/ui/src/component_prelude.rs

pub use gpui::{
    App, AppContext, InteractiveElement, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::*, svg,
};

// Component traits
pub use super::traits::{Clickable, Disableable, Selectable, Styleable};
