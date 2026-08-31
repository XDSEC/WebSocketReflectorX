use std::{ops::Range, sync::Arc};

use woocraft::gpui::{Context, HighlightStyle, Hsla, IntoElement, Styled, Window, px, rgb};
use woocraft::{
    CodeEditor, EditorActionSink, EditorBackend, EditorBackendCapabilities,
    EditorBackendEditRequest, EditorBackendEditResult, EditorContextMenuProvider, EditorEditError,
    EditorHighlighter, EditorHighlighterProvider, EditorSnapshot, EditorTextChange, HighlightTheme,
    Rope, RopeEditorSnapshot, RopeExt, ScrollbarPreview, ScrollbarPreviewLine,
};

use crate::{models::LogEntry, ui::RootView};

/// Renders the network logs page as a readonly code editor.
pub(crate) fn render_network_logs(
    _window: &mut Window, _cx: &mut Context<RootView>, root: &mut RootView,
) -> impl IntoElement {
    CodeEditor::new(&root.logs_editor)
        .h_full()
        .w_full()
        .appearance(false)
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
            out.push_str(&format!(
                "{timestamp}  {:>5} {}\n",
                log.level, log.fields.message
            ));
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
    ("TRACE", 0x9A6FCE),
    ("DEBUG", 0x4F9CF6),
    ("INFO", 0x3FB950),
    ("WARN", 0xD29922),
    ("ERROR", 0xF85149),
];

/// Severity-scaled preview opacity: errors stand out on the minimap while
/// debug/trace lines stay subtle.
fn preview_color(level: &str) -> Option<Hsla> {
    let (color, alpha) = match level {
        "TRACE" => (0x9A6FCE, 0.4),
        "DEBUG" => (0x4F9CF6, 0.5),
        "INFO" => (0x3FB950, 0.6),
        "WARN" => (0xD29922, 0.85),
        "ERROR" => (0xF85149, 1.0),
        _ => return None,
    };
    let color: Hsla = rgb(color).into();
    Some(color.opacity(alpha))
}

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

    /// Preview strip for the scrollbar: one 1px line per log row inside the
    /// requested window, colored by the row's level. Only rows within the
    /// window are inspected — the parsing budget is bounded by the window
    /// size, which the editor caps at the track height in pixels.
    fn scrollbar_preview(&self, window: Range<u64>) -> ScrollbarPreview {
        let start = (window.start as usize).min(self.text.lines_len());
        let end = (window.end as usize).min(self.text.lines_len());
        if end <= start {
            return ScrollbarPreview::default();
        }
        let mut lines = Vec::with_capacity(end - start);
        for row in start..end {
            let line = self.text.slice_line(row);
            let color = line
                .as_str()
                .and_then(find_level)
                .and_then(|(_, level)| preview_color(level))
                .unwrap_or_default();
            lines.push(ScrollbarPreviewLine::new(color));
        }
        // A narrow column near the inner edge of the track, so future status
        // strips (git, LSP diagnostics…) can be drawn alongside.
        ScrollbarPreview {
            left: px(2.0),
            width: px(6.0),
            lines,
        }
    }
}

impl EditorActionSink for LogBackend {}

impl EditorContextMenuProvider for LogBackend {}

impl EditorHighlighterProvider for LogBackend {
    fn create_highlighter(&self) -> Option<Box<dyn EditorHighlighter>> {
        Some(Box::new(LogHighlighter))
    }
}

/// Highlights log lines with the `tracing-subscriber` console styling:
/// timestamp and target are dimmed, the level token keeps its level color.
struct LogHighlighter;

impl EditorHighlighter for LogHighlighter {
    fn sync(&mut self, _snapshot: &dyn EditorSnapshot, _change: Option<&EditorTextChange>) {}

