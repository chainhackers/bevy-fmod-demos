# bevy-fmod-demos

Demo applications showcasing FMOD audio integration in Bevy game engine using bevy_fmod.

## Quick Start

The easiest way to run demos is using the provided helper scripts:

```bash
# First time setup
./setup_demos.sh

# Run demos
./run_demos.sh verify_fmod    # Verify FMOD installation
./run_demos.sh minimal         # Run minimal demo
./run_demos.sh spatial         # Run spatial audio demo
```

The `run_demos.sh` script automatically verifies FMOD libraries before running demos.

## Requirements

This repository requires:
- **bevy_fmod repository**: Must be in sibling directory (`../bevy_fmod`)
- **FMOD SDK**: Accessible via `../bevy_fmod/fmod` symlink (created by setup script)
- **Bank files**: FMOD Studio example project banks in `assets/audio/demo_project/Build/Desktop/`
- **Rust**: 1.70+
- **Bevy**: 0.17
- **FMOD**: 2.03.09

Directory structure:
```
parent/
  ├── bevy_fmod/           # bevy_fmod repository
  │   └── fmod/            # FMOD SDK (fmodstudioapi20310linux)
  └── bevy-fmod-demos/     # This repository
      └── assets/          # Bank files from FMOD Studio
```

## Available Demos

- **verify_fmod** - Verify FMOD library installation and version
- **minimal** - Basic sound playback
- **spatial** - 3D positioned audio with arrow key controls
- **parameters** - Event parameter control
- **audio_control** - Audio control demonstration
- **simple_sound_test** - Non-interactive playback test (auto-exits after 3s)
- **manual_sound_test** - Interactive keyboard test (SPACE, 1-2, ESC keys)

## Manual Setup

If you prefer to set up manually:

1. Download FMOD Studio API 2.03.09 from [FMOD Downloads](https://www.fmod.com/download)
2. Extract to `../bevy_fmod/fmod` or create symlink: `ln -s /path/to/fmod fmod`
3. Set environment:
```bash
export FMOD_SDK_DIR=$(pwd)/fmod
export LD_LIBRARY_PATH=$FMOD_SDK_DIR/api/core/lib/x86_64:$FMOD_SDK_DIR/api/studio/lib/x86_64:$LD_LIBRARY_PATH
```
4. Run demos:
```bash
cargo run --example verify_fmod
cargo run --bin minimal
cargo run --bin spatial
```

## Bank Files

Demo bank files are included in this repository (`assets/audio/demo_project/Build/Desktop/`). These banks must contain specific events from FMOD Studio's Getting Started example project:
- `event:/Music/Level 03` (used by minimal demo)
- Other events as required by each demo

**If demos hang or fail:**
The included bank files may not match the expected events. To rebuild with correct events:

1. Download FMOD Studio from [fmod.com](https://www.fmod.com/download) (free account required)
2. Open the Getting Started example: `Help > Getting Started` or `File > Open > ~/.fmod/FMOD Studio/Examples/GettingStarted.fspro`
3. Build banks: `File > Build`
4. Copy to repository: `cp -r ~/.fmod/FMOD\ Studio/Examples/Build/Desktop/* assets/audio/demo_project/Build/Desktop/`

## Troubleshooting

If demos fail to run:
1. Run `./setup_demos.sh` to verify setup and get bank file instructions
2. Run `./run_demos.sh verify_fmod` to check FMOD installation
3. Ensure bevy_fmod is in sibling directory
4. Verify bank files contain required events (see Bank Files section above)
