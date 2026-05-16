# Changelog

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
