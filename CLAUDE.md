# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`bevy-fmod-demos` is a collection of demo applications showcasing FMOD audio integration in the Bevy game engine using the `bevy_fmod` plugin. These demos demonstrate various FMOD features including 3D spatial audio, event parameters, and basic sound playback.

## Build and Run Commands

### Setup

First-time setup requires FMOD SDK and bank files:
```bash
./setup_demos.sh
```

This script:
- Verifies bevy_fmod repository exists in sibling directory
- Creates symlink to FMOD SDK (from `../bevy_fmod/fmod`)
- Checks for FMOD Studio bank files
- Verifies FMOD library files exist

### Running Demos

Use the run script for best experience:
```bash
./run_demos.sh verify_fmod  # Verify FMOD installation
./run_demos.sh minimal      # Run minimal demo
./run_demos.sh spatial      # Run spatial audio demo
./run_demos.sh parameters   # Run parameters demo
```

The run script automatically:
- Sets FMOD library paths (LD_LIBRARY_PATH)
- Detects architecture (x86_64/arm64)
- Verifies FMOD libraries before running demos
- Provides helpful error messages

### Manual Running

```bash
# Set environment
export FMOD_SDK_DIR=fmod
export LD_LIBRARY_PATH=$FMOD_SDK_DIR/api/core/lib/x86_64:$FMOD_SDK_DIR/api/studio/lib/x86_64:$LD_LIBRARY_PATH

# Run demos
cargo run --example verify_fmod
cargo run --bin minimal
cargo run --bin spatial
```

## Architecture

### Demo Structure

Demos are organized as binary targets in `src/bin/`:
- `minimal.rs` - Basic sound playback
- `spatial.rs` - 3D positioned audio with interactive controls
- `parameters.rs` - Event parameter control
- `audio_control.rs` - Audio control demonstration
- `simple_sound_test.rs` - Non-interactive test
- `manual_sound_test.rs` - Interactive keyboard test

The `verify_fmod` example is in `examples/` for consistency with bevy_fmod.

### Dependencies

- **bevy_fmod**: Git dependency from `https://github.com/chainhackers/bevy_fmod.git`
- **bevy**: Version 0.17 with default features

### Bank Files

FMOD Studio bank files are located in:
```
assets/audio/demo_project/Build/Desktop/
  ├── Master.bank
  ├── Master.strings.bank
  └── Music.bank
```

These come from the FMOD Studio example project. Build them in FMOD Studio or copy from another source.

## Project Structure

```
bevy-fmod-demos/
├── assets/audio/demo_project/Build/Desktop/  # FMOD bank files
├── examples/verify_fmod.rs                   # FMOD verification
├── src/bin/                                  # Demo applications
├── setup_demos.sh                            # Setup script
├── run_demos.sh                              # Run script
├── Cargo.toml                                # Dependencies
└── README.md                                 # Documentation
```

## Relationship to Other Repositories

This repository depends on:
- **bevy_fmod**: Bevy plugin for FMOD integration (sibling directory)
- **FMOD SDK**: Audio engine libraries (via bevy_fmod/fmod symlink)

Related repositories:
- `libfmod`: Rust bindings for FMOD (dependency of bevy_fmod)
- `libfmod-demos`: Direct libfmod usage examples (no Bevy)

## Git Commit Guidelines

- Always add files individually using their full paths (never use `git add -A` or `git add -u`)
- Keep commit messages concise and on a single line
- Use Conventional Commits format (https://www.conventionalcommits.org/en/v1.0.0/)
    - Common prefixes: `feat:`, `fix:`, `docs:`, `style:`, `refactor:`, `test:`, `chore:`
    - For CI/CD and devops: use `chore(ci):` prefix
- Add issue/task number at the end of commit messages (e.g., `feat: add spatial audio demo #1`)
- Do NOT add attribution lines or Co-Authored-By to commits
- Do NOT use emoji in commit messages
- Avoid redundant adjectives in commit messages, test names, and documentation:
    - Don't use: "comprehensive", "robust", "sophisticated", "advanced", "complete", "critical", "important"
    - These words are redundant - demos should be thorough by default
    - Good names are specific: "spatial audio demo", "parameter control demo", "minimal playback"
- Focus on what changed, not how well it was done

## Common Tasks

### Adding a New Demo

1. Create new file in `src/bin/your_demo.rs`
2. Add `[[bin]]` section to Cargo.toml
3. Update README.md Available Demos section
4. Update setup_demos.sh help text
5. Update run_demos.sh help text
6. Test with `./run_demos.sh your_demo`

### Updating FMOD Version

1. Update bank files in `assets/audio/demo_project/`
2. Update README.md requirements
3. Verify with `./run_demos.sh verify_fmod`

## Notes

- FMOD libraries are NOT included due to licensing - users must download separately
- Bank files come from FMOD Studio example project
- Demos require audio device (won't work in headless environments)
- All demos use bevy_fmod plugin (not direct libfmod bindings)
