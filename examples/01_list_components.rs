//! Enumerate installed Audio Unit components and print count/name/type/manufacturer.
//!
//! Run: `cargo run --example 01_list_components`

use audiounit::prelude::*;

fn fourcc_to_str(code: u32) -> String {
    let bytes = code.to_be_bytes();
    bytes
        .iter()
        .map(|&b| {
            if b.is_ascii_graphic() || b == b' ' {
                b as char
            } else {
                '.'
            }
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== audiounit: enumerate components ===\n");

    // Enumerate all components via AVAudioUnitComponentManager.
    let all = ComponentManager::components_matching(AudioComponentDescription::any())?;
    println!("total components: {}", all.len());

    // Print first 20 as a sample.
    let sample_count = all.len().min(20);
    println!("\n--- first {sample_count} components ---");
    for comp in all.iter().take(sample_count) {
        let desc = comp.audio_component_description();
        println!(
            "  {:.<40} type={} subtype={} mfr={} v={}",
            comp.name(),
            fourcc_to_str(desc.component_type),
            fourcc_to_str(desc.component_subtype),
            fourcc_to_str(desc.component_manufacturer),
            comp.version_string(),
        );
    }

    // Count by type.
    let effects_count = all
        .iter()
        .filter(|c| c.audio_component_description().component_type == AUDIO_UNIT_TYPE_EFFECT)
        .count();
    let instruments_count = all
        .iter()
        .filter(|c| c.audio_component_description().component_type == AUDIO_UNIT_TYPE_MUSIC_DEVICE)
        .count();
    let outputs_count = all
        .iter()
        .filter(|c| c.audio_component_description().component_type == AUDIO_UNIT_TYPE_OUTPUT)
        .count();

    println!("\n--- counts by type ---");
    println!("  Effects:     {effects_count}");
    println!("  Instruments: {instruments_count}");
    println!("  Outputs:     {outputs_count}");

    // --- Smoke: instantiate Apple's DefaultOutput unit ---
    println!("\n--- instantiating DefaultOutput unit ---");
    let output_desc = AudioComponentDescription::new(
        AUDIO_UNIT_TYPE_OUTPUT,
        AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT,
        AUDIO_UNIT_MANUFACTURER_APPLE,
    );
    match AvAudioUnit::instantiate(output_desc, InstantiationOptions::InProcess) {
        Ok(unit) => {
            println!("  instantiated DefaultOutput ✓");
            let au_ptr = unit.audio_unit_ptr();
            println!("  legacy AudioUnit ptr: {au_ptr:?}");

            // Inspect parameter tree.
            match unit.parameter_tree() {
                Some(tree) => {
                    let json = tree.to_json();
                    println!("  parameter tree JSON ({} bytes)", json.len());
                    // Print first 200 chars.
                    let preview = &json[..json.len().min(200)];
                    println!("  preview: {preview}…");
                }
                None => println!("  parameter tree: <none>"),
            }
        }
        Err(e) => println!("  instantiation error (expected in headless CI): {e}"),
    }

    println!("\n✅ audiounit enumerate OK");
    Ok(())
}
