# Changelog

## [0.3.1] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.3.0] - 2026-05-20

### Added

- `async` feature plus `async_api` streams for `AUAudioUnit` render-observer capture, MIDI output capture, MIDI event-list capture, and `AUParameterTree` observer, recording, and automation capture surfaces.
- Ergonomic `render_observer_stream`, `midi_output_event_stream`, `midi_output_event_list_stream`, `parameter_observer_stream`, `parameter_recording_stream`, and `parameter_automation_stream` constructors on the owning wrapper types.

### Notes

- Phase 32 completeness + async sweep.

## [0.2.5] - 2026-05-20

- Added in-`src/` unit tests across `component_description`, `component`, and `error` (Tier 2 quality polish), providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [0.2.4] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.2.3] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## [0.2.2] - 2026-05-17

### Fixed

- Added `SAFETY` comments to all unsafe blocks in FFI boundary code (`util.rs` and `error.rs`) to document the correctness of pointer dereferencing and memory deallocation patterns.

## [0.2.1] - 2026-05-16

### Added

- Added advanced `AuAudioUnit` coverage for render invocation, parameter scheduling, render-observer capture, MIDI scheduling/output capture, host musical and transport callbacks, MIDI-CI profile helpers, message channels, and input/output hardware control.
- Added raw legacy `AudioComponent` / `AudioUnit` / `MusicDevice` lifecycle wrappers, including instance creation/disposal, start/stop, initialize/uninitialize, render, property listeners, parameter scheduling, and MIDI / `SysEx` helpers.
- Added capture-backed parameter observer, recording observer, and automation observer helpers on `AuParameterTree`.
- Added `ComponentManager` predicate/test enumeration plus richer `AudioUnitComponent` metadata APIs.
- Added `AvAudioUnitSampler`, `AvAudioUnitEQ`, `AvAudioUnitDelay`, `AvAudioUnitDistortion`, `AvAudioUnitReverb`, `AvAudioUnitTimeEffect`, `AvAudioUnitTimePitch`, and `AvAudioUnitVarispeed`.
- Added `examples/15_av_audio_unit_components.rs`, `examples/16_au_audio_unit_advanced.rs`, `tests/av_audio_unit_components_tests.rs`, `tests/av_audio_unit_specialized_tests.rs`, `tests/legacy_audio_unit_tests.rs`, and `tests/au_audio_unit_advanced_tests.rs`.

### Changed

- Upgraded the crate to v0.2.1.
- Implemented real `AvAudioUnit::load_audio_unit_preset` bridging.
- Updated `COVERAGE_AUDIT.md` to 168 verified symbols, 5 exempt symbols, and 100% non-exempt coverage.

## [0.2.0] - 2026-05-16

### Added

- Expanded the Swift bridge into logical-area files for `AUAudioUnit`, buses, parameters, factory/V2 bridge helpers, `AVAudioUnit` subclasses, and `AUVoiceIO`.
- Added safe Rust modules for `AuAudioUnit`, `AuAudioUnitBus`, `AuAudioUnitBusArray`, `AuParameter`, `AuParameterTree`, `AuParameterGroup`, `AuAudioUnitFactory`, `AuAudioUnitV2Bridge`, `AvAudioUnitEffect`, `AvAudioUnitGenerator`, `AvAudioUnitMidiInstrument`, and `AuVoiceIo`.
- Added `AvAudioUnitInstrument` as a compatibility alias to the public `AvAudioUnitMidiInstrument` surface for the logical instrument area.
- Added 13 new numbered examples (`02`–`14`) covering every requested logical area.
- Added area-specific integration smoke tests under `tests/`.
- Added `COVERAGE.md` documenting the v0.2.0 logical-area audit.
- Added additional Audio Unit subtype constants (`VOICE_PROCESSING_IO`, `SPEECH_SYNTHESIS`, `AUDIO_FILE_PLAYER`, `SCHEDULED_SOUND_PLAYER`, `MIDI_SYNTH`, and related output subtypes).

### Changed

- Upgraded the crate to v0.2.0.
- Updated `build.rs` to follow the gold-standard Swift bridge build pattern more closely, including SDK detection and Swift runtime rpaths.
- Fixed `ComponentManager::components_matching` ownership so wrapped `AVAudioUnitComponent` handles remain valid after the bridge array is freed.

## [0.1.0] - Initial release

### Added

- `ComponentManager::components_matching` wrapping `AVAudioUnitComponentManager.sharedAudioUnitComponentManager`.
- `AudioUnitComponent` with `name`, `type_name`, `manufacturer_name`, `version`, `version_string`, `has_custom_view`, `is_sandbox_safe`, `audio_component_description`, `tags`.
- `AvAudioUnit::instantiate` — synchronous wrapper around `+instantiateWithComponentDescription:options:completionHandler:` using `DispatchSemaphore`.
- `AvAudioUnit::audio_unit_ptr` / `au_audio_unit` for legacy and modern handles.
- `AuParameterTree`, `AuParameter` — enumerate and get/set parameters via `AUAudioUnit.parameterTree`.
- `AuParameterTree::to_json` — full tree serialized as JSON.
- Legacy C API wrappers: `audio_unit_get_property_info`, `audio_unit_get_property`, `audio_unit_set_property`, `audio_unit_get_parameter`, `audio_unit_set_parameter`, `audio_unit_set_render_callback`.
- `AudioStreamBasicDescription`, `AURenderCallbackStruct`, `AudioBufferList`, `AudioBuffer`, `AudioTimeStamp` repr-C structs.
- `AudioComponentDescription` with `any()` / `new()` constructors.
- Type/manufacturer/subtype constants in `component_description::constants`.
- `examples/01_list_components` — smoke example.
