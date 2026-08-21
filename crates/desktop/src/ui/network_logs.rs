use gpui::{Context, IntoElement, Styled, Window};
use woocraft::CodeEditor;

use crate::{models::LogEntry, ui::RootView};

/// Renders the network logs page as a readonly code editor.
pub(crate) fn render_network_logs(
    _window: &mut Window,
    _cx: &mut Context<RootView>,
    root: &mut RootView,
) -> impl IntoElement {
    CodeEditor::new(&root.logs_editor)
        .h_full()
        .w_full()
        .appearance(false)
        .bordered(false)
        .focus_bordered(false)
}

/// Formats log entries into a single plain-text document for the editor.
pub(crate) fn format_logs(logs: &[LogEntry]) -> String {
    let mut out = String::with_capacity(logs.len() * 96);
    for log in logs {
        let timestamp = log
            .timestamp
            .with_timezone(&chrono::Local)
            .format("%Y-%m-%d %H:%M:%S");
        out.push_str(&format!(
            "[{timestamp}] {:>5} {}  {}\n",
            log.level, log.target, log.fields.message
        ));
    }
    out
}
