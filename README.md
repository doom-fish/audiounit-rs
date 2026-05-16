# audiounit-rs

Safe Rust bindings for Apple's **`AudioUnit`** / **`AVAudioUnit`** APIs on macOS.

> **Status:** v0.1.0 covers component enumeration, `AVAudioUnit` synchronous instantiation, `AUParameterTree` introspection, legacy `AudioUnit` C property/parameter wrappers, and `AudioComponentDescription` type/manufacturer constants.

## Quick start

```rust,no_run
use audiounit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enumerate all installed components.
    let components = ComponentManager::components_matching(AudioComponentDescription::any())?;
    println!("installed components: {}", components.len());

    for c in components.iter().take(5) {
        println!("  {} — {} / {}", c.name(), c.type_name(), c.manufacturer_name());
    }

    // Instantiate Apple's default output unit.
    let desc = AudioComponentDescription::new(
        AUDIO_UNIT_TYPE_OUTPUT,
        AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT,
        AUDIO_UNIT_MANUFACTURER_APPLE,
    );
    let unit = AvAudioUnit::instantiate(desc, InstantiationOptions::InProcess)?;
    println!("legacy AudioUnit ptr: {:?}", unit.audio_unit_ptr());

    if let Some(tree) = unit.parameter_tree() {
        println!("parameter tree: {}", tree.to_json());
    }
    Ok(())
}
```

## Highlights

### Component enumeration
- `ComponentManager::components_matching(AudioComponentDescription)` — wraps `AVAudioUnitComponentManager`
- `AudioUnitComponent` — `name`, `type_name`, `manufacturer_name`, `version`, `version_string`, `has_custom_view`, `is_sandbox_safe`, `audio_component_description`, `tags`

### Instantiation
- `AvAudioUnit::instantiate(desc, options)` — sync blocking wrapper around `+instantiateWithComponentDescription:options:completionHandler:`
- `AvAudioUnit::audio_unit_ptr()` — legacy `AudioUnit` raw handle
- `AvAudioUnit::au_audio_unit()` — modern `AUAudioUnit` handle

### Parameter tree
- `AvAudioUnit::parameter_tree()` → `AuParameterTree`
- `AuParameterTree::to_json()` — full tree as JSON
- `AuParameterTree::parameter_with_address(addr)` → `AuParameter`
- `AuParameter` — `identifier`, `display_name`, `address`, `min_value`, `max_value`, `unit`, `value`, `set_value`, `string_from_value`

### Legacy C API
- `audio_unit_get_property_info`, `audio_unit_get_property`, `audio_unit_set_property`
- `audio_unit_get_parameter`, `audio_unit_set_parameter`
- `audio_unit_set_render_callback` — registers an `AURenderCallback`
- `AURenderCallbackStruct`, `AudioStreamBasicDescription`, `AudioBufferList`

### Constants
- Type: `AUDIO_UNIT_TYPE_OUTPUT`, `MUSIC_DEVICE`, `MUSIC_EFFECT`, `EFFECT`, `MIXER`, `PANNER`, `GENERATOR`, `FORMAT_CONVERTER`, `OFFLINE_EFFECT`, `MIDI_PROCESSOR`
- Manufacturer: `AUDIO_UNIT_MANUFACTURER_APPLE`
- Subtypes: `DEFAULT_OUTPUT`, `SYSTEM_OUTPUT`, `HAL_OUTPUT`, `DLS_SYNTH`, `SAMPLER`, …

## Availability

macOS 13+ (due to Swift bridge platform requirement).

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
