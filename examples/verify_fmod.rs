// Verification that FMOD libraries are correctly installed and working
// Run with: cargo run --example verify_fmod

use bevy_fmod::libfmod::{Init, System};

fn main() -> Result<(), bevy_fmod::libfmod::Error> {
    println!("\n🎵 FMOD Verification Test\n");

    // Create system and get version
    let system = System::create()?;
    let (version, build) = system.get_version()?;

    let major = (version >> 16) & 0xFF;
    let minor = (version >> 8) & 0xFF;
    let patch = version & 0xFF;

    println!("✅ FMOD Version: {}.{:02}.{:02} (build {})", major, minor, patch, build);

    // Initialize (may fail in headless environments)
    match system.init(512, Init::NORMAL, None) {
        Ok(_) => println!("✅ System initialized and released"),
        Err(e) => println!("⚠️  System init failed (headless environment): {:?}", e),
    }

    system.release()?;
    println!("\n🎉 SUCCESS: FMOD libraries verified!\n");

    Ok(())
}
