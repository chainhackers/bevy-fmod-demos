# bevy-fmod-demos

Demo applications showcasing FMOD audio integration in Bevy game engine using bevy_fmod.

## Quick Start

The easiest way to run demos is using the provided helper scripts:

```bash
# First time setup
./setup_demos.sh

# Run demos
./run_demos.sh verify_fmod         # Verify FMOD installation
./run_demos.sh minimal             # Basic sound playback
./run_demos.sh spatial             # 3D positioned audio
./run_demos.sh parameters          # Multi-track ambience
./run_demos.sh audio_control       # Audio playback controls
./run_demos.sh simple_sound_test   # Non-interactive test
./run_demos.sh manual_sound_test   # Interactive keyboard test
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
- **minimal** - Plays background music in a loop (grey window with status text)
- **spatial** - 3D audio positioning - cube orbits around player, move with arrow keys to hear volume/pan changes
- **parameters** - Multi-track ambience - plays Forest and Country ambience sounds simultaneously
- **audio_control** - Audio playback controls - Stop/Play/Toggle music with S/P/T keys
- **simple_sound_test** - Non-interactive test - plays two sounds and exits after 3 seconds
- **manual_sound_test** - Interactive keyboard test - play different sounds with SPACE/1/2 keys, ESC to exit

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

Demos use bank files from two locations:

### Music Demos (use repository banks)
These demos use banks from `assets/audio/demo_project/Build/Desktop/`:
- **minimal** - Uses Music.bank (event:/Music/Level 03)
- **spatial** - Uses Music.bank for 3D positioning
- **audio_control** - Uses Music.bank for playback control

The repository includes:
- Master.bank
- Master.strings.bank
- Music.bank

### SFX Demos (use FMOD SDK example banks)
These demos use banks from FMOD SDK at `fmod/api/studio/examples/media/` (accessed via symlink):
- **parameters** - Uses SFX.bank (event:/Ambience/Forest, event:/Ambience/Country)
- **simple_sound_test** - Uses SFX.bank (event:/UI/Cancel, event:/Weapons/Explosion)
- **manual_sound_test** - Uses SFX.bank and Music.bank

The FMOD SDK (downloaded during setup) includes all required banks:
- Master.bank, Master.strings.bank
- SFX.bank (21MB - contains ambience, weapons, UI sounds)
- Music.bank, Dialogue_*.bank, Vehicles.bank, VO.bank

**If music demos fail:**
Rebuild the banks from FMOD Studio Getting Started example:
1. Download FMOD Studio from [fmod.com](https://www.fmod.com/download)
2. Open: `Help > Getting Started` or `~/.fmod/FMOD Studio/Examples/GettingStarted.fspro`
3. Build: `File > Build`
4. Copy: `cp -r ~/.fmod/FMOD\ Studio/Examples/Build/Desktop/* assets/audio/demo_project/Build/Desktop/`

## Troubleshooting

If demos fail to run:
1. Run `./setup_demos.sh` to verify setup and get bank file instructions
2. Run `./run_demos.sh verify_fmod` to check FMOD installation
3. Ensure bevy_fmod is in sibling directory
4. Verify bank files contain required events (see Bank Files section above)
