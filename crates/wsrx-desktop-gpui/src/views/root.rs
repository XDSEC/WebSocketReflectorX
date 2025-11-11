// Root view - Main application window
use gpui::{Context, Entity, Render, Window, div, prelude::*};

use super::{ConnectionsView, GetStartedView, NetworkLogsView, SettingsView, SidebarView};
use crate::{
    components::title_bar::TitleBar, models::{LogEntry, app_state::PageId}, styles::colors,
};

pub struct RootView {
    /// Current active page (string-based: "home", "logs", "settings", "default-scope", or scope.host)
    current_page: PageId,

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
        let current_page = "home".to_string();
        let window_handle = window.window_handle();

        let root = Self {
            current_page: current_page.clone(),
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

    pub fn set_page(&mut self, page: PageId, cx: &mut Context<Self>) {
        self.current_page = page;
        cx.notify(); // Trigger re-render
    }

    pub fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.show_sidebar = !self.show_sidebar;
        cx.notify();
    }

    /// Add a log entry to the network logs view
    pub fn add_log(&mut self, log_entry: LogEntry, cx: &mut Context<Self>) {
        self.network_logs.update(cx, |logs_view, cx| {
            logs_view.add_log(log_entry);
            cx.notify();
        });
    }

    fn render_sidebar(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .when(self.show_sidebar, |div| div.w_64())
            .when(!self.show_sidebar, |div| div.w_0())
            .h_full() // Full height of window
            .overflow_hidden()
            .child(self.sidebar.clone())
    }

    fn render_main_content(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full() // Full height of window
            .bg(colors::window_alter_bg())
            .child(self.title_bar.clone())
            .child(self.render_page_content())
    }

    fn render_page_content(&self) -> impl IntoElement {
        let page = self.current_page.as_str();
        div()
            .id("page-content")
            .flex_1()
            .overflow_y_scroll() // Allow vertical scrolling when content overflows
            .child(match page {
                "home" => div().h_full().child(self.get_started.clone()),
                "logs" => div().h_full().child(self.network_logs.clone()),
                "settings" => div().h_full().child(self.settings.clone()),
                _ => div().h_full().child(self.connections.clone()), // Scope pages show connections
            })
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
