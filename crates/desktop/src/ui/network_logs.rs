use std::{ops::Range, sync::Arc};

use gpui::{Context, HighlightStyle, Hsla, IntoElement, Styled, Window, rgb};
use woocraft::{
    CodeEditor, EditorActionSink, EditorBackend, EditorBackendCapabilities, EditorBackendEditRequest,
    EditorBackendEditResult, EditorContextMenuProvider, EditorEditError, EditorHighlighter,
    EditorHighlighterProvider, EditorSnapshot, EditorTextChange, HighlightTheme, Rope,
    RopeEditorSnapshot, RopeExt,
};

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

/// Formats log entries into a single plain-text document matching the default
/// `tracing-subscriber` console output:
///
/// `2026-08-22T14:07:00.123456Z  INFO wsrx_desktop::daemon: message`
pub(crate) fn format_logs(logs: &[LogEntry]) -> String {
    let mut out = String::with_capacity(logs.len() * 96);
    for log in logs {
        let timestamp = log.timestamp.format("%Y-%m-%dT%H:%M:%S%.6fZ");
        if log.target.is_empty() {
            out.push_str(&format!("{timestamp}  {:>5} {}\n", log.level, log.fields.message));
        } else {
            out.push_str(&format!(
                "{timestamp}  {:>5} {}: {}\n",
                log.level, log.target, log.fields.message
            ));
        }
    }
    out
}

/// The default `tracing-subscriber` console level colors.
const LEVEL_COLORS: [(&str, u32); 5] = [
    ("TRACE", 0x9a6fce),
    ("DEBUG", 0x4f9cf6),
    ("INFO", 0x3fb950),
    ("WARN", 0xd29922),
    ("ERROR", 0xf85149),
];

fn level_color(level: &str) -> Option<Hsla> {
    LEVEL_COLORS
        .iter()
        .find(|(name, _)| *name == level)
        .map(|(_, color)| rgb(*color).into())
}

/// A read-only editor backend holding a plain rope of log text.
pub(crate) struct LogBackend {
    revision: u64,
    text: Rope,
}

impl LogBackend {
    pub(crate) fn new(text: impl AsRef<str>) -> Self {
        Self {
            revision: 0,
            text: Rope::from(text.as_ref()),
        }
    }
}

impl EditorBackend for LogBackend {
    fn revision(&self) -> u64 {
        self.revision
    }

    fn capabilities(&self) -> EditorBackendCapabilities {
        EditorBackendCapabilities::default()
    }

    fn snapshot(&self) -> Arc<dyn EditorSnapshot> {
        Arc::new(RopeEditorSnapshot::new(self.revision, self.text.clone()))
    }

    fn apply_edit(
        &mut self, request: EditorBackendEditRequest,
    ) -> Result<EditorBackendEditResult, EditorEditError> {
        let start = (request.range.start as usize).min(self.text.len());
        let end = (request.range.end as usize).min(self.text.len());
        self.text.replace(start..end, &request.new_text);
        self.revision = self.revision.wrapping_add(1);
        Ok(EditorBackendEditResult {
            accepted: true,
            selection: None,
            cursor: Some((start + request.new_text.len()) as u64),
        })
    }
}

impl EditorActionSink for LogBackend {}

impl EditorContextMenuProvider for LogBackend {}

impl EditorHighlighterProvider for LogBackend {
    fn create_highlighter(&self) -> Option<Box<dyn EditorHighlighter>> {
        Some(Box::new(LogHighlighter))
    }
}

/// Highlights the level token of every log line with the
/// `tracing-subscriber` console colors.
struct LogHighlighter;

impl EditorHighlighter for LogHighlighter {
    fn sync(&mut self, _snapshot: &dyn EditorSnapshot, _change: Option<&EditorTextChange>) {}

    fn highlight_range(
        &self, snapshot: &dyn EditorSnapshot, range: Range<u64>, _theme: &HighlightTheme,
    ) -> Vec<(Range<u64>, HighlightStyle)> {
        let Some(text) = snapshot.text_for_range(range.clone()) else {
            return Vec::new();
        };

        let mut highlights = Vec::new();
        let mut line_start = 0u64;
        for line in text.split('\n') {
            if let Some((offset, level)) = find_level(line) {
                let start = range.start + line_start + offset as u64;
                let end = start + level.len() as u64;
                if let Some(color) = level_color(level) {
                    highlights.push((
                        start..end,
                        HighlightStyle {
                            color: Some(color),
                            ..Default::default()
                        },
                    ));
                }
            }
            line_start += line.len() as u64 + 1;
        }
        highlights
    }
}

/// Finds the earliest level token (one of TRACE/DEBUG/INFO/WARN/ERROR) in a
/// log line, returning its byte offset and the matched level.
fn find_level(line: &str) -> Option<(usize, &'static str)> {
    let mut best: Option<(usize, &'static str)> = None;
    for (level, _) in LEVEL_COLORS {
        if let Some(pos) = find_word(line, level)
            && best.is_none_or(|(bp, _)| pos < bp)
        {
            best = Some((pos, level));
        }
    }
    best
}

/// Finds the first word-boundary occurrence of `word` in `line`.
fn find_word(line: &str, word: &str) -> Option<usize> {
    let mut search = 0;
    while let Some(rel) = line[search..].find(word) {
        let abs = search + rel;
        let before_ok = abs == 0 || !line.as_bytes()[abs - 1].is_ascii_alphanumeric();
        let end = abs + word.len();
        let after_ok = end >= line.len() || !line.as_bytes()[end].is_ascii_alphanumeric();
        if before_ok && after_ok {
            return Some(abs);
        }
        search = abs + word.len();
    }
    None
}
