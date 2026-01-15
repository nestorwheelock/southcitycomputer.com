#!/bin/bash
# Development Environment Setup for Ubuntu/Debian
# South City Computer - scc-server
#
# This script installs all dependencies needed to build and develop the server.
# Tested on: Ubuntu 22.04 LTS, Ubuntu 24.04 LTS, Debian 12
#
# Usage: ./scripts/setup-dev-ubuntu.sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[OK]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

echo ""
echo -e "${YELLOW}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${YELLOW}║${NC}  ${GREEN}South City Computer - Development Environment Setup${NC}      ${YELLOW}║${NC}"
echo -e "${YELLOW}║${NC}  ${BLUE}Ubuntu/Debian${NC}                                            ${YELLOW}║${NC}"
echo -e "${YELLOW}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if running as root (not recommended)
if [ "$EUID" -eq 0 ]; then
    log_warning "Running as root. Rust should be installed as regular user."
    log_warning "System packages will be installed, then switch to regular user for Rust."
fi

# =============================================================================
# System Package Installation
# =============================================================================

log_info "Updating package lists..."
sudo apt-get update

log_info "Installing build essentials and system libraries..."
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    curl \
    git \
    jq \
    unzip

log_success "System packages installed"

# =============================================================================
# Rust Installation
# =============================================================================

if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    log_info "Rust already installed: $RUST_VERSION"
    log_info "Updating Rust..."
    rustup update stable
else
    log_info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # Source cargo environment
    source "$HOME/.cargo/env"

    log_success "Rust installed: $(rustc --version)"
fi

# Ensure cargo is in PATH for current session
export PATH="$HOME/.cargo/bin:$PATH"

# =============================================================================
# Rust Components
# =============================================================================

log_info "Installing Rust components..."

# Clippy (linter)
rustup component add clippy

# Rustfmt (formatter)
rustup component add rustfmt

# rust-analyzer (LSP for IDEs)
rustup component add rust-analyzer 2>/dev/null || log_warning "rust-analyzer not available via rustup, install via IDE"

log_success "Rust components installed"

# =============================================================================
# Cargo Tools (Optional but Recommended)
# =============================================================================

log_info "Installing useful cargo tools..."

# cargo-watch: Auto-rebuild on file changes
if ! command -v cargo-watch &> /dev/null; then
    cargo install cargo-watch
    log_success "cargo-watch installed"
else
    log_info "cargo-watch already installed"
fi

# cargo-edit: Add/remove dependencies easily
if ! command -v cargo-add &> /dev/null; then
    cargo install cargo-edit
    log_success "cargo-edit installed"
else
    log_info "cargo-edit already installed"
fi

# =============================================================================
# Node.js (for CSS/JS minification)
# =============================================================================

if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    log_info "Node.js already installed: $NODE_VERSION"
else
    log_info "Installing Node.js via NodeSource..."
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt-get install -y nodejs
    log_success "Node.js installed: $(node --version)"
fi

# Install minification tools
log_info "Installing CSS/JS minification tools..."
sudo npm install -g clean-css-cli terser
log_success "Minification tools installed"

# =============================================================================
# Image Optimization Tools
# =============================================================================

log_info "Installing image optimization tools..."
sudo apt-get install -y webp imagemagick
log_success "Image tools installed (cwebp, convert)"

# =============================================================================
# Verify Installation
# =============================================================================

echo ""
log_info "Verifying installation..."
echo ""

echo "System:"
echo "  OS:          $(lsb_release -ds 2>/dev/null || cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
echo "  Kernel:      $(uname -r)"
echo ""

echo "Development Tools:"
echo "  Rust:        $(rustc --version 2>/dev/null || echo 'Not found')"
echo "  Cargo:       $(cargo --version 2>/dev/null || echo 'Not found')"
echo "  GCC:         $(gcc --version | head -1)"
echo "  pkg-config:  $(pkg-config --version)"
echo "  OpenSSL:     $(openssl version)"
echo ""

echo "Node.js Tools:"
echo "  Node:        $(node --version 2>/dev/null || echo 'Not found')"
echo "  npm:         $(npm --version 2>/dev/null || echo 'Not found')"
echo "  cleancss:    $(cleancss --version 2>/dev/null || echo 'Not found')"
echo "  terser:      $(terser --version 2>/dev/null || echo 'Not found')"
echo ""

echo "Image Tools:"
echo "  cwebp:       $(cwebp -version 2>&1 | head -1 || echo 'Not found')"
echo "  convert:     $(convert --version | head -1 || echo 'Not found')"
echo ""

# =============================================================================
# Build Test
# =============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

if [ -f "$PROJECT_DIR/contact-handler/Cargo.toml" ]; then
    log_info "Testing build..."
    cd "$PROJECT_DIR/contact-handler"

    if cargo build --release 2>/dev/null; then
        BINARY_SIZE=$(du -h target/release/scc-server | cut -f1)
        log_success "Build successful! Binary size: $BINARY_SIZE"
    else
        log_warning "Build test failed. Check error messages above."
    fi
else
    log_info "Project not found at $PROJECT_DIR - skipping build test"
fi

# =============================================================================
# Summary
# =============================================================================

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║${NC}  Development environment setup complete!                   ${GREEN}║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Next steps:"
echo "  1. cd contact-handler"
echo "  2. cargo build --release     # Build production binary"
echo "  3. cargo run --bin scc-dev   # Run development server"
echo "  4. cargo test                # Run tests"
echo ""
echo "Useful commands:"
echo "  cargo watch -x run           # Auto-rebuild on changes"
echo "  cargo clippy                 # Run linter"
echo "  cargo fmt                    # Format code"
echo ""

# Remind about PATH if needed
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo -e "${YELLOW}Note:${NC} Add this to your ~/.bashrc or ~/.zshrc:"
    echo '  export PATH="$HOME/.cargo/bin:$PATH"'
    echo ""
    echo "Or run: source ~/.cargo/env"
fi
