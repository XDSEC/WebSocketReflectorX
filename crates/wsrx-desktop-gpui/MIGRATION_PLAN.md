# WebSocketReflectorX GPUI Migration Plan

## Overview

This document provides a comprehensive migration plan for transitioning the WebSocketReflectorX desktop application from the Slint-based `crates/desktop` to the GPUI-based `crates/wsrx-desktop-gpui`.

**Project Status**: Initialization Phase - Base project structure established

**Reference Resources**:

- GPUI Documentation: https://docs.rs/gpui/latest/gpui/
- GPUI Website: https://gpui.rs/
- GPUI Source Code: https://github.com/zed-industries/zed/tree/main/crates/gpui
- Original Slint Project: `crates/desktop/` (kept for reference)

---

## ⚠️ CRITICAL: Verify GPUI Patterns Before Coding

**GPUI documentation can be outdated or incomplete.** Always verify patterns by checking actual Zed examples:

### How to Access Official Examples

```bash
# Clone the Zed repository (lightweight, examples only)
git clone --depth 1 --filter=blob:none --sparse https://github.com/zed-industries/zed.git
cd zed
git sparse-checkout set crates/gpui/examples crates/gpui/src

# Browse examples
cd crates/gpui/examples
ls -la
# You'll see: hello_world.rs, painting.rs, uniform_list.rs, text.rs, window.rs, etc.

# Read an example
cat hello_world.rs

# Run an example (requires full checkout and dependencies)
# cd ../../../
# cargo run --example hello_world -p gpui
```

### Key Examples to Study

| Example File       | What It Teaches                                   | Priority      |
| ------------------ | ------------------------------------------------- | ------------- |
| `hello_world.rs`   | Basic app structure, window creation, simple view | **Essential** |
| `painting.rs`      | Entity updates, callbacks, state management       | **Essential** |
| `uniform_list.rs`  | Efficient list rendering (for logs/tunnels)       | High          |
| `window.rs`        | Window management, multiple windows               | Medium        |
| `text.rs`          | Text rendering, styling                           | Medium        |
| `focus_visible.rs` | Keyboard focus, actions system                    | High          |
| `gif_viewer.rs`    | Asset loading (images)                            | Low           |

### What to Look For in Examples

1. **View Structure** - What fields do views hold?

   ```rust
   // ✅ Look for this pattern
   struct MyView {
       simple_field: usize,
       string_data: String,
       // NOT Entity<T> handles!
   }
   ```

2. **Render Signature** - Exact function signature

   ```rust
   // ✅ Verify this exact pattern
   impl Render for MyView {
       fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement
   ```

3. **Entity Creation** - How entities are created

   ```rust
   // ✅ Look for this pattern
   cx.new(|cx| MyView { /* ... */ })
   ```

4. **State Updates** - How to trigger re-renders

   ```rust
   // ✅ Look for this pattern
   entity.update(cx, |view, cx| {
       view.field = new_value;
       cx.notify();  // Triggers re-render
   });
   ```

5. **Event Handling** - Click handlers, keyboard shortcuts
   ```rust
   // ✅ Look for this pattern
   div().on_click(cx.listener(|view, _event, window, cx| {
       // Handle click
   }))
   ```

### Common Pitfalls to Avoid

❌ **DON'T** hold `Entity<T>` directly in view structs (causes ownership issues)
✅ **DO** hold plain data types, use `WeakEntity<T>` for callbacks if needed

❌ **DON'T** trust outdated blog posts or documentation
✅ **DO** verify every pattern against actual Zed examples

❌ **DON'T** assume GPUI works like other UI frameworks
✅ **DO** follow Zed's patterns exactly, especially for state management

### Quick Verification Checklist

Before implementing any GPUI feature, ask:

- [ ] Have I checked the relevant Zed example?
- [ ] Does my view struct hold only plain data?
- [ ] Am I using the correct render signature?
- [ ] Am I calling `cx.notify()` after state changes?
- [ ] Am I using `cx.listener()` for event handlers?

---

## Part 1: Architecture Overview

### GPUI Core Concepts

GPUI is a GPU-accelerated UI framework using hybrid immediate/retained mode, with three layers:

1. **Entity Layer** (State Management)

   - GPUI manages the lifecycle of all Entities
   - Strong references via `Entity<T>`
   - Weak references via `WeakEntity<T>`
   - ⚠️ **Note**: `EventEmitter` not found in current GPUI examples; use callbacks/closures or GPUI's action system for inter-entity communication