    fn highlight_range(
        &self, snapshot: &dyn EditorSnapshot, range: Range<u64>, theme: &HighlightTheme,
    ) -> Vec<(Range<u64>, HighlightStyle)> {
        let Some(text) = snapshot.text_for_range(range.clone()) else {
            return Vec::new();
        };
        let visible_len = (range.end - range.start) as usize;

        // `tracing-subscriber` renders the timestamp and the target dimmed, so
        // derive the dim color from the editor foreground blended toward the
        // editor background.
        let fg = theme
            .style
            .editor_foreground
            .unwrap_or_else(|| rgb(0xCCCCCC).into());
        let bg = theme
            .style
            .editor_background
            .unwrap_or_else(|| rgb(0x0D1117).into());
        let dim = HighlightStyle {
            color: Some(fg.blend(bg.opacity(0.5))),
            ..Default::default()
        };

        // Collect styled segments, offsets relative to `range.start`.
        let mut segments: Vec<(usize, usize, HighlightStyle)> = Vec::new();
        let mut line_start = 0usize;
        for line in text.split('\n') {
            if let Some(spans) = spans_for_line(line)
                && let Some(color) = level_color(&line[spans.level.clone()])
            {
                // Timestamp: dimmed, like `tracing-subscriber`.
                if !spans.timestamp.is_empty() {
                    segments.push((
                        line_start + spans.timestamp.start,
                        line_start + spans.timestamp.end,
                        dim,
                    ));
                }

                // Level token: level color.
                segments.push((
                    line_start + spans.level.start,
                    line_start + spans.level.end,
                    HighlightStyle {
                        color: Some(color),
                        ..Default::default()
                    },
                ));

                // Target and its trailing colon: dimmed, like
                // `tracing-subscriber`. Module paths use `::`, so the first
                // colon-space pair after the level unambiguously ends it.
                if let Some(module) = spans.module {
                    segments.push((line_start + module.start, line_start + module.end, dim));
                }
            }
            line_start += line.len() + 1;
        }

        // The editor treats highlight runs as a contiguous partition of the
        // visible text, so fill the gaps between styled segments with the
        // default style.
        let mut highlights = Vec::new();
        let mut cursor = 0usize;
        for (start, end, style) in segments {
            if start > cursor && cursor < visible_len {
                highlights.push((cursor as u64..start as u64, HighlightStyle::default()));
            }
            if end > cursor {
                highlights.push((start as u64..end as u64, style));
                cursor = end;
            }
        }
        if cursor < visible_len {
            highlights.push((cursor as u64..visible_len as u64, HighlightStyle::default()));
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

/// Parsed spans of one log line formatted by [`format_logs`]: the timestamp,
/// the level token, and the module path including its trailing colon (when
/// present).
/// Parsed spans of one log line formatted by [`format_logs`]: the timestamp,
/// the level token, and the module path including its trailing colon (when
/// present). All ranges are byte offsets relative to the line start.
struct LineSpans {
    timestamp: Range<usize>,
    level: Range<usize>,
    module: Option<Range<usize>>,
}

fn spans_for_line(line: &str) -> Option<LineSpans> {
    let (offset, level) = find_level(line)?;
    let ts_end = line[..offset].trim_end().len();
    let level_range = offset..offset + level.len();
    let module_start = offset + level.len() + 1;
    let module = if module_start < line.len() {
        line[module_start..]
            .find(": ")
            .map(|rel| module_start..module_start + rel + 1)
    } else {
        None
    };
    Some(LineSpans {
        timestamp: 0..ts_end,
        level: level_range,
        module,
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_colors_cover_all_levels() {
        for (level, _) in LEVEL_COLORS {
            assert!(level_color(level).is_some(), "{level} has no color");
        }
    }

    #[test]
    fn find_level_ignores_level_words_in_messages_and_targets() {
        let line =
            "2026-08-22T14:07:00.123456Z  INFO wsrx_desktop::daemon: error in handler warning";
        let (offset, level) = find_level(line).unwrap();
        assert_eq!(level, "INFO");
        assert_eq!(&line[offset..offset + level.len()], "INFO");
        // "warning" embeds the WARN token without word boundaries, so it is
        // not mistaken for the real level.
        assert_eq!(find_word(line, "WARN"), None);
    }

    #[test]
    fn spans_for_line_parses_timestamp_level_module() {
        let line = "2026-08-22T14:07:00.123456Z  INFO wsrx_desktop::daemon: something happened";
        let spans = spans_for_line(line).unwrap();
        assert_eq!(&line[spans.timestamp], "2026-08-22T14:07:00.123456Z");
        assert_eq!(&line[spans.level], "INFO");
        let module = spans.module.unwrap();
        assert_eq!(&line[module], "wsrx_desktop::daemon:");
    }

    #[test]
    fn spans_for_line_keeps_message_colon_space_untouched() {
        // `::` inside the module path and a later `: ` inside the message
        // must not confuse the module boundary.
        let line =
            "2026-08-22T14:07:00.123456Z ERROR wsrx_desktop::daemon::workers: read timeout: retry";
        let spans = spans_for_line(line).unwrap();
        assert_eq!(&line[spans.level], "ERROR");
        let module = spans.module.unwrap();
        assert_eq!(&line[module], "wsrx_desktop::daemon::workers:");
    }

    #[test]
    fn spans_for_line_handles_missing_target() {
        let line = "2026-08-22T14:07:00.123456Z  INFO plain message without target";
        let spans = spans_for_line(line).unwrap();
        assert_eq!(&line[spans.timestamp], "2026-08-22T14:07:00.123456Z");
        assert_eq!(&line[spans.level], "INFO");
        assert_eq!(spans.module, None);
    }

    #[test]
    fn preview_returns_one_line_per_window_row() {
        let text = "2026-08-22T14:07:00Z  INFO a: one\n".to_owned()
            + "2026-08-22T14:07:01Z  WARN b: two\n"
            + "2026-08-22T14:07:02Z ERROR c: three\n";
        let backend = LogBackend::new(text);
        let preview = backend.scrollbar_preview(0..3);
        let lines = &preview.lines;
        assert_eq!(lines.len(), 3, "one preview line per requested row");
        // Errors are fully opaque, info lines stay subtle.
        assert_eq!(lines[2].color.a, 1.0);
        assert!(lines[0].color.a < 0.7);
        assert!(backend.scrollbar_preview(0..0).lines.is_empty());
    }

    #[test]
    fn preview_window_is_clamped_to_the_document() {
        let text = "2026-08-22T14:07:00Z  INFO a: one\n2026-08-22T14:07:01Z ERROR b: two\n";
        let backend = LogBackend::new(text);
        // Window far beyond the document must not produce phantom rows.
        let preview = backend.scrollbar_preview(0..1000);
        let lines = &preview.lines;
        assert_eq!(lines.len(), 3, "2 log lines + trailing empty row");
        assert!(lines[2].color.a <= 0.0, "empty row previews as transparent");
    }

    #[test]
    fn preview_respects_the_parsing_budget() {
        // 13 lines but a window of 5: only the requested rows are inspected.
        let text = (0..13)
            .map(|i| format!("2026-08-22T14:07:0{i}Z  INFO line {i}: hello\n"))
            .collect::<String>();
        let backend = LogBackend::new(text);
        let preview = backend.scrollbar_preview(3..8);
        let lines = &preview.lines;
        assert_eq!(lines.len(), 5);
        assert!(lines.iter().all(|line| line.color.a > 0.0));
        // A window past the document yields nothing.
        assert!(backend.scrollbar_preview(20..30).lines.is_empty());
    }

    #[test]
    fn preview_follows_the_window_position() {
        // The strip is positional: window line 0 always refers to the row the
        // window starts at, so scrolling moves the preview content.
        let text =
            "2026-08-22T14:07:00Z ERROR first\n".to_owned() + "2026-08-22T14:07:01Z  INFO second\n";
        let backend = LogBackend::new(text);
        let preview = backend.scrollbar_preview(1..2);
        let lines = &preview.lines;
        assert_eq!(lines.len(), 1);
        // Row 1 is the INFO line, not the ERROR line.
        assert!(lines[0].color.a < 1.0);
    }

    #[test]
    fn format_logs_roundtrips_through_spans() {
        let logs = vec![
            LogEntry {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-08-22T14:07:00.123456Z")
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                level: "WARN".into(),
                target: "wsrx_desktop::daemon".into(),
                fields: crate::models::LogEntryFields {
                    message: "retrying after timeout".into(),
                },
            },
            LogEntry {
                timestamp: chrono::DateTime::parse_from_rfc3339("2026-08-22T14:07:01Z")
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                level: "INFO".into(),
                target: String::new(),
                fields: crate::models::LogEntryFields {
                    message: "no target here".into(),
                },
            },
        ];
        let text = format_logs(&logs);
        let first_len = text.find('\n').unwrap() + 1;
        let spans = spans_for_line(&text[..first_len - 1]).unwrap();
        assert_eq!(
            &text[..first_len - 1][spans.timestamp],
            "2026-08-22T14:07:00.123456Z"
        );
        assert_eq!(&text[spans.level], "WARN");
        assert_eq!(&text[spans.module.unwrap()], "wsrx_desktop::daemon:");

        let second = &text[first_len..];
        let spans = spans_for_line(second).unwrap();
        assert_eq!(&second[spans.level], "INFO");
        assert_eq!(spans.module, None);
        let backend = LogBackend::new(text);
        let preview = backend.scrollbar_preview(0..10);
        let lines = &preview.lines;
        assert_eq!(lines.len(), 3, "2 entries + trailing empty row");
        assert!(lines[0].color.a > 0.0);
        assert!(lines[2].color.a <= 0.0);
    }

    #[test]
    fn preview_uses_a_narrow_strip_geometry() {
        // The strip must not claim the full track: future status markers
        // (git, LSP…) need room to sit alongside.
        let backend = LogBackend::new("2026-08-22T14:07:00Z  INFO a: one\n");
        let preview = backend.scrollbar_preview(0..10);
        assert!(!preview.lines.is_empty());
        assert_eq!(f32::from(preview.width), 6.0);
        assert_eq!(f32::from(preview.left), 2.0);
        assert!(
            f32::from(preview.width) < 32.0,
            "strip must not span the whole track"
        );
    }
}
