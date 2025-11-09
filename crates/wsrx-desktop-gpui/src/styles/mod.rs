// Styles - Theme and styling definitions for the application
// This module contains all the styling, colors, and theming configuration

/// Color palette for the application
pub mod colors {
    use gpui::Rgba;

    pub fn background() -> Rgba {
        gpui::rgba(0x1e1e1eff)
    }

    pub fn foreground() -> Rgba {
        gpui::rgba(0xe5e5e5ff)
    }

    pub fn accent() -> Rgba {
        gpui::rgba(0x007accff)
    }

    pub fn error() -> Rgba {
        gpui::rgba(0xf48771ff)
    }

    pub fn warning() -> Rgba {
        gpui::rgba(0xddb76fff)
    }

    pub fn success() -> Rgba {
        gpui::rgba(0x7ec699ff)
    }
}

/// Typography settings
pub mod typography {
    use gpui::{Pixels, px};

    pub fn font_size_xs() -> Pixels {
        px(11.0)
    }

    pub fn font_size_sm() -> Pixels {
        px(12.0)
    }

    pub fn font_size_base() -> Pixels {
        px(14.0)
    }

    pub fn font_size_lg() -> Pixels {
        px(16.0)
    }

    pub fn font_size_xl() -> Pixels {
        px(20.0)
    }
}

/// Spacing constants
pub mod spacing {
    use gpui::{Pixels, px};

    pub fn xs() -> Pixels {
        px(2.0)
    }

    pub fn sm() -> Pixels {
        px(4.0)
    }

    pub fn base() -> Pixels {
        px(8.0)
    }

    pub fn md() -> Pixels {
        px(12.0)
    }

    pub fn lg() -> Pixels {
        px(16.0)
    }

    pub fn xl() -> Pixels {
        px(24.0)
    }
}
