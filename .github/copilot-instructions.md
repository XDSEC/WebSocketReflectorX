# WebSocket Reflector X - AI Coding Agent Instructions

## Project Overview

**WebSocketReflectorX** is a controlled TCP-over-WebSocket tunneling tool written in Rust. It enables secure forwarding of TCP connections through WebSocket channels for environments with restricted network access (e.g., Ret2Shell, GZCTF platforms).

### Repository Structure

- **Workspace**: Two main crates in `crates/`
  - `wsrx`: Core library + CLI tools (daemon, client, server modes)
  - `desktop`: Cross-platform GUI application
- **Key directories**:
  - `crates/wsrx/src/`: Core proxy logic, tunnel management, CLI handlers
  - `crates/desktop/src/`: UI setup, daemon integration, native bridges
  - `crates/desktop/ui/`: UI markup and styling (framework-agnostic design)
  - `packages/`: Optional TypeScript client/documentation

## Architecture

### Three-Mode Operation

The project implements three complementary tunneling modes (launched via CLI arguments):

1. **Daemon Mode** (`wsrx daemon`): Server accepting WebSocket connections from clients
2. **Client Mode** (`wsrx connect <url>`): Lightweight CLI client connecting to daemon
3. **Serve Mode** (`wsrx serve`): Standalone server mode for direct deployments

Each mode shares the same underlying `proxy::proxy()` function for TCP↔WebSocket conversion.

### Core Abstraction Layer

**Location**: `crates/wsrx/src/proxy.rs`

The `WrappedWsStream` enum abstracts over two incompatible WebSocket implementations:

- **Tungstenite**: Used by client via `tokio-tungstenite` (feature: `client`)
- **Axum**: Used by server via `axum::extract::ws` (feature: `server`)

This design pattern allows shared proxy logic to work with both without conditional compilation in business logic. The `Message` enum normalizes all message types to Binary variants.

### Feature-Driven Compilation

**Location**: `crates/wsrx/Cargo.toml`

```toml
features:
  binary = ["server", "client", "log", ...]  # CLI tool
  client = ["dep:tokio-tungstenite"]
  server = ["dep:axum"]
```

- Library builds with `default` (binary off) for integration
- Binary requires both `client` and `server` features
- Desktop app (`wsrx-desktop`) bundles the library with all features

### Desktop App Architecture

**Location**: `crates/desktop/src/`

- **`launcher.rs`**: Single-instance guard via lock file, UI initialization
- **`daemon/`**: Spawns `wsrx` daemon subprocess with forwarded messages
- **`bridges/`**: Native bridges connecting UI layer to Rust core:
  - `ui_state.rs`: Reactive state binding for tunnels, connections, logs
  - `settings.rs`: TOML config persistence (platform-specific directories via `directories` crate)
  - `system_info.rs`: System resource monitoring
  - `window_control.rs`: Window lifecycle and menu interactions

## Critical Workflows

### Building

```bash
# Library only (no features)
cargo build -p wsrx --lib

# CLI tool with all features
cargo build -p wsrx --bin wsrx --release

# Desktop application
cargo build -p wsrx-desktop --release
```

### Platform-Specific Build Considerations

- **macOS**: Requires manual code-signing workaround (`sudo xattr -cr` in README). Desktop app uses platform-specific window configuration.
- **Windows**: Icon compilation via `winres` in `build.rs`
- **Linux**: AppImage generation via deployments script

### Testing

No dedicated test suite. Validation occurs via:

1. Manual smoke tests of proxy logic
2. Desktop app functional testing
3. CI builds on GitHub

## Desktop Application UI/UX Design

### Page Architecture

The desktop application implements a **multi-page layout** with left sidebar navigation:

1. **Get Started Page**

   - Onboarding experience for new users
   - Displays application branding and welcome message
   - Update notification banner (if available)
   - Primary action buttons for initial setup

2. **Connections Page**

   - Main hub for managing WebSocket tunnels and connections
   - Shows active scopes/instances with status indicators (pending, allowed, syncing)
   - Displays real-time connection statistics
   - Supports creating/editing tunnels with local/remote address binding
   - Visual indicators for connection states (icons + color coding)

3. **Network Logs Page**

   - Real-time log stream display with severity levels
   - Log levels: DEBUG (transparent), INFO (semi-opaque), WARN, ERROR
   - Structured log display: level + target + timestamp + message
   - Auto-scrolling to latest entries
   - Uses color-coded severity for quick scanning

4. **Settings Page**
   - Application configuration and metadata
   - Branded header with visual branding elements
   - Settings groups organized vertically
   - Toggle controls for daemon behavior, logging levels
   - Platform-specific options (macOS vs Windows vs Linux)