2. **View Layer** (High-level Declarative UI)

   - Entities implementing `Render` trait
   - `render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement`
   - Reactive updates (state changes trigger re-render automatically)

3. **Element Layer** (Low-level Imperative UI)
   - Basic building blocks: `div()`, `text()`, etc. (lowercase functions)
   - Tailwind-style CSS API: `.flex()`, `.w_full()`, `.bg()`, etc.
   - Manual control over layout and styling

### Application Architecture Design

```
Application (GPUI)
├── App (global context)
│   └── GlobalState (Entity<AppState>)
│       ├── DaemonBridge
│       ├── SettingsBridge
│       ├── SystemInfoBridge
│       └── NotificationCenter
└── Window
    └── RootView (Entity<RootState>)
        ├── SidebarView (Entity<SidebarState>)
        └── MainContentView (Entity<MainContentState>)
            ├── TitleBarComponent
            └── PageStack
                ├── GetStartedView (Entity<GetStartedState>)
                ├── ConnectionsView (Entity<ConnectionsState>)
                ├── NetworkLogsView (Entity<NetworkLogsState>)
                └── SettingsView (Entity<SettingsState>)
```

**Context Types**:

- `App` - Global application context (passed as `cx: &mut App`)
- `Window` - Window-level state (passed as `window: &mut Window`)
- `Context<T>` - Entity-specific context (passed as `cx: &mut Context<T>`)

### State Management Pattern

Uses **central event-driven** state management:

- **Global Application State**: `AppState` (in `src/models/app_state.rs`)

  - Stores application-level state (active page, global settings, etc.)
  - Managed by `RootView` or other top-level views

- **Local View State**: Each View maintains its own state

  - `ConnectionsViewState`: tunnel list, selected item, edit state
  - `NetworkLogsViewState`: log buffer, filter conditions
  - `SettingsViewState`: form inputs, validation state

- **Event Communication**: Pass events between views via callbacks or GPUI's action system
  - Callback pattern: `Arc<dyn Fn(...) + Send + Sync>`
  - Action pattern: `actions!(app, [MyAction]);` (see Phase 6)

---

## Part 2: Detailed Migration Steps

### Phase 1: Base Framework (Completed) ✅

**Goal**: Establish a compilable project structure

**Completed Items**:

- ✅ Created `crates/wsrx-desktop-gpui/` directory structure
- ✅ Configured `Cargo.toml` with GPUI dependencies
- ✅ Wrote `build.rs` (platform-specific build)
- ✅ Created module framework (views, components, models, bridges, styles)
- ✅ Implemented basic `logging.rs`
- ✅ Wrote placeholder source files

**File Checklist**:

```
crates/wsrx-desktop-gpui/
├── Cargo.toml                    # GPUI dependency configuration
├── build.rs                      # Build script
├── src/
│   ├── main.rs                   # Application entry point with i18n macro
│   ├── logging.rs                # Logging initialization
│   ├── i18n.rs                   # Internationalization setup
│   ├── models/mod.rs             # Data model definitions
│   ├── styles/mod.rs             # Themes and styles
│   ├── views/
│   │   ├── mod.rs
│   │   ├── root.rs              # Main window root view
│   │   ├── get_started.rs       # Onboarding page
│   │   ├── connections.rs       # Tunnel management
│   │   ├── network_logs.rs      # Log display
│   │   ├── settings.rs          # Settings page
│   │   └── sidebar.rs           # Navigation sidebar
│   ├── components/
│   │   ├── mod.rs
│   │   ├── prelude.rs           # Common component imports
│   │   ├── title_bar.rs         # Window title bar with drag support
│   │   ├── window_controls.rs   # Platform-aware window controls

│   │   └── tab_navigation.rs    # Placeholder
│   └── bridges/
│       ├── mod.rs
│       ├── daemon.rs            # Placeholder
│       ├── settings.rs          # Placeholder
│       └── system_info.rs       # Placeholder
└── assets/                       # Resources to be added later
```

**Verification Steps**:

```bash
cd crates/wsrx-desktop-gpui
cargo check --lib
```

---

### Phase 2: Core Data Models and Global State (Next)

**Goal**: Define application data models and state management

**To-Implement Items**:

