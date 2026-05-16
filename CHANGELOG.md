# Changelog

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
