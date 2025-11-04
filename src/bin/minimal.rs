//! This example demonstrates how to use the FmodPlugin to play a sound.
//! Make sure to follow the instructions in the README.md to set up the demo project.

use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

fn main() {
    println!("\n🎵 Minimal FMOD Demo");
    println!("===================\n");

    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin::new(&[
                "./assets/audio/demo_project/Build/Desktop/Master.bank",
                "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                "./assets/audio/demo_project/Build/Desktop/Music.bank",
            ]),
        ))
        .add_systems(Startup, (startup, display_status))
        .add_systems(PostStartup, play_music)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) {
    // Display FMOD version
    if let Ok(core) = studio.get_core_system() {
        if let Ok((version, build)) = core.get_version() {
            let major = (version >> 16) & 0xFF;
            let minor = (version >> 8) & 0xFF;
            let patch = version & 0xFF;
            println!(
                "✅ FMOD {}.{:02}.{:02} (build {})",
                major, minor, patch, build
            );
        }
    }

    // Load event
    println!("Loading event: event:/Music/Level 03");
    let event_description = match studio.get_event("event:/Music/Level 03") {
        Ok(desc) => {
            println!("✅ Event loaded successfully");
            desc
        }
        Err(e) => {
            println!("❌ Failed to load event: {:?}", e);
            println!("\nPossible issues:");
            println!("  - Audio banks not loaded correctly");
            println!("  - Bank files missing or incorrect");
            println!("  - Event 'event:/Music/Level 03' doesn't exist in banks");
            println!("\nSee README.md for bank file setup instructions.");
            panic!("Cannot continue without audio event");
        }
    };

    // Create event instance
    let instance = match event_description.create_instance() {
        Ok(inst) => {
            println!("✅ Event instance created");
            inst
        }
        Err(e) => {
            println!("❌ Failed to create event instance: {:?}", e);
            panic!("Cannot create audio instance");
        }
    };

    commands.spawn(MyMusicPlayer).insert(AudioSource {
        event_instance: instance,
        despawn_stop_mode: StopMode::AllowFadeout,
    });

    // Add camera for UI rendering
    commands.spawn(Camera2d);
}

fn display_status(mut commands: Commands) {
    commands
        .spawn((
            Text::new("[*] Minimal FMOD Demo\n\n"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextSpan::new(
                "This demo plays background music using FMOD.\n\n",
            ));
            parent.spawn(TextSpan::new("Event: Music/Level 03\n"));
            parent.spawn(TextSpan::new(
                "You should hear music playing in a loop.\n\n",
            ));
            parent.spawn(TextSpan::new("Press Ctrl+C or close window to exit."));
        });
}

fn play_music(audio_source: Single<&AudioSource, With<MyMusicPlayer>>) {
    println!("\n▶️  Starting music playback...");
    match audio_source.start() {
        Ok(_) => {
            println!("✅ Music is now playing!");
            println!("\nThe demo is working correctly.");
            println!("You should hear music playing in a loop.");
            println!("Press Ctrl+C or close the window to exit.\n");
        }
        Err(e) => {
            println!("❌ Failed to start playback: {:?}", e);
            println!("Check your audio device and FMOD configuration.");
        }
    }
}
