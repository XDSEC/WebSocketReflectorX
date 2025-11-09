#!/bin/bash
# Manual Test Runner for wsrx-desktop-gpui
# This script helps test the application manually with different scenarios

set -e

echo "=== WebSocket Reflector X Desktop - Manual Test Runner ==="
echo ""

# Check if display is available
if [ -z "$DISPLAY" ]; then
    echo "⚠️  No DISPLAY environment variable set."
    echo "    This application requires a display to run."
    echo "    Options:"
    echo "    1. Run in a graphical environment"
    echo "    2. Use Xvfb for headless testing: xvfb-run ./manual_test.sh"
    echo ""
    exit 1
fi

echo "✓ Display detected: $DISPLAY"
echo ""

# Build the application
echo "Building application..."
cargo build -p wsrx-desktop-gpui --release
echo "✓ Build complete"
echo ""

# Run unit tests
echo "Running unit tests..."
cargo test -p wsrx-desktop-gpui --lib
echo "✓ All tests passed"
echo ""

# Launch the application
echo "Launching application..."
echo "  - Click sidebar tabs to navigate between pages"
echo "  - Verify no panics occur during navigation"
echo "  - Check that page content updates correctly"
echo ""
echo "Press Ctrl+C to stop the application"
echo ""

cargo run -p wsrx-desktop-gpui --release
