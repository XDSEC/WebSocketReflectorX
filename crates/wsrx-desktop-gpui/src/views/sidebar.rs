// Sidebar view - Navigation sidebar
use gpui::{Context, Render, Window, div, prelude::*, App, SharedString, svg};
use crate::models::Page;
use crate::styles::{colors, spacing, heights, sizes, padding, border_radius};

type PageChangeCallback = Box<dyn Fn(Page, &mut App) + Send + Sync>;

pub struct SidebarView {
    active_page: Page,
    on_page_change: Option<PageChangeCallback>,
}

impl SidebarView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>, active_page: Page) -> Self {
        Self { 
            active_page,
            on_page_change: None,
        }
    }
    
    pub fn set_on_page_change(&mut self, callback: PageChangeCallback) {
        self.on_page_change = Some(callback);
    }
    
    pub fn set_active_page(&mut self, page: Page) {
        self.active_page = page;
    }
    
    fn render_tab(&self, page: Page, icon_path: &'static str, cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = self.active_page == page;
        let label_text: String = match page {
            Page::GetStarted => rust_i18n::t!("get_started").to_string(),
            Page::Connections => rust_i18n::t!("connections").to_string(),
            Page::NetworkLogs => rust_i18n::t!("network_logs").to_string(),
            Page::Settings => rust_i18n::t!("settings").to_string(),
        };
        let id = SharedString::from(format!("sidebar-tab-{:?}", page));
        
        div()
            .id(id)
            .flex()
            .flex_row()
            .items_center()
            .gap(spacing::s_lg())
            .px(padding::p_xl())
            .py(padding::p_lg())
            .rounded(border_radius::r_sm())
            .cursor_pointer()
            .when(is_active, |div| {
                div.bg(colors::layer_3())
                    .border_l_4()
                    .border_color(colors::primary_bg())
            })
            .when(!is_active, |div| {
                div.hover(|div| div.bg(colors::layer_2()))
            })
            .on_click(cx.listener(move |this, _event, _window, cx| {
                // Update our own state first
                this.active_page = page;
                // Then notify parent
                if let Some(ref callback) = this.on_page_change {
                    callback(page, cx);
                }
            }))
            .child(
                svg()
                    .path(icon_path)
                    .size(sizes::icon_md())
                    .text_color(if is_active { colors::primary_bg() } else { colors::window_fg() })
            )
            .child(
                div()
                    .text_color(colors::window_fg())
                    .font_weight(if is_active { gpui::FontWeight::BOLD } else { gpui::FontWeight::NORMAL })
                    .child(label_text)
            )
    }
}

impl Render for SidebarView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_macos = cfg!(target_os = "macos");
        
        div()
            .flex()
            .flex_col()
            .gap(spacing::s_sm())
            .px(padding::p_md())
            .pt(if is_macos { heights::h_lg() } else { padding::p_md() })
            .pb(padding::p_md())
            .bg(colors::layer_1())
            .border_r_1()
            .border_color(colors::element_border())
            .child(self.render_tab(Page::GetStarted, "icons/home.svg", cx))
            .child(self.render_tab(Page::Connections, "icons/globe-star.svg", cx))
            .child(self.render_tab(Page::NetworkLogs, "icons/code.svg", cx))
            .child(
                // Spacer
                div().flex_1()
            )
            .child(self.render_tab(Page::Settings, "icons/settings.svg", cx))
    }
}
