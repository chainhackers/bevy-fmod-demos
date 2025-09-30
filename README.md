# bevy-fmod-demos

Demo applications showcasing FMOD audio integration in Bevy game engine using bevy_fmod.

## Setup

1. Download FMOD Studio API 2.03.09 from [FMOD Downloads](https://www.fmod.com/download)
2. Extract: `tar -xzf ~/Downloads/fmodstudioapi20309linux.tar.gz -C fmod --strip-components=1`
3. Set environment:
```bash
export FMOD_SDK_DIR=$(pwd)/fmod
export LD_LIBRARY_PATH=$FMOD_SDK_DIR/api/core/lib/x86_64:$FMOD_SDK_DIR/api/studio/lib/x86_64:$LD_LIBRARY_PATH
```

## Running Demos

```bash
cargo run --bin minimal           # Basic sound playback
cargo run --bin spatial           # 3D positioned audio
cargo run --bin parameters        # Event parameter control
cargo run --bin simple_sound_test # Non-interactive test
cargo run --bin manual_sound_test # Interactive test (SPACE, 1-2, ESC keys)
```

## Requirements

- Rust 1.70+
- FMOD Studio API 2.03.09
- Bevy 0.16