1. **Expand `models/mod.rs`**

   - [ ] Add complete `Tunnel`, `Connection`, `LogEntry` definitions
   - [ ] Add state enums (`ConnectionStatus`, `LogLevel`, etc.)
   - [ ] Add `Settings` config structure

2. **Create `models/app_state.rs`**

   ```rust
   pub struct AppState {
       pub current_page: Page,
       pub tunnels: Vec<Tunnel>,
       pub connections: Vec<Connection>,
       pub settings: Settings,
       pub recent_logs: VecDeque<LogEntry>,
       pub daemon_status: DaemonStatus,
   }

   pub enum Page {
       GetStarted,
       Connections,
       NetworkLogs,
       Settings,
   }
   ```

3. **Create `models/events.rs`**
   - Define all event types (e.g., `TunnelCreated`, `ConnectionEstablished`, etc.)
   - Use for inter-view communication

**Key Design Decisions**:

- Use `Arc<RwLock<T>>` or GPUI's `Entity<T>`?

  - **Decision**: Primarily use `Entity<T>` to leverage GPUI's reactive system
  - `AppState` should be a global `Entity` rather than a plain struct

- How to serialize/deserialize GPUI Entities?
  - **Decision**: Entities are runtime-only; config serialized via `Settings`

---

### Phase 3: View Implementation

**Goal**: Implement GPUI views for four main pages

#### 3.1 Implement Root View

**File**: `src/views/root.rs`

```rust
use gpui::{App, Context, Render, Window, div, prelude::*};

// Views hold plain data, not Entity handles
pub struct RootView {
    current_page: Page,
}

impl RootView {
    pub fn new() -> Self {
        Self {
            current_page: Page::GetStarted,
        }
    }

    fn render_main_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.current_page {
            Page::GetStarted => div().child("Get Started"),
            Page::Connections => div().child("Connections"),
            Page::NetworkLogs => div().child("Network Logs"),
            Page::Settings => div().child("Settings"),
        }
    }
}

impl Render for RootView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Verified signature from Zed examples
        // Access global state through cx when needed, don't hold Entity references

        div()
            .flex()
            .w_full()
            .h_full()
            .bg(rgb(0x1e1e1e))
            .child(/* sidebar component */)
            .child(self.render_main_content(cx))
    }
}
```

**Important Pattern**: Views hold **plain data** (like in your example: `Copy` types, `String`, `Vec`, etc.), not `Entity<T>` handles. Access other entities through:

1. Global state via `cx`
2. Parameters to render methods
3. `WeakEntity<T>` for callbacks**Estimated Effort**: 2-3 days

#### 3.2 Implement Sidebar View

**File**: `src/views/sidebar.rs`

- Display navigation tabs (Get Started, Connections, Logs, Settings)
- Display status indicators (daemon status, connection count)
- Handle tab selection events

**Estimated Effort**: 1-2 days

#### 3.3 Implement Four Content Pages

##### Get Started Page

**File**: `src/views/get_started.rs`

- Application branding and welcome message
- Update notification banner (if available)
- Quick start buttons

**Estimated Effort**: 1 day

##### Connections Page

**File**: `src/views/connections.rs`

- Tunnel list (using `UniformList` for performance)
- Tunnel edit window (modal)
- Connection statistics
- Status indicators

**Estimated Effort**: 3-4 days
**Complexity**: High (most interactive page)

##### Network Logs Page

**File**: `src/views/network_logs.rs`

- Log stream display (using `UniformList`)
- Severity level color coding (DEBUG, INFO, WARN, ERROR)
- Search/filter functionality
- Auto-scroll to latest entries

**Estimated Effort**: 2-3 days
**Complexity**: Medium (requires efficient log handling)

##### Settings Page

**File**: `src/views/settings.rs`

- Application config options (theme, log level, etc.)
- About section
- Export config options

**Estimated Effort**: 1-2 days

---

### Phase 4: Component Library Implementation

**Goal**: Implement reusable UI components

#### 4.1 Title Bar Component

**File**: `src/components/title_bar.rs`

- Title text
- Window control button area placeholder

**Estimated Effort**: 0.5 days

#### 4.2 Window Controls Component

**File**: `src/components/window_controls.rs`

- Minimize, maximize, close buttons
- Platform-specific styling (macOS vs Windows vs Linux)
- Integration with GPUI window commands

**Estimated Effort**: 1 day
**Platform Considerations**:

