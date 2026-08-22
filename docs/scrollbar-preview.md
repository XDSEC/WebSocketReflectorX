# Scrollbar Preview (Minimap) — Design Plan

> Target feel: VS Code's current minimap.
> Companion validation: [`scrollbar-preview-simulate.py`](./scrollbar-preview-simulate.py)

## 1. Goal

The scrollbar track acts as a minimap of the log document. Behavior depends on
how many rows the document has relative to the track height:

| Case | Condition | Preview behavior |
| --- | --- | --- |
| 1 — few logs | `total <= viewport` | Full document preview fills the whole track; no scrolling possible, no thumb |
| 2 — medium | `viewport < total <= H` | Full document preview fills the whole track; the thumb correctly marks the visible window |
| 3 — many logs | `total > H` | A `H`-row window of the document is previewed at 1px/row; the window follows the scroll (content scrolls up while the thumb scrolls down); the thumb stays the global scrollbar |

`H` = track height in pixels = preview capacity. One preview row = 1px only
when the document exceeds the capacity; otherwise rows are scaled up to fill
the whole track.

## 2. Model & formulas

Notation (all row counts in *logical* rows; the editor translates the
viewport's top display row through soft-wrap before computing the window):

- `H` — track height (px)
- `total` — document row count
- `viewport` — visible row count
- `top_row` — first visible row
- `max_top = total - viewport` — last scrollable position
- `thumb_h = clamp(H * viewport / total, 8px, H)` — thumb height
- `travel = H - thumb_h`
- `indicator_y = top_row / max_top * travel` — thumb top edge y (px)

### Case 1 & 2 (`total <= H`)

```
window  = [0, total)                      # whole document
count   = total
scale   = H / total                       # > 1, stretch to fill the track
```

Each preview row `i` is drawn at `y = round(i*scale)` with height
`round((i+1)*scale) - y` (cumulative rounding tiles the track with no gaps).

**Alignment property**: in this regime the thumb's travel mapping is exactly
`indicator_y = top_row * H / total = top_row * scale`, so the thumb overlays
precisely the visible rows on the stretched preview. This is what makes
"indicator 正确显示视图位置" exact, not approximate.

### Case 3 (`total > H`)

```
anchor       = logical row of the viewport top        # row at the thumb's top edge
indicator_y  = thumb top edge y (px)                   # 1px == 1 row here
start        = max(0, anchor - indicator_y)            # indicator_y rows of context above
window       = [start, start + H)                      # H rows total
count        = min(H, total - start)
scale        = 1                                        # 1px per row
```

The thumb's top edge always shows `anchor` (the viewport top row), with
`indicator_y` rows of already-scrolled context above and `H - indicator_y`
rows below — matching "向上扩展 indicator_y 行，向下扩展 H - indicator_y 行".

**Scroll property**: between clamp points,
`d(start)/d(top_row) = 1 - H/total ∈ (0, 1)`, so the window advances smoothly
(`H/total` rows per scrolled row), never jumps, and the preview content moves
up while the thumb moves down — real-time, gap-free.

At the document bottom the window is clamped and still returns up to `H` rows,
so the track stays filled.

## 3. API split — woocraft / backend boundary

The `CodeEditor`/`EditorBackend` design is backend-pluggable (logs, code
files, diff views, virtualized lists…). The minimap must not bake in
log- or code-file assumptions. The boundary below keeps woocraft generic:

| Concern | Owner | Notes |
| --- | --- | --- |
| Thumb geometry (travel mapping, `indicator_y`) | woocraft | existing scrollbar |
| Window selection (fit vs windowed) | woocraft | generic minimap policy |
| Row → track-pixel scaling + seamless tiling | woocraft | cumulative rounding |
| Thumb overlay on the preview | woocraft | existing |
| Preview row content / colors | **backend** | level colors for logs |
| Clamp to own content, parse budget | **backend** | ≤ window length, never beyond |

**Row-space contract**: `scrollbar_preview`'s `row_window` is in the
**backend's own row indexing** (logical lines for text backends — the same
indexing as `EditorSnapshot::line_count()`). The editor computes the window
from its display-row scroll state and translates exactly one value — the
viewport-top anchor — through `display_row_to_line_row`; for non-wrapped
content (logs, most code) the translation is the identity.

**Fit/windowed decision uses the backend's line count** (not display rows):
the strip renders one backend row per pixel, so `total <= H` means "the
content fits the track". The thumb-vs-preview alignment is exact when
display rows == backend rows (non-wrapped); for soft-wrapped text the anchor
stays correct and the pixel/row ratio is documented as approximate.

**Custom backends**: a backend that does not implement `scrollbar_preview`
gets the default `Vec::new()` (no preview) — zero impact. `ScrollbarMarker`
remains a separate *sparse* decoration primitive (dots/ticks/bars at document
rows) that can overlay the dense preview; the two are complementary.

**API (refined):**

```rust
fn scrollbar_preview(&self, row_window: Range<u64>) -> ScrollbarPreview;

pub struct ScrollbarPreview {
    pub left: Pixels,   // strip x-offset inside the track — backend chooses
    pub width: Pixels,  // strip width — backend chooses (default: full track)
    pub lines: Vec<ScrollbarPreviewLine>,  // one line per preview row
}
// row_window: rows in the backend's own indexing (logical lines for text).
// Return at most row_window.len() lines; must not parse beyond the window.
// Each line is one preview pixel; the editor scales the strip to fill the
// track height. Empty lines hide the preview.
```

The backend owns the strip's **geometry** (width/position), so several status
strips (log levels, git, LSP diagnostics…) can share the track side by side
instead of one status claiming the full width.

**WSRX (`LogBackend`)**: already returns one level-colored line per requested
row (severity alpha, transparent for level-less rows) — **no changes**.

## 4. Implementation steps (scope of changes)

All rendering changes are **concentrated in woocraft** — the backend API
shape is unchanged and WSRX is untouched:

1. `woocraft` state.rs `render_vertical_scrollbar` — replace the current
   mini-viewport window (`[top_line, +capacity)`) with the fit/windowed
   selection:
   - `total = backend snapshot line count`, `capacity = ceil(H)`;
   - `total <= capacity` → window `[0, capacity)` (fit);
   - else `anchor = display_row_to_line_row(top_row)`,
     `start = max(0, round(anchor - thumb_y))`, window
     `[start, start + capacity)` (windowed);
   - render returned lines with `scale = capacity / max(1, lines.len())` and
     cumulative rounding (`round(i*scale)`..`round((i+1)*scale)`) — seamless
     tiling in both regimes (fit stretches, windowed is 1px);
   - keep the transparent-row skip (`color.a <= 0.0`).
2. `woocraft` docs: contract above (row space, budget, scaling).
3. tests: extend woocraft preview tests for fit/windowed window selection and
   tiling; keep WSRX `LogBackend` tests unchanged.
4. run `scrollbar-preview-simulate.py` to re-verify the numeric properties.
5. `WSRX`: zero code changes — only the woocraft dependency bump.

## 5. Acceptance

- 13 lines: preview fills the whole track (chunky 1:1 minimap, no scroll).
- 300 lines / 600px track: full document at 2px/row, thumb exactly over the
  visible window.
- 10000 lines: windowed 1px/row preview, thumb global, content scrolls up
  smoothly as the thumb goes down, no gaps at either end.
