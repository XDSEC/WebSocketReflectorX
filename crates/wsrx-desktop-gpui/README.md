````markdown
# WebSocketReflectorX - GPUI Edition

A modern WebSocket tunneling application desktop version based on the GPUI framework.

> ⚠️ **Project Status**: Initialization Phase - Development in Progress

## Overview

This project represents the migration effort of the WebSocketReflectorX desktop application from the [Slint](https://slint.dev/) framework to the [GPUI](https://gpui.rs/) framework. GPUI is a native GPU-accelerated UI framework from [Zed](https://zed.dev/) editor.

### Why Migrate to GPUI?

- **Performance**: GPU-accelerated rendering, smoother animations and interactions
- **Modern Architecture**: Hybrid immediate/retained mode UI system
- **Active Development**: Continuous maintenance and improvement by Zed team
- **Rust Native**: Completely written in Rust with no FFI overhead
- **Better Control**: More granular UI customization and styling control

## Quick Start

### System Requirements

- **Rust**: 1.89.0+
- **Platform**:
  - macOS 10.13+ (requires Metal GPU support)
  - Windows 10+
  - Ubuntu 18.04+

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

### Build and Run

```bash
# Check compilation
cargo check -p wsrx-desktop-gpui

# Development build and run
cargo run -p wsrx-desktop-gpui

# Release build
cargo build -p wsrx-desktop-gpui --release
```

## Project Structure

```
crates/wsrx-desktop-gpui/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── lib.rs                  # Library root module
│   ├── logging.rs              # Logging initialization
│   ├── models/                 # Data model definitions
│   ├── views/                  # GPUI view implementations
│   ├── components/             # Reusable UI components
│   ├── bridges/                # Integration with wsrx core
│   └── styles/                 # Theme and style definitions
├── assets/                      # Static resources (icons, fonts, etc.)
├── Cargo.toml                  # Project dependency configuration
├── build.rs                    # Build script
└── MIGRATION_PLAN.md           # Detailed migration plan
```

## Development Progress

### Completed
- ✅ Project structure initialization
- ✅ Cargo.toml configuration
- ✅ Base module framework
- ✅ Detailed migration plan documentation

### In Progress
- ⏳ Core data model implementation

### Pending
- View layer implementation
- Component library development
- Bridge layer implementation
- Keyboard shortcut system
- Styling and theming
- Comprehensive test coverage
- Packaging and release

See [MIGRATION_PLAN.md](./MIGRATION_PLAN.md) for detailed development roadmap.

## Architecture Design

### Three-Layer Architecture

```
┌─────────────────────────────────────────┐
│         View Layer (GPUI Views)         │
│  ┌──────────────┬──────────────────┐   │
│  │ Root View    │ Sidebar View     │   │
│  ├──────────────┼──────────────────┤   │
│  │ Get Started  │ Connections      │   │
│  │ Network Logs │ Settings         │   │
│  └──────────────┴──────────────────┘   │
├─────────────────────────────────────────┤
│       Component Layer (UI Widgets)      │
│   TitleBar, WindowControls, Buttons...  │
├─────────────────────────────────────────┤
│      Bridge Layer (Integration)         │
│   DaemonBridge, SettingsBridge...       │
└─────────────────────────────────────────┘
        ↓
    wsrx Core
    (Tunneling Logic)
```

### State Management

Uses GPUI's `Entity` system for state management:

- **Global State**: `AppState` - Stores application-level state
- **View State**: Each View maintains its own local state
- **Event Communication**: Inter-view communication via `EventEmitter` trait

## Key Features

### User Interface
- Four main pages: Get Started, Connections, Network Logs, Settings
- Sidebar navigation with platform-specific window controls
- Real-time log stream display with severity level filtering
- Tunnel management interface supporting create/edit/delete

### Functional Features
- TCP-over-WebSocket tunnel management
- Real-time connection monitoring
- Structured logging
- Cross-platform compatibility
- Asynchronous daemon management

## Dependencies

Main dependencies (see `Cargo.toml` for complete list):

- **gpui**: GPU-accelerated UI framework
- **tokio**: Asynchronous runtime
- **tracing**: Structured logging
- **serde**: Serialization framework
- **anyhow**: Error handling

## Relationship with Original Slint Version

The original Slint version is retained in `crates/desktop/` for reference and backward compatibility.

**Migration Status**:
- `crates/desktop/` - Original Slint implementation (retained)
- `crates/wsrx-desktop-gpui/` - New GPUI implementation (in development)

## Coding Guidelines

### Code Style
```bash
# Format code
cargo fmt -p wsrx-desktop-gpui

# Check code quality
cargo clippy -p wsrx-desktop-gpui -- -D warnings
```

### Documentation
- Add documentation comments for public APIs
- Add implementation comments for complex logic
- Keep TODO comments up to date

### Testing
```bash
# Run tests
cargo test -p wsrx-desktop-gpui

# Run specific test
cargo test -p wsrx-desktop-gpui test_name
```

## GPUI Learning Resources

### Official
- [GPUI Website](https://gpui.rs/)
- [GPUI Documentation](https://docs.rs/gpui/latest/gpui/)
- [GPUI Source Code](https://github.com/zed-industries/zed/tree/main/crates/gpui)
- [GPUI Examples](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples)

### Core Concepts
- Entity and ownership model
- View and Render trait
- Element layer and styling system
- Actions and event handling
- Asynchronous task execution

## FAQ

### Can I use both versions simultaneously?
Yes. `crates/desktop/` (Slint) and `crates/wsrx-desktop-gpui/` (GPUI) can coexist.

### When will the migration be complete?
See the time estimation section in [MIGRATION_PLAN.md](./MIGRATION_PLAN.md). Estimated 4-6 weeks.

### Will the configuration format change?
No. We maintain TOML configuration format compatible with the original Slint version.

### Which platforms are supported?
- macOS (10.13+)
- Windows (10+)
- Linux (Ubuntu 18.04+)

## Troubleshooting

### Compilation Errors
```bash
# Clean build cache
cargo clean -p wsrx-desktop-gpui

# Re-check
cargo check -p wsrx-desktop-gpui -vv
```

### GPUI Runtime Errors
- Ensure system supports GPU acceleration
- macOS: Check Metal support
- Linux: Ensure required X11/Wayland libraries are installed

### Diagnostic Logging
```bash
RUST_LOG=debug cargo run -p wsrx-desktop-gpui
```

## Contributing

Contributions are welcome! Please refer to the "Contributor Guidelines" section in [MIGRATION_PLAN.md](./MIGRATION_PLAN.md).

## License

MIT License - See the LICENSE file in the project root directory

## Related Projects

- [WebSocketReflectorX](https://github.com/XDSEC/WebSocketReflectorX) - Main project
- [Zed Editor](https://zed.dev/) - GPUI creator
- [GPUI](https://gpui.rs/) - UI framework

---

**Project Status**: Initialization Phase
**Last Updated**: 2025-11-09
**Maintainers**: XDSEC Team

````
