//! Simple non-interactive test that plays a sound and exits
//! This verifies that FMOD 2.03.09 can actually play audio through the sound device

use bevy::prelude::*;
use bevy_fmod::prelude::{AudioSource, FmodPlugin, FmodStudio, StopMode};
use std::time::Duration;

fn main() {
    println!("\n🎵 FMOD Simple Sound Test");
    println!("=========================\n");

    App::new()
        .add_plugins((
            MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                Duration::from_millis(100),
            )),
            bevy::log::LogPlugin::default(),
            FmodPlugin::new(&[
                "./fmod/api/studio/examples/media/Master.bank",
                "./fmod/api/studio/examples/media/Master.strings.bank",
                "./fmod/api/studio/examples/media/SFX.bank",
            ]),
        ))
        .add_systems(Startup, play_test_sound)
        .add_systems(Update, check_completion)
        .run();
}

#[derive(Resource)]
struct TestTimer {
    timer: Timer,
}

fn play_test_sound(mut commands: Commands, studio: Res<FmodStudio>) {
    // Display FMOD version
    if let Ok(core) = studio.get_core_system() {
        if let Ok((version, build)) = core.get_version() {
            let major = (version >> 16) & 0xFF;
            let minor = (version >> 8) & 0xFF;
            let patch = version & 0xFF;
            println!(
                "✅ FMOD {}.{:02}.{:02} (build {})\n",
                major, minor, patch, build
            );
        }
    }

    println!("Attempting to play test sounds...\n");

    // Try to play the UI Cancel sound (should be quick)
    match studio.get_event("event:/UI/Cancel") {
        Ok(event_desc) => {
            match event_desc.create_instance() {
                Ok(instance) => {
                    match instance.start() {
                        Ok(_) => {
                            println!("✅ Successfully started UI/Cancel sound!");
                            println!("   You should hear a UI sound effect now.");
                        }
                        Err(e) => println!("❌ Failed to start sound: {:?}", e),
                    }
                    // Release after starting (one-shot)
                    let _ = instance.release();
                }
                Err(e) => println!("❌ Failed to create instance: {:?}", e),
            }
        }
        Err(e) => println!("❌ Failed to get UI/Cancel event: {:?}", e),
    }

    // Also try the explosion sound
    match studio.get_event("event:/Weapons/Explosion") {
        Ok(event_desc) => match event_desc.create_instance() {
            Ok(instance) => {
                match instance.start() {
                    Ok(_) => {
                        println!("✅ Successfully started Explosion sound!");
                        println!("   You should hear an explosion sound effect.");
                    }
                    Err(e) => println!("❌ Failed to start explosion: {:?}", e),
                }
                let _ = instance.release();
            }
            Err(e) => println!("❌ Failed to create explosion instance: {:?}", e),
        },
        Err(e) => println!("❌ Failed to get Weapons/Explosion event: {:?}", e),
    }

    // Set timer to exit after sounds play
    commands.insert_resource(TestTimer {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
    });

    println!("\nWaiting 3 seconds for sounds to finish...");
}

fn check_completion(time: Res<Time>, mut timer: ResMut<TestTimer>, mut exit: EventWriter<AppExit>) {
    timer.timer.tick(time.delta());

    if timer.timer.finished() {
        println!("\n✅ Test completed");
        println!("If you heard sound effects, FMOD is working correctly!\n");
        exit.write(AppExit::Success);
    }
}
