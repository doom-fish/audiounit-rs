# audiounit-rs

Safe Rust bindings for Apple's **`AudioUnit`** / **`AVFAudio`** APIs on macOS.

> **Status:** v0.4.0 makes the render-thread captures lock-free and bounded with owning tokens, lets `render` pull input for effects, makes instantiation timeouts configurable, and serializes audio unit initialization. See `CHANGELOG.md` for the breaking changes.

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
- `AuAudioUnit::instantiate(desc, options)` — synchronous wrapper around `AUAudioUnit.instantiate`; `instantiate_with_timeout` takes the wait limit as a `Duration` (`DEFAULT_INSTANTIATION_TIMEOUT` is 15 s). `AvAudioUnit::instantiate_with_timeout` and `AuVoiceIo::new_with_timeout` do the same.
- `AuAudioUnit::info()` — snapshot component metadata, render flags, presets, channel maps, MIDI protocol hints, and bus counts.
- `allocate_render_resources`, `deallocate_render_resources`, `reset`, `parameters_for_overview`, `set_context_name`, `set_current_preset`, and `set_channel_map`.
- `component_ptr`, unsafe `render` with an optional pull-input callback, `schedule_parameter`, bounded render-observer capture, MIDI scheduling and output capture, host musical/transport context helpers, MIDI-CI profile helpers, `message_channel`, `set_device_id`, `start_hardware`, and `stop_hardware`.
- `input_busses()` / `output_busses()` returning typed `AuAudioUnitBusArray` handles.

### Busses and formats
- `AuAudioUnitBusArray` — enumerate bus counts, inspect array metadata, and change bus count where the host AU allows it.
- `AuAudioUnitBus` — inspect `AVAudioFormat` snapshots, `should_allocate_buffer`, `enabled`, `name`, supported channel-layout tags, and context latency.

### Parameters
- `AuParameterTree::info()` / `to_json()` — full recursive tree snapshots.
- `AuParameterTree::parameter_with_address` and `parameter_with_id` for `AUv2` parameters.
- `AuParameterTree` also exposes bounded parameter, recording, and automation observer captures; each returns an owning token that unregisters on drop.
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

## Rendering

`AuAudioUnit::render` calls the unit's `renderBlock`. Effects pull their input through the optional `AuPullInput` callback, so set the input bus format and enable the bus (`AuAudioUnitBus::set_enabled(true)`) before `allocate_render_resources`. `examples/17_render_effect.rs` renders a tone through the delay effect.

## Captures on the render thread

`add_render_observer_capture`, `add_midi_output_capture`, `add_midi_output_event_list_capture`, and the `AuParameterTree::add_parameter_*_capture` methods take a capacity from 1 to `CAPTURE_CAPACITY_MAX` (65,536 events). Each returns a token that owns a lock-free ring. The render thread copies plain values into the ring without allocating or locking. A full ring drops its oldest event and `dropped_events()` counts the drops. `take_events()` drains the ring, and dropping the token unregisters the observer.

- MIDI-output captures chain the `MIDIOutputEventBlock` / `MIDIOutputEventListBlock` that was installed when the first capture was added, and restore it when the last capture is dropped. Units read these blocks when render resources are allocated, so add captures before `allocate_render_resources`.
- MIDI output events keep the first `MIDI_OUTPUT_INLINE_BYTES` (128) bytes and set `truncated` for longer messages. Event-list captures record only the protocol and packet count.
- `AUAudioUnit` keeps removed render-observer blocks until the unit is deallocated. A dropped token gets no further events and its ring is freed; only a small context stays alive until then.
- The framework delivers parameter-observer notifications asynchronously, so events arrive shortly after the change.

## Threading

Apple's DLS synth crashes when several threads initialize it at once. The crate therefore serializes the following behind one process-wide lock: allocating and deallocating render resources, the legacy `AudioComponentInstanceNew`/`Dispose` and `AudioUnitInitialize`/`Uninitialize` helpers, sampler instrument and preset loading, and the final release of every handle that owns an audio unit. The lock belongs to this crate, so the same calls made through another crate in the same process are not serialized with it.

File paths are passed to the framework byte for byte, so non-UTF-8 paths are not rewritten.

## Not wrapped

- `AUAudioUnit.outputProvider` and `inputHandler`. Drive I/O units with the legacy render callback (`legacy::audio_unit_set_render_callback`) instead.
- `saveUserPreset`, `deleteUserPreset`, and `presetStateFor` (user presets are readable through `AuAudioUnitInfo::user_presets`).
- `requestViewController` (`CoreAudioKit` view controllers).
- Implementing audio units in Rust (`AUAudioUnit` subclasses and `internalRenderBlock`).

## Async capture streams

Enable the `async` feature for executor-agnostic streams. Each stream owns a capture token and drains its ring on a lightweight polling thread, so the render thread never runs executor code.

```toml
[dependencies]
audiounit = { version = "0.4", features = ["async"] }
```

```rust,no_run
use audiounit::prelude::*;

# async fn run() -> Result<(), audiounit::AuError> {
let unit = AuAudioUnit::instantiate(
    AudioComponentDescription::apple(
        AUDIO_UNIT_TYPE_OUTPUT,
        AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT,
    ),
    InstantiationOptions::InProcess,
)?;
let stream = unit.render_observer_stream(128)?;

while let Some(event) = stream.next().await {
    println!("{:?}", event?);
}
# Ok(())
# }
```

## Examples and tests

- `examples/01_list_components.rs` plus `02`–`17` cover every logical area above.
- `tests/*_tests.rs` cover each area, including effect rendering with pull input, bounded captures, parallel initialization, and instantiation timeouts.
- `COVERAGE.md` summarizes the logical-area surface. `COVERAGE_AUDIT.md` scores a hand-picked sample of SDK symbols; it is not a full enumeration of the frameworks.

## Availability

macOS 13+ (matching the Swift bridge deployment target).

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
