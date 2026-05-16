# audiounit-rs

Safe Rust bindings for Apple's **`AudioUnit`** / **`AVFAudio`** APIs on macOS.

> **Status:** v0.2.1 closes the sampled SDK gaps with advanced `AUAudioUnit` render/MIDI/host-control helpers, capture-backed parameter observers, raw legacy `AudioUnit` / `MusicDevice` wrappers, richer component queries, and dedicated wrappers for `AVAudioUnitSampler`, `AVAudioUnitEQ`, `AVAudioUnitDelay`, `AVAudioUnitDistortion`, `AVAudioUnitReverb`, `AVAudioUnitTimeEffect`, `AVAudioUnitTimePitch`, and `AVAudioUnitVarispeed`.

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
- `allocate_render_resources`, `deallocate_render_resources`, `reset`, `parameters_for_overview`, `set_context_name`, `set_current_preset`, and `set_channel_map`.
- `component_ptr`, unsafe `render`, `schedule_parameter`, render-observer capture, MIDI scheduling/output capture, host musical/transport context helpers, MIDI-CI profile helpers, `message_channel`, `set_device_id`, `start_hardware`, and `stop_hardware`.
- `input_busses()` / `output_busses()` returning typed `AuAudioUnitBusArray` handles.

### Busses and formats
- `AuAudioUnitBusArray` — enumerate bus counts, inspect array metadata, and change bus count where the host AU allows it.
- `AuAudioUnitBus` — inspect `AVAudioFormat` snapshots, `should_allocate_buffer`, `enabled`, `name`, supported channel-layout tags, and context latency.

### Parameters
- `AuParameterTree::info()` / `to_json()` — full recursive tree snapshots.
- `AuParameterTree::parameter_with_address` and `parameter_with_id` for `AUv2` parameters.
- `AuParameterTree` also exposes capture-backed parameter, recording, and automation observer helpers with explicit observer tokens.
- `AuParameterGroup::children()` / `all_parameters()`.
- `AuParameter` — identifier/display-name access, value get/set, host-time setters, and string conversion helpers.

### Factories, bridges, and voice I/O
- `AuAudioUnitFactory` — bridge-backed helper mirroring `AUAudioUnitFactory` creation semantics.
- `AuAudioUnitV2Bridge` — cast modern units back to their underlying v2 `AudioUnit` handles when available.
- `AuVoiceIo` / `AuVoiceIO` — instantiate `kAudioUnitSubType_VoiceProcessingIO` and control bypass, AGC, mute, and ducking configuration.

### `AVAudioUnit` subclasses
- `AvAudioUnit` — generic synchronous instantiation, metadata snapshots, raw `AudioUnit` / `AUAudioUnit` access, and real preset loading.
- `ComponentManager::components_matching_predicate` / `components_passing_test` and `AudioUnitComponent::{user_tag_names, all_tag_names, available_architectures, configuration_dictionary, supports_number_input_channels}` cover the richer component-manager metadata surface.
- `AvAudioUnitEffect` and `AvAudioUnitGenerator` create `AUv2` effects/generators and toggle `bypass`.
- `AvAudioUnitSampler`, `AvAudioUnitEQ`, `AvAudioUnitDelay`, `AvAudioUnitDistortion`, `AvAudioUnitReverb`, `AvAudioUnitTimeEffect`, `AvAudioUnitTimePitch`, and `AvAudioUnitVarispeed` wrap the common concrete subclasses.
- `AvAudioUnitMidiInstrument` / `AvAudioUnitMIDIInstrument` — create Apple music devices and send note/controller/program/`SysEx` events.
- `AvAudioUnitInstrument` — compatibility alias to the public `AvAudioUnitMidiInstrument` surface because current macOS SDKs do not expose a separate public `AVAudioUnitInstrument` class.

### Legacy C API + enumeration
- `ComponentManager::components_matching(AudioComponentDescription)` — wraps `AVAudioUnitComponentManager`.
- `AudioUnitComponent` — `name`, `type_name`, `manufacturer_name`, `version`, `version_string`, `has_custom_view`, `is_sandbox_safe`, `audio_component_description`, tags, architectures, and configuration metadata.
- Legacy raw C helpers remain available for property/parameter access and render callbacks, plus direct `AudioComponentInstance*`, `AudioOutputUnit*`, `AudioUnitInitialize` / `AudioUnitRender` / `AudioUnitScheduleParameters`, property listeners, and `MusicDevice*` MIDI / note / `SysEx` entry points.

## Examples and tests

- `examples/01_list_components.rs` plus `02`–`16` cover every logical area above.
- `tests/*_tests.rs` provide smoke coverage for all newly-added areas.
- `COVERAGE.md` summarizes the logical-area surface and `COVERAGE_AUDIT.md` tracks the sampled 173-symbol SDK audit for v0.2.1.

## Availability

macOS 13+ (matching the Swift bridge deployment target).

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
