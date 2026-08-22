use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};
use woocraft::{ActiveTheme, Theme, ThemeMode, h_flex, v_flex, window_border};

use super::{connections, get_started, network_logs, settings, sidebar, title_bar};
use crate::{
    daemon::{self, ServerState, UiEvent},
    i18n, launcher,
    models::{InstanceData, LogEntry, ScopeData, WsrxDesktopConfig},
};

/// The page currently displayed in the main content area.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Home,
    Logs,
    Settings,
    /// A scope page; `"default-scope"` is the built-in manual scope.
    Scope(String),
}

/// The root view of the main window: title bar, sidebar and page content.
pub struct RootView {
    state: ServerState,
    page: Page,

    // --- snapshots updated from shared state via `UiEvent::Refresh` ---
    scopes: Vec<ScopeData>,
    instances: Vec<InstanceData>,
    logs: Vec<LogEntry>,
    api_port: u16,
    online: bool,
    has_updates: bool,

    // --- settings ---
    pub(crate) settings: WsrxDesktopConfig,

    // --- get started page ---
    interfaces: Vec<String>,
    selected_interface: String,
    pub(crate) remote_input: gpui::Entity<woocraft::InputState>,
    pub(crate) port_input: gpui::Entity<woocraft::InputState>,
    cursor_visible: bool,

    // --- network logs page ---
    pub(crate) logs_editor: gpui::Entity<woocraft::EditorState>,

    // --- settings page ---
    pub(crate) info_editor: gpui::Entity<woocraft::EditorState>,
    info: String,
    version: String,
}

impl RootView {
    pub fn view(window: &mut Window, cx: &mut App, state: ServerState) -> Entity<Self> {
        // Apply persisted settings before the first frame.
        let settings = state.settings.blocking_read().clone();
        match settings.theme.as_str() {
            "light" => Theme::set_mode(ThemeMode::Light, cx),
            _ => Theme::set_mode(ThemeMode::Dark, cx),
        }
        i18n::set_locale(&settings.language);

        let logs_editor = cx.new(|cx| {
            woocraft::EditorState::new(window, cx)
                .code_editor("text")
                .backend(super::network_logs::LogBackend::new(""))
                .read_only(true)
        });

        let info = daemon::system_info();
        let info_editor = cx.new(|cx| {
            woocraft::EditorState::new(window, cx)
                .code_editor("text")
                .read_only(true)
                .default_value(info.clone())
        });

        cx.new(|cx| {
            let remote_input = cx.new(|cx| {
                woocraft::InputState::new(cx).placeholder(i18n::t("[ws|wss]://address..."))
            });
            let port_input = cx.new(|cx| {
                woocraft::InputState::new(cx)
                    .placeholder(i18n::t("Port..."))
                    .default_value("0")
            });

            let mut view = Self {
                state: state.clone(),
                page: Page::Home,
                scopes: vec![],
                instances: vec![],
                logs: vec![],
                api_port: 0,
                online: false,
                has_updates: false,
                settings: settings.clone(),
                interfaces: default_interfaces(),
                selected_interface: "127.0.0.1".to_string(),
                remote_input,
                port_input,
                cursor_visible: true,
                logs_editor,
                info_editor,
                info,
                version: env!("CARGO_PKG_VERSION").to_string(),
            };
            view.refresh_from_state();
            view
        })
    }

    /// Re-reads instances / scopes / settings from shared state.
    pub(crate) fn refresh_from_state(&mut self) {
        self.scopes = self.state.scopes.blocking_read().clone();
        self.instances = self
            .state
            .instances
            .blocking_read()
            .iter()
            .map(Into::into)
            .collect();
        self.settings = self.state.settings.blocking_read().clone();
    }

