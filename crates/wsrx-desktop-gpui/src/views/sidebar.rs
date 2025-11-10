// Sidebar view - Navigation sidebar
use gpui::{Context, Render, Window, div, prelude::*, App, SharedString};
use crate::models::Page;
use crate::styles::{colors, spacing};

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
    
    fn render_tab(&self, page: Page, label: impl Into<String>, cx: &mut Context<Self>) -> impl IntoElement {
        let is_active = self.active_page == page;
        let label_text = label.into();
        let id = SharedString::from(format!("sidebar-tab-{:?}", page));
        
        div()
            .id(id)
            .flex()
            .items_center()
            .px(spacing::p_md())
            .py(spacing::p_lg())
            .cursor_pointer()
            .when(is_active, |div| {
                div.bg(colors::layer_3())
                    .border_l_2()
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
                div()
                    .text_color(colors::window_fg())
                    .child(label_text)
            )
    }
}

impl Render for SidebarView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(spacing::s_md())
            .pt(spacing::p_md())
            .child(self.render_tab(Page::GetStarted, "Get Started", cx))
            .child(self.render_tab(Page::Connections, "Connections", cx))
            .child(self.render_tab(Page::NetworkLogs, "Network Logs", cx))
            .child(self.render_tab(Page::Settings, "Settings", cx))
    }
}

