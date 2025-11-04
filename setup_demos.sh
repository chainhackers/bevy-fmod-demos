#!/bin/bash

# Setup script for bevy-fmod-demos
# Creates symlink to FMOD SDK and verifies bank files

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== bevy-fmod-demos Setup ===${NC}"
echo

# Detect the script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo -e "${YELLOW}Checking for bevy_fmod repository...${NC}"

# Check if bevy_fmod exists in sibling directory
BEVY_FMOD_DIR="../bevy_fmod"
if [ ! -d "$BEVY_FMOD_DIR" ]; then
    echo -e "${RED}✗ bevy_fmod repository not found${NC}"
    echo
    echo "Expected location: $BEVY_FMOD_DIR"
    echo
    echo "bevy-fmod-demos requires bevy_fmod to be checked out in a sibling directory."
    echo "Please ensure the repository structure:"
    echo
    echo "  parent/"
    echo "    ├── bevy_fmod/"
    echo "    └── bevy-fmod-demos/"
    echo
    exit 1
fi

echo -e "${GREEN}✓ Found bevy_fmod repository${NC}"

# Check/create symlink to FMOD SDK
FMOD_SDK_LINK="fmod"
FMOD_SDK_TARGET="../bevy_fmod/fmod"

if [ -L "$FMOD_SDK_LINK" ]; then
    CURRENT_TARGET=$(readlink "$FMOD_SDK_LINK")
    if [ "$CURRENT_TARGET" = "$FMOD_SDK_TARGET" ]; then
        echo -e "${GREEN}✓ FMOD SDK symlink is correct${NC}"
    else
        echo -e "${YELLOW}⚠ Updating FMOD SDK symlink${NC}"
        rm "$FMOD_SDK_LINK"
        ln -s "$FMOD_SDK_TARGET" "$FMOD_SDK_LINK"
        echo -e "${GREEN}✓ Symlink updated${NC}"
    fi
elif [ -e "$FMOD_SDK_LINK" ]; then
    echo -e "${RED}✗ fmod exists but is not a symlink${NC}"
    echo "Please remove or rename it manually, then run this script again."
    exit 1
else
    echo -n "Creating fmod symlink... "
    ln -s "$FMOD_SDK_TARGET" "$FMOD_SDK_LINK"
    echo -e "${GREEN}✓${NC}"
fi

echo
echo -e "${YELLOW}Checking for FMOD Studio bank files...${NC}"

# Check if bank files exist
BANK_DIR="assets/audio/demo_project/Build/Desktop"
if [ ! -d "$BANK_DIR" ]; then
    mkdir -p "$BANK_DIR"
fi

REQUIRED_BANKS=("Master.bank" "Master.strings.bank" "Music.bank")
MISSING_BANKS=0

for bank in "${REQUIRED_BANKS[@]}"; do
    if [ -f "$BANK_DIR/$bank" ]; then
        echo -e "${GREEN}✓${NC} $bank"
    else
        echo -e "${RED}✗${NC} $bank (missing)"
        MISSING_BANKS=$((MISSING_BANKS + 1))
    fi
done

if [ $MISSING_BANKS -gt 0 ]; then
    echo
    echo -e "${RED}Missing $MISSING_BANKS required bank file(s)${NC}"
    echo
    echo -e "${YELLOW}To build FMOD Studio banks:${NC}"
    echo
    echo "1. Download FMOD Studio from: https://www.fmod.com/download"
    echo "   (Free account required)"
    echo
    echo "2. Install and launch FMOD Studio"
    echo
    echo "3. Open the built-in example project:"
    echo "   File > Open > Browse to: "
    echo "   Linux: ~/.fmod/FMOD Studio/Examples/GettingStarted.fspro"
    echo "   Or use: Help > Getting Started"
    echo
    echo "4. Build the banks:"
    echo "   File > Build"
    echo
    echo "5. Copy the built banks to this repository:"
    echo "   cp -r ~/.fmod/FMOD\\ Studio/Examples/Build/Desktop/* $BANK_DIR/"
    echo
    echo "Alternative: If you have banks from another source, copy them to:"
    echo "   $BANK_DIR/"
    echo
    read -p "Continue without banks? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
    echo -e "${YELLOW}Note: Demos will fail without valid bank files${NC}"
fi

echo
echo -e "${YELLOW}Checking for FMOD SDK...${NC}"

# Set default FMOD SDK path if not already set
if [ -z "$FMOD_SDK_DIR" ]; then
    if [ -d "$FMOD_SDK_LINK" ]; then
        export FMOD_SDK_DIR="$FMOD_SDK_LINK"
        echo -e "${YELLOW}Using FMOD SDK from symlink: $FMOD_SDK_DIR${NC}"
    fi
fi

# Check if FMOD SDK path is set
if [ -z "$FMOD_SDK_DIR" ]; then
    echo -e "${RED}❌ FMOD_SDK_DIR not set${NC}"
    echo
    echo "Please download FMOD Engine SDK from: https://www.fmod.com/download"
    echo "Then set the environment variable:"
    echo "  export FMOD_SDK_DIR=/path/to/fmod/sdk"
    echo
    echo "Or ensure the fmod symlink points to a valid FMOD SDK location."
else
    echo -e "${GREEN}✓ FMOD_SDK_DIR is set to: $FMOD_SDK_DIR${NC}"

    # Detect architecture
    ARCH=$(uname -m)
    if [ "$ARCH" = "aarch64" ]; then
        LIB_ARCH="arm64"
    else
        LIB_ARCH="x86_64"
    fi

    # Check if the libraries exist
    if [ -d "$FMOD_SDK_DIR/api/core/lib/$LIB_ARCH" ]; then
        echo -e "${GREEN}✓ FMOD libraries found ($LIB_ARCH)${NC}"
    else
        echo -e "${RED}✗ FMOD libraries not found at expected location${NC}"
        echo "  Expected: $FMOD_SDK_DIR/api/core/lib/$LIB_ARCH"
    fi
fi

echo
echo -e "${GREEN}Setup complete!${NC}"
echo
echo "To run demos:"
echo "  chmod +x run_demos.sh"
echo "  ./run_demos.sh <demo_name>"
echo
echo "Available demos:"
echo "  verify_fmod         - Verify FMOD library installation"
echo "  minimal             - Basic sound playback"
echo "  spatial             - 3D positioned audio with controls"
echo "  parameters          - Event parameter control"
echo "  audio_control       - Audio control demonstration"
echo
echo "Example:"
echo "  ./run_demos.sh verify_fmod"
echo "  ./run_demos.sh minimal"
