# Actual Architecture Analysis from Slint Project

## Core Concept: Scope-Based Architecture

The application is **scope-centric**, not page-centric. Each "scope" represents a domain/site that can control tunnels.

### Scope Types
1. **Default Scope** (`default-scope`): User-controlled, manual tunnel creation
2. **External Scopes**: Remote domains that request control (e.g., `gzctf.example.com`)

### Sidebar Navigation
The sidebar does NOT show pages. It shows:
1. "Get Started" - Main page with tunnel creation form
2. "Network Logs" - Tracing logs page
3. **Separator**
4. "Default Scope" - User's manual tunnels
5. **Dynamic Scope List** - External domains (with status icons)
6. **Separator**
7. "Settings" - Configuration
8. "Controller Port" - API status button

### Page/View Structure

#### 1. Get Started Page (`home`)
**Purpose**: Primary tunnel creation interface

**Components**:
- Application branding with animated cursor
- Update notification button (if available)
- **Network interface selector dropdown** (127.0.0.1, 0.0.0.0, LAN IPs)
- **Port input field**
- **Remote WebSocket address input** (ws:// or wss://)
- **Send button** - Creates tunnel and navigates to default-scope

**NOT a modal - this IS the main page**

#### 2. Connections Page (Dynamic, scope-specific)
**Purpose**: Display tunnels for current scope

**Displayed for**: Any scope page (default-scope or external domain)

**Header Section**:
- Scope icon (globe-star for default, globe-warning if pending, lock-closed if allowed)
- Scope name and host
- Control type badge (Manually Controlled vs External Controlled)
- Features list (e.g., "basic", "basic,pingfall")
- **Accept/Decline buttons** (for pending external scopes)
- **Remove button** (for allowed external scopes)

**Instance List** (scrollable):
- Each tunnel shows:
  - Label (custom name)
  - Local address (clickable to copy)
  - Remote address
  - Latency in ms (or "--" if connecting)
  - Click anywhere to copy local address
  - Click latency area to close tunnel

**Behavior**:
- Default scope: User creates tunnels from Get Started page
- External scopes: Remote API creates tunnels, user accepts/declines scope access

#### 3. Network Logs Page (`logs`)
**Purpose**: Real-time tracing log display

**Features**:
- Streams from `wsrx.log` file (JSON format)
- Displays logs with:
  - Level badge (DEBUG/INFO/WARN/ERROR) with color
  - Target module name
  - Timestamp
  - Message (word-wrapped)
- Opacity varies by level (DEBUG=0.5, INFO=0.8, WARN/ERROR=1.0)
- Separator lines between entries

**NOT sample data - reads actual tracing logs**

#### 4. Settings Page (`settings`)
**Purpose**: Configuration and about info

**Sections**:
- Theme selector
- Language selector  
- Running in tray toggle
- System information display
- Version info

### Data Models

#### Instance (Tunnel)
```rust
pub struct Instance {
    label: String,        // Display name
    remote: String,       // ws:// or wss:// address
    local: String,        // IP:port
    latency: i32,         // -1 if not connected
    scope_host: String,   // Which scope owns this tunnel
}
```

#### Scope
```rust
pub struct Scope {
    host: String,         // Domain name (unique ID)
    name: String,         // Display name
    state: String,        // "pending", "allowed", "syncing"
    features: String,     // Comma-separated: "basic", "pingfall"
    settings: HashMap,    // Feature-specific config
}
```

#### Log Entry
```rust
pub struct Log {
    timestamp: String,    // "2025-11-10 15:30:45"
    level: String,        // "DEBUG", "INFO", "WARN", "ERROR"
    target: String,       // Module path (e.g., "wsrx::tunnel")
    message: String,      // Log message
}
```

### Bridges (State Management)

#### WindowControlBridge
- Window control actions (drag, minimize, maximize, close)

#### SystemInfoBridge
- OS type, version, has_updates
- Network interfaces list
- **Logs array** (for Network Logs page)
- Callbacks: refresh_interfaces, open_link, open_logs

#### InstanceBridge
- **instances**: All tunnels across all scopes
- **scoped_instances**: Filtered tunnels for current scope
- Callbacks:
  - `add(remote, local)`: Create tunnel (default-scope only)
  - `del(local)`: Delete tunnel

#### ScopeBridge
- **scopes**: Array of all external scopes
- Callbacks:
  - `allow(host)`: Accept external scope
  - `del(host)`: Remove/decline scope

#### SettingsBridge
- theme, language, running_in_tray
- api_port, online (daemon status)

#### UiState (Global State)
- **page**: Current page ID (string)
  - "home" - Get Started
  - "logs" - Network Logs
  - "settings" - Settings
  - "default-scope" - User's tunnels
  - `<domain>` - External scope (e.g., "gzctf.example.com")
- **scope**: Current scope object (for connections page)
- **show_sidebar**: Boolean
- Callbacks:
  - `change_scope(host)`: Updates scope and filters scoped_instances

### Navigation Flow

1. **App starts** → "home" page (Get Started)
2. **User creates tunnel** → Navigates to "default-scope" page
3. **External domain requests** → New scope appears in sidebar with "pending" status
4. **User clicks pending scope** → Shows connections page with Accept/Decline buttons
5. **User accepts** → Scope state becomes "allowed", shows tunnels
6. **Click "Network Logs"** → Shows logs page
7. **Click "Settings"** → Shows settings page

### Key Differences from Initial Implementation

#### ❌ What I Got Wrong:
1. Tunnel creation via modal dialog - **WRONG**, it's the main Get Started page
2. Connections page as standalone - **WRONG**, it's scope-specific
3. Sample log data - **WRONG**, must stream from tracing log file
4. Static page navigation - **WRONG**, sidebar shows scopes not pages
5. Separate pages for each view - **PARTIALLY WRONG**, connections page is dynamic

#### ✅ What to Keep:
- GPUI entity-based state management
- Component library (buttons, inputs, etc.)
- Vertical scrolling implementation
- Window controls and title bar

### Implementation Priority

1. **Phase 1**: Correct data models (Scope, Instance, Log with proper fields)
2. **Phase 2**: Implement UiState global state with scope management
3. **Phase 3**: Rewrite Get Started page as tunnel creation form
4. **Phase 4**: Make Connections page scope-aware with accept/decline
5. **Phase 5**: Implement tracing log subscriber and file streaming
6. **Phase 6**: Update sidebar to show scopes dynamically
7. **Phase 7**: Wire up bridges for actual daemon communication
