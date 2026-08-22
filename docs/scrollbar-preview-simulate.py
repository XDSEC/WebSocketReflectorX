#!/usr/bin/env python3
"""Numeric validation for the scrollbar preview (minimap) design.

See docs/scrollbar-preview.md. Verifies, across many (total, viewport, H)
scenarios and EVERY scroll position:

  P1  fit regime (total <= H): thumb alignment is exact
      — indicator_y == top_row * H / total
  P2  fit regime: the thumb covers exactly the visible window on the
      stretched preview (thumb_h == viewport * H / total)
  P3  windowed regime (total > H): the row at the thumb's top edge is within
      one row of the viewport top (integer window starts quantize to <=1px)
  P4  windowed regime: the window advances monotonically, 0 or 1 rows per
      scrolled row (no jumps) — content scrolls up while the thumb moves down
  P5  windowed regime: the window never overflows the document; count <= H
  P6  both: cumulative rounding tiles the whole track with no gaps
      (sum of rendered heights == H)
"""

MIN_THUMB = 8.0


def thumb_h(H, viewport, total):
    return max(MIN_THUMB, min(H, H * viewport / total))


def indicator_y(top_row, H, viewport, total):
    max_top = total - viewport
    if max_top <= 0:
        return 0.0
    travel = H - thumb_h(H, viewport, total)
    return min(top_row, max_top) / max_top * travel


def window_for(top_row, H, viewport, total):
    """Returns (window_start, count, scale, indicator_y) per the design doc."""
    if total <= H:
        count = total
        return 0, count, H / max(1, count), 0.0
    anchor = min(top_row, total - viewport)          # viewport top (logical)
    iy = indicator_y(anchor, H, viewport, total)      # thumb top edge y (px)
    start = max(0, round(anchor - iy))
    count = min(H, max(0, total - start))
    scale = H / max(1, count)
    return start, count, scale, iy


def tiled_heights(count, scale, H):
    return [round((i + 1) * scale) - round(i * scale) for i in range(count)]


def run(scenario):
    total, viewport, H = scenario
    max_top = max(0, total - viewport)
    failures = []

    if total <= H:
        # ---- fit regime ----
        scale = H / max(1, total)
        for tr in range(max_top + 1):
            iy = indicator_y(tr, H, viewport, total)
            if abs(iy - tr * scale) > 1e-6:                       # P1
                failures.append(f"P1 top={tr}: iy {iy} != {tr*scale}")
        if total > viewport:
            th = thumb_h(H, viewport, total)
            if abs(th - viewport * scale) > 1e-6:                 # P2
                failures.append(f"P2 thumb {th} != {viewport*scale}")
        _, count, scale, _ = window_for(0, H, viewport, total)
        if sum(tiled_heights(count, scale, H)) != H:              # P6
            failures.append(f"P6 fit: tiled heights != {H}")
    else:
        # ---- windowed regime ----
        prev_start = None
        for tr in range(max_top + 1):
            start, count, scale, iy = window_for(tr, H, viewport, total)
            anchor = tr
            if abs(start + iy - anchor) > 1.0:                    # P3 (<=1px quantization)
                failures.append(f"P3 top={tr}: edge row {start+iy} vs anchor {anchor}")
            if prev_start is not None:
                d = start - prev_start
                if not (0 <= d <= 1):                             # P4 (monotonic, no jumps)
                    failures.append(f"P4 top={tr}: window {prev_start}->{start}")
            prev_start = start
            if not (0 <= count <= H and start + count <= total):  # P5
                failures.append(f"P5 top={tr}: window [{start},{start+count}) overflows {total}")
            if sum(tiled_heights(count, scale, H)) != H:          # P6
                failures.append(f"P6 top={tr}: tiled heights != {H}")

    return failures


def main():
    scenarios = [
        (13, 40, 600),       # few: fits in viewport
        (300, 40, 600),      # medium: fits in track, overflows viewport
        (600, 40, 600),      # boundary: total == H
        (650, 40, 600),      # slightly over capacity
        (3000, 40, 600),     # many
        (10000, 40, 600),    # many, big
        (50000, 100, 800),   # many, big viewport
        (60, 10, 50),        # small track, slight overflow
        (7, 24, 120),        # few, small track
    ]
    all_failures = []
    for scenario in scenarios:
        fails = run(scenario)
        total, viewport, H = scenario
        regime = "fit" if total <= H else "windowed"
        status = "PASS" if not fails else f"FAIL ({len(fails)})"
        print(f"  total={total:>6} viewport={viewport:>4} H={H:>4}  [{regime:>8}]  {status}")
        for f in fails[:8]:
            print(f"      !! {f}")
        all_failures += fails

    print()
    print(f"TOTAL: {len(all_failures)} failures")
    if all_failures:
        for f in all_failures[:20]:
            print("  ", f)
        return 1
    print("ALL PROPERTIES HOLD")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
