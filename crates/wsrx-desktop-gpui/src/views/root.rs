// Root view - Main application window
use gpui::{Context, Render, Window, div, prelude::*, Entity, AnyWindowHandle};
use crate::models::Page;
use crate::styles::colors;
use crate::components::TitleBar;
use super::{SidebarView, GetStartedView, ConnectionsView, NetworkLogsView, SettingsView};

pub struct RootView {
    /// Window handle
    window: AnyWindowHandle,
    
    /// Current active page
    current_page: Page,
    
    /// Title bar
    title_bar: Entity<TitleBar>,
    
    /// Sidebar entity
    sidebar: Entity<SidebarView>,
    
    /// Page views
    get_started: Entity<GetStartedView>,
    connections: Entity<ConnectionsView>,
    network_logs: Entity<NetworkLogsView>,
    settings: Entity<SettingsView>,
    
    /// Sidebar visibility
    show_sidebar: bool,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let current_page = Page::GetStarted;
        let window_handle = window.window_handle();
        
        let root = Self {
            window: window_handle.clone(),
            current_page,
            show_sidebar: true,
            title_bar: cx.new(|_cx| TitleBar::new(window_handle.clone())),
            sidebar: cx.new(|cx| SidebarView::new(window, cx, current_page)),
            get_started: cx.new(|cx| GetStartedView::new(window, cx)),
            connections: cx.new(|cx| ConnectionsView::new(window, cx)),
            network_logs: cx.new(|cx| NetworkLogsView::new(window, cx)),
            settings: cx.new(|cx| SettingsView::new(window, cx)),
        };
        
        // Set up the navigation callback for sidebar
        let weak_self = cx.weak_entity();
        root.sidebar.update(cx, |sidebar, _| {
            sidebar.set_on_page_change(Box::new(move |page, cx| {
                if let Some(root) = weak_self.upgrade() {
                    root.update(cx, |root, cx| {
                        root.set_page(page, cx);
                    });
                }
            }));
        });
        
        // Set up title bar sidebar toggle callback
        let weak_self = cx.weak_entity();
        root.title_bar.update(cx, |title_bar, _| {
            title_bar.set_show_sidebar_callback(Box::new(move |cx| {
                if let Some(root) = weak_self.upgrade() {
                    root.update(cx, |root, cx| {
                        root.toggle_sidebar(cx);
                    });
                }
            }));
        });
        
        root
    }
    
    pub fn set_page(&mut self, page: Page, cx: &mut Context<Self>) {
        self.current_page = page;
        cx.notify(); // Trigger re-render
    }
    
    pub fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.show_sidebar = !self.show_sidebar;
        cx.notify();
    }
    
    fn render_sidebar(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .when(self.show_sidebar, |div| div.w_64())
            .when(!self.show_sidebar, |div| div.w_0())
            .h_full()
            .overflow_hidden()
            .child(self.sidebar.clone())
    }
    
    fn render_main_content(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .bg(colors::window_alter_bg())
            .child(self.title_bar.clone())
            .child(self.render_page_content())
    }
    
    fn render_page_content(&self) -> impl IntoElement {
        div()
            .flex_1()
            .overflow_hidden()
            .child(
                match self.current_page {
                    Page::GetStarted => div().child(self.get_started.clone()),
                    Page::Connections => div().child(self.connections.clone()),
                    Page::NetworkLogs => div().child(self.network_logs.clone()),
                    Page::Settings => div().child(self.settings.clone()),
                }
            )
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h_full()
            .bg(colors::window_bg())
            .text_color(colors::window_fg())
            .child(self.render_sidebar())
            .child(self.render_main_content())
    }
}
