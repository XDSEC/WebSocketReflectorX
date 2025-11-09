# Testing Guide for wsrx-desktop-gpui

## Overview

This document describes how to test the GPUI-based desktop application.

## Unit Tests

### Running Unit Tests

```bash
cd crates/wsrx-desktop-gpui
cargo test --lib
```

### Test Coverage

Current unit tests cover:
- **AppState**: State management, tunnel CRUD operations, log buffer management
- All 8 tests passing

### Adding New Tests

Tests are located in `src/models/app_state_tests.rs`. To add new tests:

1. Add test functions with `#[test]` attribute
2. Use descriptive test names: `test_<functionality>_<scenario>`
3. Follow AAA pattern: Arrange, Act, Assert

## Integration Testing

### Manual Testing

Since GPUI requires a display, manual testing is necessary for UI verification.

#### Prerequisites

- Linux with X11 display server
- Or use Xvfb for headless testing

#### Running Manual Tests

```bash
# With display available
./manual_test.sh

# Headless with Xvfb
xvfb-run ./manual_test.sh
```

#### Test Scenarios

**Navigation Testing:**
1. Launch application
2. Click "Get Started" tab → Verify welcome screen displays
3. Click "Connections" tab → Verify connections page loads
4. Click "Network Logs" tab → Verify logs page loads
5. Click "Settings" tab → Verify settings page loads
6. Repeat navigation → **Verify no panics occur**

**State Management Testing:**
1. Navigate between pages multiple times
2. Verify active tab indicator updates correctly
3. Verify page content switches appropriately
4. Check console for any error messages

**Visual Testing:**
1. Verify sidebar displays correctly
2. Check hover effects on inactive tabs
3. Verify active tab has accent color
4. Confirm all text is readable
5. Check layout responsiveness

### Known Limitations

- No automated UI testing framework yet (GPUI doesn't have mature testing tools)
- Requires X11 display or Xvfb for any UI testing
- Cannot run in CI without display configuration

## Debugging

### Common Issues

**Panic: "cannot update X while it is already being updated"**
- **Cause**: Nested Entity updates (circular dependencies)
- **Solution**: Ensure entities update their own state before calling callbacks
- **Fixed in**: commit d998639

**Linking errors (libxcb, libxkbcommon)**
- **Cause**: Missing X11 development libraries
- **Solution**: `sudo apt-get install libxcb1-dev libxkbcommon-dev libxkbcommon-x11-dev`

### Debug Build

```bash
cargo build -p wsrx-desktop-gpui
RUST_LOG=debug cargo run -p wsrx-desktop-gpui
```

### Release Build

```bash
cargo build -p wsrx-desktop-gpui --release
cargo run -p wsrx-desktop-gpui --release
```

## Future Testing Improvements

1. **Automated UI Testing**: Investigate GPUI testing capabilities when mature
2. **CI Integration**: Set up Xvfb in CI for headless testing
3. **Property Testing**: Add property-based tests for state management
4. **Integration Tests**: Test bridge layer with mock daemon
5. **Performance Testing**: Benchmark render performance with large data sets

## Test Checklist

Before considering a feature complete:

- [ ] Unit tests written and passing
- [ ] Manual testing performed
- [ ] No panics or errors in console
- [ ] Visual inspection confirms correct rendering
- [ ] Navigation works smoothly
- [ ] State updates correctly
- [ ] Memory usage is reasonable
- [ ] No console warnings

## Reporting Issues

When reporting test failures or bugs:

1. Specify test type (unit/manual)
2. Include full error message and stack trace
3. Describe reproduction steps
4. Note platform and environment (Linux, X11 vs Wayland, etc.)
5. Include GPUI version from Cargo.toml
