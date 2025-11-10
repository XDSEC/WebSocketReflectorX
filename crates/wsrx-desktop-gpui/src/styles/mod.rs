// Styles - Theme and styling definitions for the application
// This module contains all the styling, colors, and theming configuration

/// Color palette for the application
/// Aligned with Slint design system for consistency
pub mod colors {
    use gpui::Rgba;

    // Dark mode palette (matching Slint dark-palette)
    pub fn window_fg() -> Rgba {
        gpui::rgba(0xCDD6F4FF) // #cdd6f4
    }

    pub fn window_bg() -> Rgba {
        gpui::rgba(0x151515FF) // #151515
    }

    pub fn window_alter_bg() -> Rgba {
        gpui::rgba(0x1E1E1EFF) // #1e1e1e
    }

    pub fn primary_bg() -> Rgba {
        gpui::rgba(0x0078D6FF) // #0078D6
    }

    pub fn window_border() -> Rgba {
        gpui::rgba(0x323232FF) // #323232
    }

    pub fn element_border() -> Rgba {
        gpui::rgba(0x2D2D2DFF) // #2d2d2d
    }

    // Legacy aliases for backward compatibility
    pub fn background() -> Rgba {
        window_bg()
    }

    pub fn foreground() -> Rgba {
        window_fg()
    }

    pub fn accent() -> Rgba {
        primary_bg()
    }

    // Semantic colors (matching Slint)
    pub fn error() -> Rgba {
        gpui::rgba(0xEF303FFF) // #ef303f
    }

    pub fn error_bg() -> Rgba {
        error()
    }

    pub fn warning() -> Rgba {
        gpui::rgba(0xE85D03FF) // #e85d03
    }

    pub fn warning_bg() -> Rgba {
        warning()
    }

    pub fn success() -> Rgba {
        gpui::rgba(0x03A44EFF) // #03a44e
    }

    pub fn success_bg() -> Rgba {
        success()
    }

    pub fn info() -> Rgba {
        gpui::rgba(0x0078D6FF) // #0078D6
    }

    pub fn info_bg() -> Rgba {
        info()
    }

    pub fn debug() -> Rgba {
        gpui::rgba(0x808080FF) // #808080
    }

    // Layer colors (for depth/elevation)
    pub fn layer_1() -> Rgba {
        gpui::rgba(0xFFFFFF10) // #ffffff10
    }

    pub fn layer_2() -> Rgba {
        gpui::rgba(0xFFFFFF18) // #ffffff18
    }

    pub fn layer_3() -> Rgba {
        gpui::rgba(0xFFFFFF20) // #ffffff20
    }

    pub fn layer_4() -> Rgba {
        gpui::rgba(0xFFFFFF28) // #ffffff28
    }

    pub fn layer_5() -> Rgba {
        gpui::rgba(0xFFFFFF30) // #ffffff30
    }
}

/// Typography settings (matching Slint font sizes)
pub mod typography {
    use gpui::{Pixels, px};

    pub fn font_size_xs() -> Pixels {
        px(12.0)
    }

    pub fn font_size_sm() -> Pixels {
        px(14.0)
    }

    pub fn font_size_base() -> Pixels {
        px(16.0) // matches Slint font: 16px
    }

    pub fn font_size_lg() -> Pixels {
        px(18.0)
    }

    pub fn font_size_xl() -> Pixels {
        px(20.0)
    }

    pub fn font_size_2xl() -> Pixels {
        px(24.0)
    }
}

/// Spacing constants (matching Slint sizes)
pub mod spacing {
    use gpui::{Pixels, px};

    // Padding sizes (p-*)
    pub fn p_xs() -> Pixels {
        px(1.0)
    }

    pub fn p_sm() -> Pixels {
        px(2.0)
    }

    pub fn p_md() -> Pixels {
        px(4.0)
    }

    pub fn p_lg() -> Pixels {
        px(8.0)
    }

    pub fn p_xl() -> Pixels {
        px(12.0)
    }

    // Spacing sizes (s-*)
    pub fn s_xs() -> Pixels {
        px(1.0)
    }

    pub fn s_sm() -> Pixels {
        px(2.0)
    }

    pub fn s_md() -> Pixels {
        px(4.0)
    }

    pub fn s_lg() -> Pixels {
        px(8.0)
    }

    pub fn s_xl() -> Pixels {
        px(12.0)
    }

    // Legacy aliases
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

/// Border radius constants (matching Slint r-*)
pub mod radius {
    use gpui::{Pixels, px};

    pub fn r_xs() -> Pixels {
        px(2.0)
    }

    pub fn r_sm() -> Pixels {
        px(4.0)
    }

    pub fn r_md() -> Pixels {
        px(6.0)
    }

    pub fn r_lg() -> Pixels {
        px(8.0)
    }

    pub fn r_xl() -> Pixels {
        px(10.0)
    }
}

/// Height constants (matching Slint h-*)
pub mod heights {
    use gpui::{Pixels, px};

    pub fn h_xs() -> Pixels {
        px(16.0)
    }

    pub fn h_sm() -> Pixels {
        px(24.0)
    }

    pub fn h_md() -> Pixels {
        px(32.0)
    }

    pub fn h_lg() -> Pixels {
        px(36.0)
    }

    pub fn h_xl() -> Pixels {
        px(40.0)
    }
}

/// Icon and misc sizes
pub mod sizes {
    use gpui::{Pixels, px};

    pub fn icon_xs() -> Pixels {
        px(12.0)
    }

    pub fn icon_sm() -> Pixels {
        px(16.0)
    }

    pub fn icon_md() -> Pixels {
        px(20.0)
    }

    pub fn icon_lg() -> Pixels {
        px(24.0)
    }

    pub fn icon_xl() -> Pixels {
        px(32.0)
    }
}

// Module aliases for convenience
pub use radius as border_radius;
pub use spacing as padding;