- macOS: Use Cmd+Q shortcut, fullscreen support
- Windows: Standard window controls
- Linux: Show close button only (handled by window manager)

#### 4.3 Tab Navigation Component

**File**: `src/components/tab_navigation.rs`

- Tab styling
- Active state indicator
- Click event handling

**Estimated Effort**: 1 day

#### 4.4 Other Common Components

- `StatusIndicator`: Connection status dot
- `Button`: Styled button
- `TextInput`: Form input
- `Modal`: Modal dialog
- `Tooltip`: Tooltip

**Estimated Effort**: 2-3 days

---

### Phase 5: Bridge Layer Implementation

**Goal**: Connect UI with wsrx core functionality

#### 5.1 Daemon Bridge

**File**: `src/bridges/daemon.rs`

Features:

- Start/stop `wsrx daemon` subprocess
- Listen to daemon's stdout/stderr
- Forward logs to UI
- Handle tunnel commands

```rust
pub struct DaemonBridge {
    process: Option<Child>,
    log_tx: mpsc::UnboundedSender<LogEntry>,
}

impl DaemonBridge {
    pub async fn start(&mut self) -> Result<()> { }
    pub async fn stop(&mut self) -> Result<()> { }
    pub async fn add_tunnel(&self, tunnel: &Tunnel) -> Result<()> { }
}
```

**Estimated Effort**: 2-3 days

#### 5.2 Settings Bridge

**File**: `src/bridges/settings.rs`

Features:

- Read/write TOML config files
- Cross-platform config directory support (`ProjectDirs`)
- Default value management

**Estimated Effort**: 1 day

#### 5.3 System Info Bridge

**File**: `src/bridges/system_info.rs`

Features:

- CPU/memory monitoring
- Network statistics
- System status queries

**Estimated Effort**: 1 day

---

### Phase 6: Keyboard Shortcuts and Event Handling

**Goal**: Implement keyboard navigation and shortcuts

**To-Implement Items**:

- [ ] Register global shortcuts (Ctrl+Q to quit)
- [ ] Implement tab switching shortcuts (Ctrl+Tab, Ctrl+Shift+Tab)
- [ ] Implement action shortcuts (Ctrl+N new tunnel, Ctrl+S save)
- [ ] Focus management and Tab key navigation

**Using GPUI Actions**:

```rust
#[derive(Action)]
pub struct CreateTunnel;

#[derive(Action)]
pub struct DeleteTunnel(pub String);
```

**Estimated Effort**: 1-2 days

---

### Phase 7: Asynchronous Tasks and Concurrency

**Goal**: Implement background task handling

**To-Implement Items**:

- [ ] Use GPUI's `BackgroundExecutor` for background tasks
- [ ] Implement Future and Stream integration
- [ ] Handle network latency and timeouts
- [ ] Implement progress indicators

**Pattern**:

```rust
cx.spawn({
    let state = state.clone();
    async move {
        match fetch_tunnels().await {
            Ok(tunnels) => {
                state.update(cx, |state, cx| {
                    state.tunnels = tunnels;
                    cx.notify();
                });
            }
            Err(e) => { /* handle error */ }
        }
    }
}).detach();
```

**Estimated Effort**: 2 days

---

### Phase 8: Styling, Theming and Polish

**Goal**: Finalize UI appearance

**To-Implement Items**:

- [ ] Expand `styles/mod.rs` color palette
- [ ] Implement dark/light theme switching
- [ ] Responsive layout optimization
- [ ] Platform-specific styling (macOS, Windows, Linux)

**Theme Implementation**:

```rust
pub trait Theme {
    fn background_color() -> Rgba;
    fn text_color() -> Rgba;
    fn accent_color() -> Rgba;
    // ... other colors
}
```

**Estimated Effort**: 2-3 days

---

### Phase 9: Testing and Debugging

**Goal**: Ensure functional correctness and stability

**To-Implement Items**:

- [ ] Unit tests (models, bridges)
- [ ] Integration tests (UI interactions)
- [ ] Performance tests (large log processing)
- [ ] Platform testing (macOS, Windows, Linux)

**Using GPUI Tests**:

```rust
#[gpui::test]
fn test_tunnel_creation(cx: &mut TestAppContext) {
    // ... test code
}
```

**Estimated Effort**: 3-5 days

---

### Phase 10: Documentation and Packaging

**Goal**: Prepare for release

**To-Implement Items**:

