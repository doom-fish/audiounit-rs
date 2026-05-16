# audiounit-rs

Safe Rust bindings for Apple's **`AudioUnit`** / **`AVFAudio`** APIs on macOS.

> **Status:** v0.2.0 extends the Swift bridge and safe Rust API across `AUAudioUnit`, `AUAudioUnitBus`, `AUAudioUnitBusArray`, `AUParameter`, `AUParameterTree`, `AUParameterGroup`, `AUAudioUnitFactory`, `AUAudioUnitV2Bridge`, `AVAudioUnitEffect`, `AVAudioUnitGenerator`, `AVAudioUnitMIDIInstrument`, `AUVoiceIO`, plus an `AVAudioUnitInstrument` compatibility alias.

## Quick start

```rust,no_run
use audiounit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let components = ComponentManager::components_matching(AudioComponentDescription::any())?;
    println!("installed components: {}", components.len());

    let unit = AuAudioUnit::instantiate(
        AudioComponentDescription::apple(
            AUDIO_UNIT_TYPE_OUTPUT,
            AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT,
        ),
        InstantiationOptions::InProcess,
    )?;

    let info = unit.info()?;
    println!("AUAudioUnit: {:?}", info.audio_unit_name);

    if let Some(tree) = unit.parameter_tree() {
        println!("parameter tree bytes: {}", tree.to_json().len());
    }

    let effect = AvAudioUnitEffect::new(AudioComponentDescription::apple(
        AUDIO_UNIT_TYPE_EFFECT,
        AUDIO_UNIT_SUBTYPE_PEAK_LIMITER,
    ))?;
    println!("effect bypass: {}", effect.bypass());
    Ok(())
}
```

## Highlights

### `AUAudioUnit` core
- `AuAudioUnit::instantiate(desc, options)` — synchronous wrapper around `AUAudioUnit.instantiate`.
- `AuAudioUnit::info()` — snapshot component metadata, render flags, presets, channel maps, MIDI protocol hints, and bus counts.
- `allocate_render_resources`, `deallocate_render_resources`, `reset`, `parameters_for_overview`, `set_context_name`, `set_current_preset`, `set_channel_map`.
- `input_busses()` / `output_busses()` returning typed `AuAudioUnitBusArray` handles.

### Busses and formats
- `AuAudioUnitBusArray` — enumerate bus counts, inspect array metadata, and change bus count where the host AU allows it.
- `AuAudioUnitBus` — inspect `AVAudioFormat` snapshots, `should_allocate_buffer`, `enabled`, `name`, supported channel-layout tags, and context latency.

### Parameters
- `AuParameterTree::info()` / `to_json()` — full recursive tree snapshots.
- `AuParameterTree::parameter_with_address` and `parameter_with_id` for AUv2 parameters.
- `AuParameterGroup::children()` / `all_parameters()`.
- `AuParameter` — identifier/display-name access, value get/set, host-time setters, and string conversion helpers.

### Factories, bridges, and voice I/O
- `AuAudioUnitFactory` — bridge-backed helper mirroring `AUAudioUnitFactory` creation semantics.
- `AuAudioUnitV2Bridge` — cast modern units back to their underlying v2 `AudioUnit` handles when available.
- `AuVoiceIo` / `AuVoiceIO` — instantiate `kAudioUnitSubType_VoiceProcessingIO` and control bypass, AGC, mute, and ducking configuration.

### `AVAudioUnit` subclasses
- `AvAudioUnit` — generic synchronous instantiation, metadata snapshots, raw `AudioUnit` access, and preset-loading stub reporting.
- `AvAudioUnitEffect` — create AUv2 effects and toggle `bypass`.
- `AvAudioUnitGenerator` — create AUv2 generators and toggle `bypass`.
- `AvAudioUnitMidiInstrument` / `AvAudioUnitMIDIInstrument` — create Apple music devices and send note/controller/program/SysEx events.
- `AvAudioUnitInstrument` — compatibility alias to the public `AvAudioUnitMidiInstrument` surface because current macOS SDKs do not expose a separate public `AVAudioUnitInstrument` class.

### Legacy C API + enumeration
- `ComponentManager::components_matching(AudioComponentDescription)` — wraps `AVAudioUnitComponentManager`.
- `AudioUnitComponent` — `name`, `type_name`, `manufacturer_name`, `version`, `version_string`, `has_custom_view`, `is_sandbox_safe`, `audio_component_description`, `tags`.
- Legacy raw C helpers remain available: `audio_unit_get_property_info`, `audio_unit_get_property`, `audio_unit_set_property`, `audio_unit_get_parameter`, `audio_unit_set_parameter`, `audio_unit_set_render_callback`.

## Examples and tests

- `examples/01_list_components.rs` plus `02`–`14` cover every logical area above.
- `tests/*_tests.rs` provide smoke coverage for all newly-added areas.
- `COVERAGE.md` summarizes the audited logical-area surface for v0.2.0.

## Availability

macOS 13+ (matching the Swift bridge deployment target).

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
