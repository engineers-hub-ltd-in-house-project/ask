#!/bin/bash
set -e

# Ask CLI installer script
# Usage: curl -fsSL https://raw.githubusercontent.com/engineers-hub/ask/main/scripts/install.sh | bash

REPO="engineers-hub/ask"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
TMPDIR="${TMPDIR:-/tmp}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
log() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect platform
detect_platform() {
    local os="$(uname -s)"
    local arch="$(uname -m)"
    
    case "$os" in
        Linux*)
            case "$arch" in
                x86_64) echo "linux-x86_64" ;;
                aarch64|arm64) echo "linux-aarch64" ;;
                *) error "Unsupported architecture: $arch" ;;
            esac
            ;;
        Darwin*)
            case "$arch" in
                x86_64) echo "macos-x86_64" ;;
                arm64) echo "macos-aarch64" ;;
                *) error "Unsupported architecture: $arch" ;;
            esac
            ;;
        MINGW*|MSYS*|CYGWIN*)
            case "$arch" in
                x86_64) echo "windows-x86_64.exe" ;;
                aarch64|arm64) echo "windows-aarch64.exe" ;;
                *) error "Unsupported architecture: $arch" ;;
            esac
            ;;
        *)
            error "Unsupported operating system: $os"
            ;;
    esac
}

# Get latest release info
get_latest_release() {
    log "Fetching latest release information..."
    
    local api_url="https://api.github.com/repos/$REPO/releases/latest"
    
    if command -v curl >/dev/null 2>&1; then
        curl -s "$api_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$api_url" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
    else
        error "Neither curl nor wget is available"
    fi
}

# Download and install
install_ask() {
    local platform="$(detect_platform)"
    local version="$(get_latest_release)"
    
    if [ -z "$version" ]; then
        error "Could not determine latest version"
    fi
    
    log "Installing Ask CLI $version for $platform..."
    
    local filename="ask-$platform"
    local download_url="https://github.com/$REPO/releases/download/$version/$filename.tar.gz"
    
    # Handle Windows executables
    if [[ "$platform" == *".exe" ]]; then
        filename="${filename%.*}" # Remove .exe from filename for download
        download_url="https://github.com/$REPO/releases/download/$version/$filename.zip"
    fi
    
    local tmp_file="$TMPDIR/ask-$version-$platform"
    
    log "Downloading from: $download_url"
    
    # Download
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$download_url" -o "$tmp_file"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$download_url" -O "$tmp_file"
    else
        error "Neither curl nor wget is available"
    fi
    
    # Extract and install
    local extract_dir="$TMPDIR/ask-extract-$$"
    mkdir -p "$extract_dir"
    
    if [[ "$download_url" == *".tar.gz" ]]; then
        tar -xzf "$tmp_file" -C "$extract_dir"
    elif [[ "$download_url" == *".zip" ]]; then
        if command -v unzip >/dev/null 2>&1; then
            unzip -q "$tmp_file" -d "$extract_dir"
        else
            error "unzip is not available"
        fi
    fi
    
    # Find the binary
    local binary_path=""
    if [[ "$platform" == *".exe" ]]; then
        binary_path="$(find "$extract_dir" -name "ask.exe" | head -1)"
    else
        binary_path="$(find "$extract_dir" -name "ask" -type f | head -1)"
    fi
    
    if [ ! -f "$binary_path" ]; then
        error "Could not find ask binary in downloaded archive"
    fi
    
    # Install
    if [ -w "$INSTALL_DIR" ]; then
        cp "$binary_path" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/ask"
    else
        log "Installing to $INSTALL_DIR requires sudo..."
        sudo cp "$binary_path" "$INSTALL_DIR/"
        sudo chmod +x "$INSTALL_DIR/ask"
    fi
    
    # Cleanup
    rm -f "$tmp_file"
    rm -rf "$extract_dir"
    
    success "Ask CLI $version installed successfully!"
    
    # Verify installation
    if command -v ask >/dev/null 2>&1; then
        log "Verifying installation..."
        ask --version
        echo
        success "Installation complete! 🎉"
        echo
        echo -e "${CYAN}Quick start:${NC}"
        echo "1. Set your API key: ${YELLOW}ask config set-key YOUR_API_KEY${NC}"
        echo "2. Ask a question: ${YELLOW}ask \"What is Rust?\"${NC}"
        echo "3. Get help: ${YELLOW}ask --help${NC}"
    else
        warn "Installation completed, but 'ask' command not found in PATH"
        warn "You may need to add $INSTALL_DIR to your PATH or restart your shell"
    fi
}

# Main execution
main() {
    echo -e "${CYAN}"
    echo "  █████╗ ███████╗██╗  ██╗"
    echo " ██╔══██╗██╔════╝██║ ██╔╝"
    echo " ███████║███████╗█████╔╝ "
    echo " ██╔══██║╚════██║██╔═██╗ "
    echo " ██║  ██║███████║██║  ██╗"
    echo " ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝"
    echo -e "${NC}"
    echo "High-Performance Claude AI CLI"
    echo
    
    # Check dependencies
    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        error "This installer requires curl or wget"
    fi
    
    if ! command -v tar >/dev/null 2>&1; then
        error "This installer requires tar"
    fi
    
    install_ask
}

# Run main function
main "$@" 