// Root view - Main application window
use gpui::{Context, Render, Window, div, prelude::*, Entity};
use crate::models::Page;
use crate::styles::colors;
use super::{SidebarView, GetStartedView, ConnectionsView, NetworkLogsView, SettingsView};

pub struct RootView {
    /// Current active page
    current_page: Page,
    
    /// Sidebar entity
    sidebar: Entity<SidebarView>,
    
    /// Page views
    get_started: Entity<GetStartedView>,
    connections: Entity<ConnectionsView>,
    network_logs: Entity<NetworkLogsView>,
    settings: Entity<SettingsView>,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_page = Page::GetStarted;
        
        Self {
            current_page,
            sidebar: cx.new(|cx| SidebarView::new(window, cx, current_page)),
            get_started: cx.new(|cx| GetStartedView::new(window, cx)),
            connections: cx.new(|cx| ConnectionsView::new(window, cx)),
            network_logs: cx.new(|cx| NetworkLogsView::new(window, cx)),
            settings: cx.new(|cx| SettingsView::new(window, cx)),
        }
    }
    
    pub fn set_page(&mut self, page: Page, _cx: &mut Context<Self>) {
        self.current_page = page;
    }
    
    fn render_sidebar(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_64()
            .h_full()
            .bg(gpui::rgba(0x00000020))
            .border_r_1()
            .border_color(gpui::rgba(0x00000050))
            .child(self.sidebar.clone())
    }
    
    fn render_main_content(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .bg(colors::background())
            .child(self.render_page_content())
    }
    
    fn render_page_content(&self) -> impl IntoElement {
        match self.current_page {
            Page::GetStarted => div().child(self.get_started.clone()),
            Page::Connections => div().child(self.connections.clone()),
            Page::NetworkLogs => div().child(self.network_logs.clone()),
            Page::Settings => div().child(self.settings.clone()),
        }
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h_full()
            .bg(colors::background())
            .text_color(colors::foreground())
            .child(self.render_sidebar())
            .child(self.render_main_content())
    }
}

