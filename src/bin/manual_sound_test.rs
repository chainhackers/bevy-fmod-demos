//! Manual test for playing sounds using FMOD SDK example banks
//!
//! Controls:
//! - SPACE: Play/stop looping ambience
//! - 1: Play explosion sound
//! - 2: Play UI cancel sound
//! - ESC: Exit

use bevy::prelude::*;
use bevy_fmod::prelude::{AudioSource, FmodPlugin, FmodStudio, StopMode};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin::new(&[
                "./fmod/api/studio/examples/media/Master.bank",
                "./fmod/api/studio/examples/media/Master.strings.bank",
                "./fmod/api/studio/examples/media/SFX.bank",
                "./fmod/api/studio/examples/media/Music.bank",
            ]),
        ))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, setup_sounds)
        .add_systems(Update, handle_input)
        .run();
}

#[derive(Component)]
struct AmbienceSound;

#[derive(Component)]
struct ExplosionSound;

#[derive(Component)]
struct UISound;

#[derive(Resource)]
struct SoundState {
    ambience_playing: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.insert_resource(SoundState {
        ambience_playing: false,
    });

    // Display instructions
    commands.spawn((
        Text::new(
            "FMOD 2.03.09 Manual Sound Test\n\n\
            SPACE: Play/stop Country ambience\n\
            1: Play explosion\n\
            2: Play UI cancel sound\n\
            ESC: Exit\n\n\
            Status: Ready"
        ),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

fn setup_sounds(
    mut commands: Commands,
    studio: Res<FmodStudio>,
) {
    println!("Setting up FMOD events...");

    // Setup ambience sound (looping)
    match studio.get_event("event:/Ambience/Country") {
        Ok(event_desc) => {
            match event_desc.create_instance() {
                Ok(instance) => {
                    println!("✓ Created Country ambience instance");
                    commands.spawn((
                        AmbienceSound,
                        AudioSource {
                            event_instance: instance,
                            despawn_stop_mode: StopMode::AllowFadeout,
                        },
                    ));
                }
                Err(e) => println!("✗ Failed to create ambience instance: {:?}", e),
            }
        }
        Err(e) => println!("✗ Failed to get Ambience/Country event: {:?}", e),
    }

    println!("Sound setup complete\n");
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut sound_state: ResMut<SoundState>,
    ambience_query: Query<&AudioSource, With<AmbienceSound>>,
    mut commands: Commands,
    studio: Res<FmodStudio>,
    mut exit: EventWriter<AppExit>,
    mut text_query: Query<&mut Text>,
) {
    let mut status = String::from("Status: ");

    // Toggle ambience with SPACE
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok(ambience) = ambience_query.get_single() {
            if !sound_state.ambience_playing {
                match ambience.start() {
                    Ok(_) => {
                        println!("🎵 Started Country ambience");
                        sound_state.ambience_playing = true;
                        status.push_str("Playing ambience");
                    }
                    Err(e) => {
                        println!("✗ Failed to start ambience: {:?}", e);
                        status.push_str("Failed to start ambience");
                    }
                }
            } else {
                match ambience.stop(StopMode::AllowFadeout) {
                    Ok(_) => {
                        println!("⏹ Stopped Country ambience");
                        sound_state.ambience_playing = false;
                        status.push_str("Stopped ambience");
                    }
                    Err(e) => {
                        println!("✗ Failed to stop ambience: {:?}", e);
                        status.push_str("Failed to stop ambience");
                    }
                }
            }
        }
    }

    // Play explosion with 1
    if keyboard.just_pressed(KeyCode::Digit1) {
        match studio.get_event("event:/Weapons/Explosion") {
            Ok(event_desc) => {
                match event_desc.create_instance() {
                    Ok(instance) => {
                        // Start and release immediately (one-shot)
                        let _ = instance.start();
                        let _ = instance.release();
                        println!("💥 Played explosion");
                        status.push_str("Played explosion");
                    }
                    Err(e) => {
                        println!("✗ Failed to create explosion instance: {:?}", e);
                        status.push_str("Failed to create explosion");
                    }
                }
            }
            Err(e) => {
                println!("✗ Failed to get explosion event: {:?}", e);
                status.push_str("Explosion event not found");
            }
        }
    }

    // Play UI cancel with 2
    if keyboard.just_pressed(KeyCode::Digit2) {
        match studio.get_event("event:/UI/Cancel") {
            Ok(event_desc) => {
                match event_desc.create_instance() {
                    Ok(instance) => {
                        // Start and release immediately (one-shot)
                        let _ = instance.start();
                        let _ = instance.release();
                        println!("🔊 Played UI cancel");
                        status.push_str("Played UI cancel");
                    }
                    Err(e) => {
                        println!("✗ Failed to create UI instance: {:?}", e);
                        status.push_str("Failed to create UI sound");
                    }
                }
            }
            Err(e) => {
                println!("✗ Failed to get UI/Cancel event: {:?}", e);
                status.push_str("UI event not found");
            }
        }
    }

    // Update status if changed
    if status != "Status: " || sound_state.ambience_playing {
        if let Ok(mut text) = text_query.single_mut() {
            if sound_state.ambience_playing && status == "Status: " {
                status = String::from("Status: Ambience playing");
            } else if sound_state.ambience_playing {
                status = format!("{} (Ambience: ON)", status);
            }

            // Update the last line of the text
            let current = text.as_str();
            let lines = current.split('\n').collect::<Vec<_>>();
            let mut new_text = lines[..lines.len() - 1].join("\n");
            new_text.push_str("\n");
            new_text.push_str(&status);
            *text = Text::new(new_text);
        }
    }

    // Exit with ESC
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("Exiting...");
        exit.write(AppExit::Success);
    }
}