    /// Applies a background event pushed through the event channel.
    pub fn handle_event(&mut self, event: UiEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            UiEvent::Online { port } => {
                self.api_port = port;
                self.online = true;
            }
            UiEvent::Refresh => self.refresh_from_state(),
            UiEvent::Log(entry) => self.handle_logs(vec![entry], window, cx),
            UiEvent::Logs(entries) => self.handle_logs(entries, window, cx),
            UiEvent::HasUpdates(value) => self.has_updates = value,
            UiEvent::Popup => {
                window.activate_window();
            }
            // Handled by the app-level event loop in `main`.
            UiEvent::Quit => {}
            UiEvent::CursorTick => {
                if matches!(self.page, Page::Home) {
                    self.cursor_visible = !self.cursor_visible;
                }
            }
        }
        cx.notify();
    }

    /// Appends log entries and pushes the accumulated text into the readonly
    /// editor in a single coalesced update. `set_value` is a no-op while
    /// `read_only` is set, so it is flipped around the update.
    pub fn handle_logs(
        &mut self, entries: Vec<LogEntry>, window: &mut Window, cx: &mut Context<Self>,
    ) {
        self.logs.extend(entries);
        const MAX_LOGS: usize = 1000;
        if self.logs.len() > MAX_LOGS {
            let excess = self.logs.len() - MAX_LOGS;
            self.logs.drain(..excess);
        }

        let text = super::network_logs::format_logs(&self.logs);
        let editor = self.logs_editor.clone();
        cx.spawn_in(window, async move |_, cx| {
            let _ = editor.update_in(cx, |state, window, cx| {
                state.set_read_only(false, window, cx);
                state.set_value(text, window, cx);
                state.set_read_only(true, window, cx);
            });
        })
        .detach();

        cx.notify();
    }

    /// Returns the scope data for the currently displayed page, when the page
    /// is a scope page.
    pub(crate) fn scope_for_page(&self) -> Option<ScopeData> {
        match &self.page {
            Page::Scope(host) if host == "default-scope" => Some(default_scope()),
            Page::Scope(host) => self.scopes.iter().find(|s| &s.host == host).cloned(),
            _ => None,
        }
    }

    /// Instances belonging to the currently displayed scope page.
    pub(crate) fn scoped_instances(&self) -> Vec<InstanceData> {
        match &self.page {
            Page::Scope(host) => self
                .instances
                .iter()
                .filter(|i| &i.scope_host == host)
                .cloned()
                .collect(),
            _ => vec![],
        }
    }

    pub(crate) fn change_page(&mut self, page: Page, cx: &mut Context<Self>) {
        self.page = page;
        self.refresh_from_state();
        cx.notify();
    }

    pub(crate) fn state(&self) -> &ServerState {
        &self.state
    }

    pub(crate) fn settings(&self) -> &WsrxDesktopConfig {
        &self.settings
    }

    pub(crate) fn interfaces(&self) -> &[String] {
        &self.interfaces
    }

    pub(crate) fn selected_interface(&self) -> &str {
        &self.selected_interface
    }

    pub(crate) fn remote_input(&self) -> &gpui::Entity<woocraft::InputState> {
        &self.remote_input
    }

    pub(crate) fn port_input(&self) -> &gpui::Entity<woocraft::InputState> {
        &self.port_input
    }

    pub(crate) fn has_updates(&self) -> bool {
        self.has_updates
    }

    pub(crate) fn info(&self) -> &str {
        &self.info
    }

    pub(crate) fn version(&self) -> &str {
        &self.version
    }

    pub(crate) fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub(crate) fn refresh_interfaces(&mut self) {
        self.interfaces = default_interfaces();
        if !self.interfaces.contains(&self.selected_interface) {
            self.selected_interface = "127.0.0.1".to_string();
        }
    }

    pub(crate) fn select_interface(&mut self, interface: String) {
        self.selected_interface = interface;
    }

    pub(crate) fn open_link(url: &str) {
        open::that_detached(url).unwrap_or_else(|_| {
            tracing::error!("Failed to open link {url} in default browser.");
        });
    }

    pub(crate) fn open_logs_dir() {
        let log_dir = launcher::project_dirs().data_local_dir().join("logs");
        open::that_detached(&log_dir).unwrap_or_else(|_| {
            tracing::error!("Failed to open logs directory.");
        });
    }

    pub(crate) fn copy_to_clipboard(cx: &mut App, text: &str) {
        cx.write_to_clipboard(gpui::ClipboardItem::new_string(text.to_string()));
    }
}

fn default_interfaces() -> Vec<String> {
    let mut interfaces = vec!["127.0.0.1".to_string()];
    if let Ok(netifas) = local_ip_address::list_afinet_netifas() {
        for (_, addr) in netifas {
            if addr.is_ipv6() {
                continue;
            }
            let ip = addr.to_string();
            if !interfaces.contains(&ip) {
                interfaces.push(ip);
            }
        }
    }
    if !interfaces.contains(&"0.0.0.0".to_string()) {
        interfaces.push("0.0.0.0".to_string());
    }
    interfaces
}

pub(crate) fn default_scope() -> ScopeData {
    ScopeData {
        host: "default-scope".to_string(),
        name: "Default Scope".to_string(),
        state: "allowed".to_string(),
        features: crate::models::FeatureFlags::Basic,
        settings: Default::default(),
    }
}

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity();
        let background = cx.theme().background;
        let border = cx.theme().border;
        let state = self.state.clone();

        window_border().child(
            v_flex()
                .size_full()
                .min_h_0()
                .child(title_bar::render_title_bar(window, cx, &this, &state))
                // Divider between the title bar and the content area.
                .child(div().h_px().bg(border))
                .child(
                    h_flex()
                        .size_full()
                        .min_h_0()
                        .child(
                            h_flex()
                                .h_full()
                                .flex_shrink_0()
                                .child(sidebar::render_sidebar(
                                    window,
                                    cx,
                                    &this,
                                    &self.page,
                                    &self.scopes,
                                    self.online,
                                    self.api_port,
                                ))
                                // Divider between the sidebar and the content.
                                .child(div().w_px().h_full().bg(border)),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .min_h_0()
                                .size_full()
                                .bg(background)
                                .child(self.render_page(window, cx)),
                        ),
                ),
        )
    }
}

impl RootView {
    fn render_page(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match &self.page {
            Page::Home => get_started::render_get_started(window, cx, self).into_any_element(),
            Page::Logs => network_logs::render_network_logs(window, cx, self).into_any_element(),
            Page::Settings => settings::render_settings(window, cx, self).into_any_element(),
            Page::Scope(host) => {
                let host = host.clone();
                connections::render_connections(window, cx, self, &host).into_any_element()
            }
        }
    }
}