- [ ] Write usage documentation
- [ ] Update contribution guidelines
- [ ] Configure CI/CD build scripts
- [ ] Package (DMG for macOS, MSI for Windows, AppImage for Linux)

**Estimated Effort**: 2-3 days

---

## Part 3: Key Technical Decisions

### 1. State Management

**Question**: How to manage complex application state in GPUI?

**Options**:

- A: Global `AppState` Entity (centralized)
- B: Distributed Entities (each view manages its own state)
- C: Hybrid approach

**Decision**: **Option C - Hybrid Approach**

Rationale:

- Global state (`AppState`): page navigation, global settings, daemon status
- Local state: each View maintains its own UI state (input values, selected items, etc.)
- Inter-view communication via `EventEmitter`

### 2. Asynchronous Communication

**Question**: How to handle real-time data streams from daemon?

**Decision**: Use `tokio::sync::mpsc` + GPUI's `spawn()`

```rust
let (tx, mut rx) = mpsc::unbounded_channel();

cx.spawn(async move {
    while let Some(msg) = rx.recv().await {
        // Update state
    }
}).detach();
```

### 3. Performance Optimization

**Question**: How to maintain UI responsiveness with large log volumes?

**Decision**:

- Use `UniformList` instead of plain `List`
- Cap log buffer size (e.g., max 10,000 entries)
- Asynchronously write logs to file

### 4. Platform Compatibility

**Question**: How to handle platform differences?

**Decision**:

- Use conditional compilation `#[cfg(target_os = "...")]`
- Create platform-specific component variants
- Generate platform constants in `build.rs`

### 5. Internationalization (i18n)

**Question**: Do we need multi-language support?

**Decision**: **Deferred** (can be added in later phases)

- Use `fluent-rs` or `icu4x` library
- All UI strings should use translation keys, not hardcoded

---

## Part 4: Risk Assessment and Mitigation

### High Risk Items

| Risk                            | Probability | Impact | Mitigation                                             |
| ------------------------------- | ----------- | ------ | ------------------------------------------------------ |
| GPUI API changes                | Medium      | High   | Frequently check upstream updates, write adapter layer |
| Platform compatibility issues   | Medium      | Medium | Early platform testing, CI/CD coverage                 |
| Log performance issues          | Low         | Medium | Use `UniformList`, cap buffer size                     |
| Daemon communication complexity | Low         | Medium | Write detailed bridge documentation                    |

### Medium Risk Items

| Risk                           | Probability | Impact | Mitigation                                          |
| ------------------------------ | ----------- | ------ | --------------------------------------------------- |
| Long compile times             | High        | Low    | Incremental builds, parallel compilation            |
| High memory usage              | Medium      | Medium | Memory profiling tools, data structure optimization |
| Insufficient component library | Low         | Medium | Reference Zed source code examples                  |

---

## Part 5: Development Environment Setup

### System Requirements

- **Rust**: 1.89.0+ (workspace already defines)
- **macOS**: 10.13+, requires Metal support
- **Windows**: Windows 10+
- **Linux**: Ubuntu 18.04+ or equivalent

### Installing Dependencies

#### macOS

```bash
xcode-select --install
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

#### Linux (Ubuntu/Debian)

```bash
sudo apt-get install build-essential libssl-dev pkg-config
```

#### Windows

```bash
# Visual Studio or Build Tools
# or use winget
winget install Microsoft.VisualStudio.BuildTools
```

### Development Commands

```bash
# Check compilation
cargo check -p wsrx-desktop-gpui

# Development build
cargo build -p wsrx-desktop-gpui

# Release build
cargo build -p wsrx-desktop-gpui --release

# Run application
cargo run -p wsrx-desktop-gpui

# Run tests
cargo test -p wsrx-desktop-gpui

