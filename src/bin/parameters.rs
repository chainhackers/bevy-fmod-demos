//! This example demonstrates playing multiple ambience sounds simultaneously.
//!
//! The demo plays two ambience tracks from the FMOD SDK example banks:
//! - Forest ambience
//! - Country ambience
//!
//! Note: The FMOD SDK example banks don't include event parameters,
//! so this demo focuses on basic multi-track ambience playback.

use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

fn main() {
    println!("\n🎵 FMOD Ambience Demo");
    println!("====================\n");

    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin::new(&[
                "./fmod/api/studio/examples/media/Master.bank",
                "./fmod/api/studio/examples/media/Master.strings.bank",
                "./fmod/api/studio/examples/media/SFX.bank",
            ]),
        ))
        .add_systems(Startup, (startup, display_info))
        .add_systems(PostStartup, play_ambience)
        .run();
}

#[derive(Component)]
struct ForestSfxPlayer;

#[derive(Component)]
struct CountrySfxPlayer;

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

    println!("Loading events...");

    // Load Forest ambience event
    let forest_event = match studio.get_event("event:/Ambience/Forest") {
        Ok(event) => {
            println!("✅ Loaded event: Ambience/Forest");
            event
        }
        Err(e) => {
            println!("❌ Failed to load Forest event: {:?}", e);
            panic!("Cannot continue without Forest event");
        }
    };

    let forest_instance = match forest_event.create_instance() {
        Ok(instance) => instance,
        Err(e) => {
            println!("❌ Failed to create Forest instance: {:?}", e);
            panic!("Cannot create Forest instance");
        }
    };

    commands.spawn(ForestSfxPlayer).insert(AudioSource {
        event_instance: forest_instance,
        despawn_stop_mode: StopMode::AllowFadeout,
    });

    // Load Country ambience event
    let country_event = match studio.get_event("event:/Ambience/Country") {
        Ok(event) => {
            println!("✅ Loaded event: Ambience/Country");
            event
        }
        Err(e) => {
            println!("❌ Failed to load Country event: {:?}", e);
            panic!("Cannot continue without Country event");
        }
    };

    let country_instance = match country_event.create_instance() {
        Ok(instance) => instance,
        Err(e) => {
            println!("❌ Failed to create Country instance: {:?}", e);
            panic!("Cannot create Country instance");
        }
    };

    commands.spawn(CountrySfxPlayer).insert(AudioSource {
        event_instance: country_instance,
        despawn_stop_mode: StopMode::AllowFadeout,
    });

    // Add camera for UI rendering
    commands.spawn(Camera2d);
}

fn play_ambience(audio_sources: Query<&AudioSource>) {
    println!("\n▶️  Starting ambience playback...");
    for audio_source in audio_sources.iter() {
        match audio_source.start() {
            Ok(_) => {}
            Err(e) => println!("❌ Failed to start audio: {:?}", e),
        }
    }
    println!("✅ Playing Forest and Country ambience sounds");
    println!("\nYou should hear layered ambient sounds.");
    println!("Press Ctrl+C or close window to exit.\n");
}

fn display_info(mut commands: Commands) {
    commands.spawn(Text::default()).with_children(|parent| {
        parent.spawn(TextSpan::new("[*] FMOD Ambience Demo\n\n"));
        parent.spawn(TextSpan::new("This demo plays multiple ambience tracks:\n"));
        parent.spawn(TextSpan::new("- Forest ambience\n"));
        parent.spawn(TextSpan::new("- Country ambience\n\n"));
        parent.spawn(TextSpan::new("Both sounds play simultaneously,\n"));
        parent.spawn(TextSpan::new("demonstrating multi-track audio playback."));
    });
}