### UI Component Structure

**Main Window Layout**

- Container: Custom window chrome for all platforms
- Layout: [SideBar | MainContent]
  - **SideBar**: Tab navigation + status indicators, collapsible
  - **MainContent**: [TitleBar | PageStack]
    - **TitleBar**: Window controls (minimize/maximize/close), platform-aware
    - **PageStack**: Page transitions between 4 pages

**Navigation Model**

- State managed in `bridges/ui_state.rs`: `page` property
- Page switching is reactive (state-driven, not imperative)
- SideBar tabs include visual indicators for active scope/instance state

### Design System & Theming

**Core Principles**:

- Consistent visual hierarchy with background layers (layer-1, layer-2, layer-3)
- Foreground color (window-fg) with semantic colors (error, warn, info, debug)
- Consistent spacing units across the application
- Platform-aware styling (different treatment for macOS, Windows, Linux)

**Implementation Guidelines**:

- Use centralized theme/style definitions (NOT scattered throughout components)
- Support light/dark mode switching (store preference in config)
- Ensure readable contrast ratios for accessibility
- Consistent padding, spacing, and border radius across all pages

### State Binding & Reactivity

**Core Principles**:

- Bidirectional binding between UI and Rust state
- Page content updates reactively when state changes
- List views auto-update when data changes (no manual refresh)

**Bridge Architecture** (`crates/desktop/src/bridges/`):

- `WindowControlBridge`: Native window control callbacks (close, minimize, maximize)
- `SystemInfoBridge`: System info + logs stream
- `InstanceBridge`: Tunnel instance management
- `ScopeBridge`: WebSocket scope state management
- `SettingsBridge`: TOML config persistence

### Accessibility & Platform Considerations

- **macOS**: Frameless window with transparent titlebar, custom window chrome
- **Windows**: Standard window frame with platform conventions
- **Linux**: Platform-agnostic with window manager compatibility
- Global keyboard shortcut: Ctrl+Q to quit application
- Tab navigation and focus management throughout
- Keyboard-accessible controls (all interactive elements reachable via keyboard)

### Internationalization (i18n)

- Translation system support for UI strings
- Language auto-detection from system settings
- Centralized locale management
- Right-to-left (RTL) language support considerations

## External Dependencies & Integration Points

### Critical Dependencies

- **Tokio**: Async runtime (required by all crates)
- **Axum**: HTTP + WebSocket server for daemon mode
- **tokio-tungstenite**: WebSocket client implementation
- **Rustls**: TLS, uses AWS-LC-RS (with ring fallback)
- **UI Framework**: GPUI (replacing previous Slint implementation)

### Crypto Backend Selection

**Location**: `crates/wsrx/src/main.rs` lines 66-80

Priority order (configurable):

1. AWS-LC-RS (preferred)
2. Ring (fallback)

Log initialization **before** crypto selection. If both backends fail, exit immediately.

### Platform Detection

Uses `build-target` crate for compile-time platform info. Generated constants in `constants.rs` include:

- `WSRX_VERSION`: Semantic version
- `WSRX_FULL_VERSION`: Version with git commit, arch, OS, environment

## Common Gotchas

1. **Library vs. CLI**: `wsrx` library exports only `proxy::proxy()`, `tunnel::Tunnel`. CLI features gated by feature flags.

2. **Instance Locking**: Desktop app enforces single-instance via lock file in `~/.local/share/wsrx-desktop/`. Subsequent launches notify the existing instance to raise its window.

3. **Subprocess Daemon**: Desktop spawns `wsrx daemon` subprocess. Daemon's stdout/stderr must be captured for log integration into UI.

4. **Cross-Platform Paths**: Use `directories` crate for config paths (ProjectDirs), NOT hardcoded paths.

5. **Heartbeat Timeout**: Daemon supports optional heartbeat; desktop app does not set it by default. Useful for ephemeral deployments.

6. **UI Framework Migration**: The current Slint-based UI is planned for replacement. New implementations must maintain the same page architecture (Get Started, Connections, Network Logs, Settings) and state binding model.

## Key Files to Understand New Features

- **Protocol details**: `docs/PROTOCOL.md` (binary message format)
- **Daemon CLI parsing**: `crates/wsrx/src/main.rs` (clap Parser)
- **Proxy core**: `crates/wsrx/src/proxy.rs` (Message handling, Stream/Sink traits)
- **UI logic**: `crates/desktop/src/bridges/ui_state.rs` (state synchronization)
- **Build configuration**: `crates/desktop/build.rs` (constants generation, platform resources)

---

**Last Updated**: 2025-11-09
**Rust Edition**: 2024 | **MSRV**: 1.89.0
**License**: MIT