# Check formatting and code quality
cargo fmt -p wsrx-desktop-gpui
cargo clippy -p wsrx-desktop-gpui -- -D warnings
```

---

## Part 6: Comparison with Original Slint Project

### File Mapping Table

| Original (Slint)    | New (GPUI)            | Notes                                    |
| ------------------- | --------------------- | ---------------------------------------- |
| `ui/main.slint`     | `src/views/root.rs`   | Main window definition                   |
| `ui/pages/*.slint`  | `src/views/*.rs`      | Page implementations                     |
| `ui/blocks/*.slint` | `src/components/*.rs` | Reusable components                      |
| `src/bridges/`      | `src/bridges/`        | Mostly the same (API adjustments needed) |
| `src/launcher.rs`   | `src/main.rs`         | Application startup                      |
| N/A                 | `src/styles/mod.rs`   | Centralized style management             |

### Key Code Migration

#### Old Way (Slint)

```rust
// In .slint file
export component MainWindow inherits Window {
    width: 800px;
    height: 600px;
    // ...
}
```

#### New Way (GPUI)

```rust
struct MainView { }

impl Render for MainView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size(Size::new(Pixels(800.0), Pixels(600.0)))
            // ...
    }
}
```

---

## Part 7: Reference Resources and Links

### Official Resources

- [GPUI Documentation](https://docs.rs/gpui/latest/gpui/)
- [GPUI Website](https://gpui.rs/)
- [Zed Source Code](https://github.com/zed-industries/zed)
- [GPUI Examples](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples)

### Learning Materials

- GPUI Ownership and Data Flow: `docs.rs/gpui/latest/gpui/_ownership_and_data_flow/`
- Taffy Layout: https://taffy.dev/
- GPUI Keyboard Events: `docs.rs/gpui/latest/gpui/` (Key Dispatch)

### Tools and Dependencies

- `directories` crate: Cross-platform directory support
- `tokio`: Async runtime
- `tracing`: Logging and tracing
- `serde`/`toml`: Config serialization

---

## Part 8: Time Estimation

### Overall Timeline

| Phase     | Description                 | Estimated Time | Status       |
| --------- | --------------------------- | -------------- | ------------ |
| 1         | Base framework              | 1-2 days       | ✅ Completed |
| 2         | Core data models            | 2-3 days       | ⏳ Next      |
| 3         | View implementation         | 10-12 days     | Pending      |
| 4         | Component library           | 4-5 days       | Pending      |
| 5         | Bridge layer                | 4-5 days       | Pending      |
| 6         | Keyboard shortcuts          | 1-2 days       | Pending      |
| 7         | Async tasks                 | 2 days         | Pending      |
| 8         | Styling and theming         | 2-3 days       | Pending      |
| 9         | Testing and debugging       | 3-5 days       | Pending      |
| 10        | Documentation and packaging | 2-3 days       | Pending      |
| **Total** |                             | **31-42 days** |              |

### Parallelization Opportunities

- Phases 3-5 can be partially done in parallel (multiple developers)
- Phases 8-9 can be done in parallel with phases 6-7

---

## Part 9: Checklist

### Before Starting

- [ ] Review this migration plan
- [ ] Understand GPUI core concepts
- [ ] Set up development environment
- [ ] Clone Zed repository as reference

### During Development

- [ ] Update this plan after each phase completion
- [ ] Regularly compare with original Slint project
- [ ] Document encountered issues and solutions
- [ ] Maintain backward-compatible config format

### Before Release

- [ ] Test on all platforms
- [ ] Performance benchmarking
- [ ] User documentation updates
- [ ] Release notes preparation

---

## Appendix: Frequently Asked Questions (FAQ)

### Q1: Why choose GPUI?

**A**: GPUI offers better performance, more modern API, alignment with the Zed project, and a more active development community.

### Q2: Can we run both versions simultaneously?

**A**: Yes. The old Slint version is in `crates/desktop/`, new version in `crates/wsrx-desktop-gpui/`. Both can coexist.

### Q3: Will the config file format change?

**A**: No. We maintain the same TOML format to ensure backward compatibility.

### Q4: Which platforms need support?

**A**: Primary support: macOS, Windows, Linux. GPUI can also support iOS/Android (future).

### Q5: What to do with old code?

**A**: Keep `crates/desktop/` as reference. After migration completes, consider archiving it.

---

## Contributor Guidelines

### Code Style

- Follow official Rust style guidelines
- Run `cargo fmt` and `cargo clippy`
- Add necessary documentation comments

### Pull Request Process

1. Create feature branch based on current phase
2. Implement feature and add tests
3. Run full tests before submitting PR
4. Reference relevant phase in PR description

### Reporting Issues

- Use GitHub Issues
- Include reproduction steps, GPUI version, Rust version
- Link to relevant phase in this plan

---

**Document Version**: 1.1
**Last Updated**: 2025-11-09 (API verified against Zed GPUI source code)
**Maintainers**: XDSEC Team

**Verification Status**: ✅ Core API terminology confirmed correct by reviewing actual Zed codebase
