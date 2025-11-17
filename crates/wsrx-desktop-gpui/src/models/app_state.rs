// Application State - Global application state management
// Architecture: Scope-centric with dynamic page navigation

use gpui::SharedString;

use super::{Instance, LogEntry, Scope, Settings};

/// Current page identifier
/// - "home": Get Started page (tunnel creation)
/// - "logs": Network Logs page
/// - "settings": Settings page
/// - "default-scope": User's manual tunnels
/// - <domain>: External scope (e.g., "gzctf.example.com")
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Page {
    #[default]
    Home,
    Logs,
    Settings,
    DefaultScope,
    Scope(SharedString),
}

impl Page {
    pub fn as_page_id(&self) -> &str {
        match self {
            Page::Home => "home",
            Page::Logs => "logs",
            Page::Settings => "settings",
            Page::DefaultScope => "default",
            Page::Scope(scope) => scope,
        }
    }
}

/// Global UI state
pub struct UiState {
    /// Current active page/scope
    pub page: Page,

    /// Current scope for connections page
    pub current_scope: Option<Scope>,

    /// Whether sidebar is visible
    pub show_sidebar: bool,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            page: Page::Home,
            current_scope: None,
            show_sidebar: true,
        }
    }

    /// Navigate to a page
    pub fn navigate_to(&mut self, page: Page) {
        self.page = page;
    }

    /// Change to a specific scope (for connections page)
    pub fn change_scope(&mut self, scope: Scope) {
        self.page = Page::Scope(scope.host.clone());
        self.current_scope = Some(scope.clone());
    }

    /// Check if current page is a scope (connections page)
    pub fn is_scope_page(&self) -> bool {
        matches!(self.page, Page::Scope(_) | Page::DefaultScope)
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global application state
/// Holds the main application data shared across views
pub struct AppState {
    /// UI state (page navigation, current scope)
    pub ui_state: UiState,

    /// All tunnel instances across all scopes
    pub instances: Vec<Instance>,

    /// All scopes (external domains)
    pub scopes: Vec<Scope>,

    /// Application settings
    pub settings: Settings,

    /// Recent log entries from tracing
    pub logs: Vec<LogEntry>,

    /// Maximum number of logs to keep in memory
    pub max_logs: usize,
}

impl AppState {
    /// Create a new AppState with default values
    pub fn new() -> Self {
        Self {
            ui_state: UiState::new(),
            instances: Vec::new(),
            scopes: Vec::new(),
            settings: Settings::default(),
            logs: Vec::new(),
            max_logs: 1000,
        }
    }

    /// Add a log entry, removing oldest if over capacity
    pub fn add_log(&mut self, entry: LogEntry) {
        if self.logs.len() >= self.max_logs {
            self.logs.remove(0);
        }
        self.logs.push(entry);
    }

    /// Clear all logs
    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    /// Get instances for current scope
    pub fn scoped_instances(&self) -> Vec<Instance> {
        if let Some(ref scope) = self.ui_state.current_scope {
            self.instances
                .iter()
                .filter(|i| i.scope_host == scope.host)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Add an instance to a scope
    pub fn add_instance(&mut self, instance: Instance) {
        self.instances.push(instance);
    }

    /// Remove an instance by local address
    pub fn remove_instance(&mut self, local: &str) {
        self.instances.retain(|i| i.local != local);
    }

    /// Add or update a scope
    pub fn upsert_scope(&mut self, scope: Scope) {
        if let Some(pos) = self.scopes.iter().position(|s| s.host == scope.host) {
            self.scopes[pos] = scope;
        } else {
            self.scopes.push(scope);
        }
    }

    /// Remove a scope by host
    pub fn remove_scope(&mut self, host: &str) {
        self.scopes.retain(|s| s.host != host);
        // Also remove all instances for this scope
        self.instances.retain(|i| i.scope_host != host);
    }

    /// Get a scope by host
    pub fn get_scope(&self, host: &str) -> Option<&Scope> {
        self.scopes.iter().find(|s| s.host == host)
    }

    /// Allow (accept) a pending scope
    pub fn allow_scope(&mut self, host: &str) {
        if let Some(scope) = self.scopes.iter_mut().find(|s| s.host == host) {
            scope.state = "allowed".to_string();
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
