#!/bin/bash

# Run script for bevy-fmod-demos
# Usage: ./run_demos.sh [demo_name]

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Detect the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check if FMOD SDK symlink is set up
if [ ! -e "fmod" ]; then
    echo -e "${YELLOW}Warning: FMOD SDK symlink not found${NC}"
    echo "Run ./setup_demos.sh first to set up environment"
    echo
    read -p "Run setup now? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        ./setup_demos.sh
    else
        echo -e "${RED}Cannot run demos without FMOD SDK${NC}"
        exit 1
    fi
fi

# Set FMOD SDK directory if not already set (needed for build time and runtime)
if [ -z "$FMOD_SDK_DIR" ]; then
    # Use absolute path to work correctly with cargo
    export FMOD_SDK_DIR="$(readlink -f fmod)"
fi

# Force bevy_fmod to rebuild with current FMOD_SDK_DIR
# This ensures the git dependency's build.rs runs with the correct environment
cargo clean -p bevy_fmod >/dev/null 2>&1 || true

# Check if FMOD SDK exists
if [ ! -d "$FMOD_SDK_DIR" ]; then
    echo -e "${RED}Error: FMOD SDK not found at $FMOD_SDK_DIR${NC}"
    echo "Run ./setup_demos.sh for more information."
    exit 1
fi

# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "aarch64" ]; then
    LIB_ARCH="arm64"
else
    LIB_ARCH="x86_64"
fi

# Set library paths
FMOD_CORE_LIB="$FMOD_SDK_DIR/api/core/lib/$LIB_ARCH"
FMOD_STUDIO_LIB="$FMOD_SDK_DIR/api/studio/lib/$LIB_ARCH"

# Verify library directories exist
if [ ! -d "$FMOD_CORE_LIB" ] || [ ! -d "$FMOD_STUDIO_LIB" ]; then
    echo -e "${RED}Error: FMOD libraries not found${NC}"
    echo "Expected core lib: $FMOD_CORE_LIB"
    echo "Expected studio lib: $FMOD_STUDIO_LIB"
    exit 1
fi

# Verify actual library files exist
if [ ! -f "$FMOD_CORE_LIB/libfmod.so.14" ] || [ ! -f "$FMOD_STUDIO_LIB/libfmodstudio.so.14" ]; then
    echo -e "${RED}Error: FMOD library files not found${NC}"
    echo "Expected files:"
    echo "  $FMOD_CORE_LIB/libfmod.so.14"
    echo "  $FMOD_STUDIO_LIB/libfmodstudio.so.14"
    exit 1
fi

# Set runtime library path
export LD_LIBRARY_PATH="$FMOD_CORE_LIB:$FMOD_STUDIO_LIB:$LD_LIBRARY_PATH"

# Get demo name from argument
DEMO=$1

# Show help if no argument provided
if [ -z "$DEMO" ]; then
    echo -e "${GREEN}=== bevy-fmod-demos Runner ===${NC}"
    echo
    echo "Usage: ./run_demos.sh <demo_name>"
    echo
    echo "Available demos:"
    echo "  verify_fmod         - Verify FMOD library installation"
    echo "  minimal             - Basic sound playback"
    echo "  spatial             - 3D positioned audio with arrow key controls"
    echo "  parameters          - Event parameter control"
    echo "  audio_control       - Audio control demonstration"
    echo "  simple_sound_test   - Non-interactive playback test"
    echo "  manual_sound_test   - Interactive keyboard test"
    echo
    echo "Examples:"
    echo "  ./run_demos.sh verify_fmod"
    echo "  ./run_demos.sh minimal"
    echo "  ./run_demos.sh spatial"
    echo
    exit 0
fi

# Verify FMOD works before running demos (skip for verify_fmod itself)
if [ "$DEMO" != "verify_fmod" ]; then
    echo -e "${GREEN}bevy-fmod-demos Runner${NC}"
    echo "================================"
    echo -e "${GREEN}FMOD SDK:${NC} $FMOD_SDK_DIR"
    echo -e "${GREEN}Architecture:${NC} $LIB_ARCH"
    echo ""

    echo -e "${YELLOW}Verifying FMOD...${NC}"
    cargo run --example verify_fmod 2>&1 | grep -v "warning:"

    if [ ${PIPESTATUS[0]} -ne 0 ]; then
        echo -e "${RED}Error: FMOD verification failed${NC}"
        echo "FMOD libraries may not be properly installed or accessible."
        exit 1
    fi

    echo ""
fi

# Run the demo
echo -e "${GREEN}Running demo: $DEMO${NC}"

if [ "$DEMO" = "verify_fmod" ]; then
    # verify_fmod is an example
    cargo run --example "$DEMO"
else
    # Other demos are binaries
    cargo run --bin "$DEMO"
fi
