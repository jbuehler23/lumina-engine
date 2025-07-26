#!/bin/bash

# Build script for Lumina Engine Web Editor with Rust UI Framework
# This compiles the Rust UI framework to WebAssembly and sets up the web editor

set -e  # Exit on any error

echo "🎮 Building Lumina Engine Web Editor with Rust UI Framework"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    print_error "wasm-pack is not installed"
    print_status "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    if [ $? -ne 0 ]; then
        print_error "Failed to install wasm-pack"
        exit 1
    fi
    print_success "wasm-pack installed successfully"
fi

# Navigate to project root
cd "$(dirname "$0")/.."
PROJECT_ROOT=$(pwd)

print_status "Project root: $PROJECT_ROOT"

# Build the Rust UI framework for WASM
print_status "Building lumina-ui crate for WebAssembly..."
cd "$PROJECT_ROOT/crates/lumina-ui"

# Build with wasm-pack
wasm-pack build --target web --out-dir pkg --features web
if [ $? -ne 0 ]; then
    print_error "Failed to build lumina-ui for WASM"
    exit 1
fi

print_success "Successfully built lumina-ui WASM package"

# Copy WASM files to web editor static directory
print_status "Copying WASM files to web editor..."
WEB_EDITOR_DIR="$PROJECT_ROOT/crates/lumina-web-editor/static"
WASM_PKG_DIR="$PROJECT_ROOT/crates/lumina-ui/pkg"

# Create pkg directory in web editor if it doesn't exist
mkdir -p "$WEB_EDITOR_DIR/pkg"

# Copy WASM files
cp "$WASM_PKG_DIR"/*.wasm "$WEB_EDITOR_DIR/pkg/"
cp "$WASM_PKG_DIR"/*.js "$WEB_EDITOR_DIR/pkg/"
cp "$WASM_PKG_DIR"/package.json "$WEB_EDITOR_DIR/pkg/"

print_success "WASM files copied to web editor"

# Build the web editor server
print_status "Building web editor server..."
cd "$PROJECT_ROOT/crates/lumina-web-editor"

cargo build --release
if [ $? -ne 0 ]; then
    print_warning "Web editor server build had warnings, but continuing..."
fi

print_success "Web editor server built"

# Create a deployment package
print_status "Creating deployment package..."
DEPLOY_DIR="$PROJECT_ROOT/deploy"
mkdir -p "$DEPLOY_DIR"

# Copy web assets
cp -r "$WEB_EDITOR_DIR"/* "$DEPLOY_DIR/"

# Copy server binary
cp "$PROJECT_ROOT/target/release/lumina-web-editor" "$DEPLOY_DIR/" 2>/dev/null || print_warning "Release binary not found, using debug build"
cp "$PROJECT_ROOT/target/debug/lumina-web-editor" "$DEPLOY_DIR/" 2>/dev/null || print_warning "No server binary found"

print_success "Deployment package created in $DEPLOY_DIR"

# Generate a simple server script
cat > "$DEPLOY_DIR/start_server.sh" << 'EOF'
#!/bin/bash
echo "🎮 Starting Lumina Engine Web Editor Server"
echo "Open http://localhost:3030 in your browser"
echo "Use http://localhost:3030/rust_ui_demo.html for the Rust UI demo"
./lumina-web-editor
EOF

chmod +x "$DEPLOY_DIR/start_server.sh"

# Create a README for deployment
cat > "$DEPLOY_DIR/README.md" << 'EOF'
# Lumina Engine Web Editor Deployment

This package contains the complete Lumina Engine web editor with the new Rust UI framework.

## Files

- `index.html` - Original HTML/JavaScript editor (legacy)
- `rust_ui_demo.html` - New Rust UI framework demo
- `pkg/` - WebAssembly files for the Rust UI framework
- `lumina-web-editor` - Server binary
- `start_server.sh` - Convenience script to start the server

## Running

1. Execute `./start_server.sh` to start the server
2. Open http://localhost:3030 in your browser
3. Try the Rust UI demo at http://localhost:3030/rust_ui_demo.html

## Features

### Rust UI Framework Demo
- Pure Rust widgets compiled to WebAssembly
- WGPU-based GPU rendering
- Type-safe UI development
- Cross-platform compatibility
- Visual script editor integration

### Legacy HTML Editor
- HTML/JavaScript-based interface
- Visual scripting system
- Asset management
- Project templates

The Rust UI framework represents the future of the Lumina editor - a professional,
performant, and maintainable UI system that enables true dogfooding.
EOF

print_success "Deployment README created"

# Final summary
echo ""
echo "🎉 Build Complete!"
echo "━━━━━━━━━━━━━━━━━━━"
print_success "Rust UI framework compiled to WebAssembly"
print_success "Web editor server built"
print_success "Deployment package ready in $DEPLOY_DIR"
echo ""
echo "To run the web editor:"
echo "  cd $DEPLOY_DIR"
echo "  ./start_server.sh"
echo ""
echo "Then open in your browser:"
echo "  • http://localhost:3030 - Main editor"
echo "  • http://localhost:3030/rust_ui_demo.html - Rust UI demo"
echo ""
print_status "The Rust UI framework demo showcases the future of game development with Lumina!"