# audiounit-rs coverage audit (vs MacOSX26.2.sdk)

Scope note: `AudioUnit.framework` in MacOSX26.2.sdk is a thin umbrella that reexports `AudioToolbox.framework` headers. This audit treats those reexported `AudioToolbox` declarations as the `AudioUnit.framework` public surface.

Sampling note: the combined `AudioUnit.framework` + `AVFAudio.framework` AUAudioUnit-family surface is large, so this audit samples **173 high-signal public symbols** across modern `AUAudioUnit` / parameter APIs, legacy `AudioUnit` / `MusicDevice` control APIs, VoiceProcessing I/O properties, and the `AVAudioUnit` family.

SDK_PUBLIC_SYMBOLS: 173
VERIFIED: 168
GAPS: 0
EXEMPT: 5
COVERAGE_PCT: 100.00%

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `AudioComponentDescription` | struct | `AudioComponent.h` | `AudioComponentDescription` |
| `kAudioUnitType_Output` | enum constant | `AUComponent.h` | `component_description::constants::AUDIO_UNIT_TYPE_OUTPUT` |
| `kAudioUnitType_MusicDevice` | enum constant | `AUComponent.h` | `component_description::constants::AUDIO_UNIT_TYPE_MUSIC_DEVICE` |
| `kAudioUnitType_Effect` | enum constant | `AUComponent.h` | `component_description::constants::AUDIO_UNIT_TYPE_EFFECT` |
| `kAudioUnitType_Generator` | enum constant | `AUComponent.h` | `component_description::constants::AUDIO_UNIT_TYPE_GENERATOR` |
| `kAudioUnitManufacturer_Apple` | enum constant | `AUComponent.h` | `component_description::constants::AUDIO_UNIT_MANUFACTURER_APPLE` |
| `kAudioUnitSubType_DefaultOutput` | enum constant | `AUComponent.h` | `component_description::constants::AUDIO_UNIT_SUBTYPE_DEFAULT_OUTPUT` |
| `kAudioUnitSubType_VoiceProcessingIO` | enum constant | `AUComponent.h` | `component_description::constants::AUDIO_UNIT_SUBTYPE_VOICE_PROCESSING_IO` |
| `kAudioUnitSubType_PeakLimiter` | enum constant | `AUComponent.h` | `component_description::constants::AUDIO_UNIT_SUBTYPE_PEAK_LIMITER` |
| `AudioUnitGetPropertyInfo` | function | `AudioUnit.h` | `legacy::audio_unit_get_property_info` |
| `AudioUnitGetProperty` | function | `AudioUnit.h` | `legacy::audio_unit_get_property` |
| `AudioUnitSetProperty` | function | `AudioUnit.h` | `legacy::audio_unit_set_property` |
| `AudioUnitGetParameter` | function | `AudioUnit.h` | `legacy::audio_unit_get_parameter` |
| `AudioUnitSetParameter` | function | `AudioUnit.h` | `legacy::audio_unit_set_parameter` |
| `kAudioUnitProperty_StreamFormat` | enum constant | `AudioUnitProperties.h` | `legacy::property_id::STREAM_FORMAT` |
| `kAudioUnitProperty_SetRenderCallback` | enum constant | `AudioUnitProperties.h` | `legacy::property_id::SET_RENDER_CALLBACK and legacy::audio_unit_set_render_callback` |
| `kAUVoiceIOProperty_BypassVoiceProcessing` | enum constant | `AudioUnitProperties.h` | `AuVoiceIo::bypass_voice_processing and set_bypass_voice_processing` |
| `kAUVoiceIOProperty_VoiceProcessingEnableAGC` | enum constant | `AudioUnitProperties.h` | `AuVoiceIo::enable_agc and set_enable_agc` |
| `kAUVoiceIOProperty_MuteOutput` | enum constant | `AudioUnitProperties.h` | `AuVoiceIo::mute_output and set_mute_output` |
| `kAUVoiceIOProperty_OtherAudioDuckingConfiguration` | enum constant | `AudioUnitProperties.h` | `AuVoiceIo::other_audio_ducking_configuration and set_other_audio_ducking` |
| `AUAudioUnit` | class | `AUAudioUnit.h` | `AuAudioUnit` |
| `AUAudioUnit.instantiateWithComponentDescription:options:completionHandler:` | class method | `AUAudioUnit.h` | `AuAudioUnit::instantiate` |
| `AUAudioUnit.componentDescription` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::component_description` |
| `AUAudioUnit.componentName` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::component_name` |
| `AUAudioUnit.audioUnitName` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::audio_unit_name` |
| `AUAudioUnit.manufacturerName` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::manufacturer_name` |
| `AUAudioUnit.componentVersion` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::component_version` |
| `AUAudioUnit.allocateRenderResourcesAndReturnError:` | method | `AUAudioUnit.h` | `AuAudioUnit::allocate_render_resources` |
| `AUAudioUnit.deallocateRenderResources` | method | `AUAudioUnit.h` | `AuAudioUnit::deallocate_render_resources` |
| `AUAudioUnit.reset` | method | `AUAudioUnit.h` | `AuAudioUnit::reset` |
| `AUAudioUnit.inputBusses` | property | `AUAudioUnit.h` | `AuAudioUnit::input_busses` |
| `AUAudioUnit.outputBusses` | property | `AUAudioUnit.h` | `AuAudioUnit::output_busses` |
| `AUAudioUnit.maximumFramesToRender` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::maximum_frames_to_render and set_maximum_frames_to_render` |
| `AUAudioUnit.parameterTree` | property | `AUAudioUnit.h` | `AuAudioUnit::parameter_tree` |
| `AUAudioUnit.parametersForOverviewWithCount:` | method | `AUAudioUnit.h` | `AuAudioUnit::parameters_for_overview` |
| `AUAudioUnit.fullState` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::full_state_plist snapshot` |
| `AUAudioUnit.fullStateForDocument` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::full_state_for_document_plist snapshot` |
| `AUAudioUnit.factoryPresets` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::factory_presets` |
| `AUAudioUnit.userPresets` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::user_presets` |
| `AUAudioUnit.supportsUserPresets` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::supports_user_presets` |
| `AUAudioUnit.isLoadedInProcess` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::is_loaded_in_process` |
| `AUAudioUnit.currentPreset` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::current_preset and set_current_preset` |
| `AUAudioUnit.latency` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::latency` |
| `AUAudioUnit.tailTime` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::tail_time` |
| `AUAudioUnit.renderQuality` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::render_quality and set_render_quality` |
| `AUAudioUnit.shouldBypassEffect` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::should_bypass_effect and set_should_bypass_effect` |
| `AUAudioUnit.canProcessInPlace` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::can_process_in_place` |
| `AUAudioUnit.renderingOffline` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::rendering_offline and set_rendering_offline` |
| `AUAudioUnit.channelCapabilities` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::channel_capabilities` |
| `AUAudioUnit.contextName` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::context_name and set_context_name` |
| `AUAudioUnit.supportsMPE` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::supports_mpe` |
| `AUAudioUnit.channelMap` | property | `AUAudioUnit.h` | `AuAudioUnitInfo::channel_map and set_channel_map` |
| `AUAudioUnitBusArray` | class | `AUAudioUnit.h` | `AuAudioUnitBusArray` |
| `AUAudioUnitBusArray.setBusCount:error:` | method | `AUAudioUnit.h` | `AuAudioUnitBusArray::set_bus_count` |
| `AUAudioUnitBus` | class | `AUAudioUnit.h` | `AuAudioUnitBus` |
| `AUAudioUnitBus.setFormat:error:` | method | `AUAudioUnit.h` | `AuAudioUnitBus::set_standard_format` |
| `AUAudioUnitBus.shouldAllocateBuffer` | property | `AUAudioUnit.h` | `AuAudioUnitBusInfo::should_allocate_buffer and set_should_allocate_buffer` |
| `AUAudioUnitBus.enabled` | property | `AUAudioUnit.h` | `AuAudioUnitBusInfo::enabled and set_enabled` |
| `AUAudioUnitBus.name` | property | `AUAudioUnit.h` | `AuAudioUnitBusInfo::name and set_name` |
| `AUAudioUnitBus.contextPresentationLatency` | property | `AUAudioUnit.h` | `AuAudioUnitBusInfo::context_presentation_latency and set_context_presentation_latency` |
| `AUAudioUnitFactory` | protocol | `AUAudioUnitImplementation.h` | `AuAudioUnitFactory` |
| `AUAudioUnitFactory.createAudioUnitWithComponentDescription:error:` | protocol method | `AUAudioUnitImplementation.h` | `AuAudioUnitFactory::create_audio_unit` |
| `AUAudioUnitV2Bridge` | class | `AUAudioUnitImplementation.h` | `AuAudioUnitV2Bridge` |
| `AUAudioUnitV2Bridge.audioUnit` | property | `AUAudioUnitImplementation.h` | `AuAudioUnitV2Bridge::audio_unit_ptr` |
| `AUParameterGroup` | class | `AUParameters.h` | `AuParameterGroup` |
| `AUParameterGroup.children` | property | `AUParameters.h` | `AuParameterGroup::children` |
| `AUParameterGroup.allParameters` | property | `AUParameters.h` | `AuParameterGroup::all_parameters` |
| `AUParameterTree` | class | `AUParameters.h` | `AuParameterTree` |
| `AUParameterTree.parameterWithAddress:` | method | `AUParameters.h` | `AuParameterTree::parameter_with_address` |
| `AUParameterTree.parameterWithID:scope:element:` | method | `AUParameters.h` | `AuParameterTree::parameter_with_id` |
| `AUParameter` | class | `AUParameters.h` | `AuParameter` |
| `AUParameter.identifier` | property | `AUParameters.h` | `AuParameter::identifier` |
| `AUParameter.displayNameWithLength:` | method | `AUParameters.h` | `AuParameter::display_name_with_length` |
| `AUParameter.minValue` | property | `AUParameters.h` | `AuParameter::min_value` |
| `AUParameter.maxValue` | property | `AUParameters.h` | `AuParameter::max_value` |
| `AUParameter.value` | property | `AUParameters.h` | `AuParameter::value and set_value` |
| `AUParameter.setValue:originator:atHostTime:eventType:` | method | `AUParameters.h` | `AuParameter::set_value_with_event` |
| `AUParameter.stringFromValue:` | method | `AUParameters.h` | `AuParameter::string_from_value` |
| `AUParameter.valueFromString:` | method | `AUParameters.h` | `AuParameter::value_from_string` |
| `AVAudioUnit` | class | `AVAudioUnit.h` | `AvAudioUnit` |
| `AVAudioUnit.instantiateWithComponentDescription:options:completionHandler:` | class method | `AVAudioUnit.h` | `AvAudioUnit::instantiate` |
| `AVAudioUnit.audioComponentDescription` | property | `AVAudioUnit.h` | `AvAudioUnitInfo::audio_component_description` |
| `AVAudioUnit.audioUnit` | property | `AVAudioUnit.h` | `AvAudioUnit::audio_unit_ptr` |
| `AVAudioUnit.AUAudioUnit` | property | `AVAudioUnit.h` | `AvAudioUnit::au_audio_unit` |
| `AVAudioUnit.name` | property | `AVAudioUnit.h` | `AvAudioUnitInfo::name` |
| `AVAudioUnit.manufacturerName` | property | `AVAudioUnit.h` | `AvAudioUnitInfo::manufacturer_name` |
| `AVAudioUnit.version` | property | `AVAudioUnit.h` | `AvAudioUnitInfo::version` |
| `AVAudioUnitComponentManager` | class | `AVAudioUnitComponent.h` | `ComponentManager` |
| `AVAudioUnitComponentManager.componentsMatchingDescription:` | method | `AVAudioUnitComponent.h` | `ComponentManager::components_matching` |
| `AVAudioUnitComponent` | class | `AVAudioUnitComponent.h` | `AudioUnitComponent` |
| `AVAudioUnitComponent.name` | property | `AVAudioUnitComponent.h` | `AudioUnitComponent::name` |
| `AVAudioUnitComponent.audioComponentDescription` | property | `AVAudioUnitComponent.h` | `AudioUnitComponent::audio_component_description` |
| `AVAudioUnitComponent.versionString` | property | `AVAudioUnitComponent.h` | `AudioUnitComponent::version_string` |
| `AVAudioUnitComponent.hasCustomView` | property | `AVAudioUnitComponent.h` | `AudioUnitComponent::has_custom_view` |
| `AVAudioUnitComponent.sandboxSafe` | property | `AVAudioUnitComponent.h` | `AudioUnitComponent::is_sandbox_safe` |
| `AVAudioUnitEffect` | class | `AVAudioUnitEffect.h` | `AvAudioUnitEffect` |
| `AVAudioUnitEffect.initWithAudioComponentDescription:` | method | `AVAudioUnitEffect.h` | `AvAudioUnitEffect::new` |
| `AVAudioUnitEffect.bypass` | property | `AVAudioUnitEffect.h` | `AvAudioUnitEffect::bypass and set_bypass` |
| `AVAudioUnitGenerator` | class | `AVAudioUnitGenerator.h` | `AvAudioUnitGenerator` |
| `AVAudioUnitGenerator.initWithAudioComponentDescription:` | method | `AVAudioUnitGenerator.h` | `AvAudioUnitGenerator::new` |
| `AVAudioUnitGenerator.bypass` | property | `AVAudioUnitGenerator.h` | `AvAudioUnitGenerator::bypass and set_bypass` |
| `AVAudioUnitMIDIInstrument` | class | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument` |
| `AVAudioUnitMIDIInstrument.initWithAudioComponentDescription:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::new` |
| `AVAudioUnitMIDIInstrument.startNote:withVelocity:onChannel:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::start_note` |
| `AVAudioUnitMIDIInstrument.stopNote:onChannel:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::stop_note` |
| `AVAudioUnitMIDIInstrument.sendController:withValue:onChannel:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::send_controller` |
| `AVAudioUnitMIDIInstrument.sendPitchBend:onChannel:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::send_pitch_bend` |
| `AVAudioUnitMIDIInstrument.sendPressure:onChannel:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::send_pressure` |
| `AVAudioUnitMIDIInstrument.sendPressureForKey:withValue:onChannel:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::send_pressure_for_key` |
| `AVAudioUnitMIDIInstrument.sendProgramChange:onChannel:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::send_program_change` |
| `AVAudioUnitMIDIInstrument.sendProgramChange:bankMSB:bankLSB:onChannel:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::send_program_change_bank` |
| `AVAudioUnitMIDIInstrument.sendMIDISysExEvent:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::send_sysex` |
| `AVAudioUnitMIDIInstrument.sendMIDIEventList:` | method | `AVAudioUnitMIDIInstrument.h` | `AvAudioUnitMidiInstrument::send_event_list_raw` |
| `AudioComponentInstanceNew` | function | `AudioComponent.h` | `legacy::audio_component_instance_new` |
| `AudioComponentInstanceDispose` | function | `AudioComponent.h` | `legacy::audio_component_instance_dispose` |
| `AudioOutputUnitStart` | function | `AudioOutputUnit.h` | `legacy::audio_output_unit_start` |
| `AudioOutputUnitStop` | function | `AudioOutputUnit.h` | `legacy::audio_output_unit_stop` |
| `AudioUnitInitialize` | function | `AudioUnit.h` | `legacy::audio_unit_initialize` |
| `AudioUnitUninitialize` | function | `AudioUnit.h` | `legacy::audio_unit_uninitialize` |
| `AudioUnitRender` | function | `AudioUnit.h` | `legacy::audio_unit_render` |
| `AudioUnitAddPropertyListener` | function | `AudioUnit.h` | `legacy::audio_unit_add_property_listener` |
| `AudioUnitRemovePropertyListenerWithUserData` | function | `AudioUnit.h` | `legacy::audio_unit_remove_property_listener_with_user_data` |
| `AudioUnitScheduleParameters` | function | `AudioUnit.h` | `legacy::audio_unit_schedule_parameters` |
| `MusicDeviceMIDIEvent` | function | `MusicDevice.h` | `legacy::music_device_midi_event` |
| `MusicDeviceMIDIEventList` | function | `MusicDevice.h` | `legacy::music_device_midi_event_list_raw` |
| `MusicDeviceStartNote` | function | `MusicDevice.h` | `legacy::music_device_start_note` |
| `MusicDeviceStopNote` | function | `MusicDevice.h` | `legacy::music_device_stop_note` |
| `MusicDeviceSysEx` | function | `MusicDevice.h` | `legacy::music_device_sysex` |
| `AUAudioUnit.component` | property | `AUAudioUnit.h` | `AuAudioUnit::component_ptr` |
| `AUAudioUnit.renderBlock` | property | `AUAudioUnit.h` | `AuAudioUnit::render` |
| `AUAudioUnit.scheduleParameterBlock` | property | `AUAudioUnit.h` | `AuAudioUnit::schedule_parameter` |
| `AUAudioUnit.tokenByAddingRenderObserver:` | method | `AUAudioUnit.h` | `AuAudioUnit::add_render_observer_capture` |
| `AUAudioUnit.removeRenderObserver:` | method | `AUAudioUnit.h` | `AuAudioUnit::remove_render_observer` |
| `AUAudioUnit.scheduleMIDIEventBlock` | property | `AUAudioUnit.h` | `AuAudioUnit::schedule_midi_event` |
| `AUAudioUnit.scheduleMIDIEventListBlock` | property | `AUAudioUnit.h` | `AuAudioUnit::schedule_midi_event_list_raw` |
| `AUAudioUnit.MIDIOutputEventBlock` | property | `AUAudioUnit.h` | `AuAudioUnit::set_midi_output_event_capture_enabled and take_captured_midi_output_events` |
| `AUAudioUnit.MIDIOutputEventListBlock` | property | `AUAudioUnit.h` | `AuAudioUnit::set_midi_output_event_list_capture_enabled and take_captured_midi_output_event_lists` |
| `AUAudioUnit.musicalContextBlock` | property | `AUAudioUnit.h` | `AuAudioUnit::set_musical_context and musical_context` |
| `AUAudioUnit.transportStateBlock` | property | `AUAudioUnit.h` | `AuAudioUnit::set_transport_state and transport_state` |
| `AUAudioUnit.profileStateForCable:channel:` | method | `AUAudioUnit.h` | `AuAudioUnit::profile_state_for_cable_channel` |
| `AUAudioUnit.enableProfile:cable:onChannel:error:` | method | `AUAudioUnit.h` | `AuAudioUnit::enable_profile` |
| `AUAudioUnit.disableProfile:cable:onChannel:error:` | method | `AUAudioUnit.h` | `AuAudioUnit::disable_profile` |
| `AUAudioUnit.messageChannelFor:` | method | `AUAudioUnit.h` | `AuAudioUnit::message_channel and AuMessageChannel::call_audio_unit_json` |
| `AUAudioUnit.canPerformInput` | property | `AUAudioUnit.h` | `AuAudioUnit::can_perform_input` |
| `AUAudioUnit.canPerformOutput` | property | `AUAudioUnit.h` | `AuAudioUnit::can_perform_output` |
| `AUAudioUnit.setDeviceID:error:` | method | `AUAudioUnit.h` | `AuAudioUnit::set_device_id` |
| `AUAudioUnit.startHardwareAndReturnError:` | method | `AUAudioUnit.h` | `AuAudioUnit::start_hardware` |
| `AUAudioUnit.stopHardware` | method | `AUAudioUnit.h` | `AuAudioUnit::stop_hardware` |
| `AUParameterNode.tokenByAddingParameterObserver:` | method | `AUParameters.h` | `AuParameterTree::add_parameter_observer_capture` |
| `AUParameterNode.tokenByAddingParameterRecordingObserver:` | method | `AUParameters.h` | `AuParameterTree::add_parameter_recording_observer_capture` |
| `AUParameterNode.tokenByAddingParameterAutomationObserver:` | method | `AUParameters.h` | `AuParameterTree::add_parameter_automation_observer_capture` |
| `AUParameterNode.removeParameterObserver:` | method | `AUParameters.h` | `AuParameterTree::remove_parameter_observer` |
| `AVAudioUnit.loadAudioUnitPresetAtURL:error:` | method | `AVAudioUnit.h` | `AvAudioUnit::load_audio_unit_preset` |
| `AVAudioUnitComponentManager.componentsMatchingPredicate:` | method | `AVAudioUnitComponent.h` | `ComponentManager::components_matching_predicate` |
| `AVAudioUnitComponentManager.componentsPassingTest:` | method | `AVAudioUnitComponent.h` | `ComponentManager::components_passing_test` |
| `AVAudioUnitSampler` | class | `AVAudioUnitSampler.h` | `AvAudioUnitSampler` |
| `AVAudioUnitEQ` | class | `AVAudioUnitEQ.h` | `AvAudioUnitEQ` |
| `AVAudioUnitDelay` | class | `AVAudioUnitDelay.h` | `AvAudioUnitDelay` |
| `AVAudioUnitDistortion` | class | `AVAudioUnitDistortion.h` | `AvAudioUnitDistortion` |
| `AVAudioUnitReverb` | class | `AVAudioUnitReverb.h` | `AvAudioUnitReverb` |
| `AVAudioUnitTimeEffect` | class | `AVAudioUnitTimeEffect.h` | `AvAudioUnitTimeEffect` |
| `AVAudioUnitTimePitch` | class | `AVAudioUnitTimePitch.h` | `AvAudioUnitTimePitch` |
| `AVAudioUnitVarispeed` | class | `AVAudioUnitVarispeed.h` | `AvAudioUnitVarispeed` |
| `AVAudioUnitComponent.userTagNames` | property | `AVAudioUnitComponent.h` | `AudioUnitComponent::user_tag_names and set_user_tag_names` |
| `AVAudioUnitComponent.allTagNames` | property | `AVAudioUnitComponent.h` | `AudioUnitComponent::all_tag_names` |
| `AVAudioUnitComponent.availableArchitectures` | property | `AVAudioUnitComponent.h` | `AudioUnitComponent::available_architectures` |
| `AVAudioUnitComponent.configurationDictionary` | property | `AVAudioUnitComponent.h` | `AudioUnitComponent::configuration_dictionary` |
| `AVAudioUnitComponent.supportsNumberInputChannels:outputChannels:` | method | `AVAudioUnitComponent.h` | `AudioUnitComponent::supports_number_input_channels` |
## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `MusicDevicePrepareInstrument` | function | `MusicDevice.h` | Deprecated on macOS and explicitly skipped | `API_DEPRECATED("no longer supported", macos(10.0, 10.5)) API_UNAVAILABLE(ios, watchos, tvos)` |
| `MusicDeviceReleaseInstrument` | function | `MusicDevice.h` | Deprecated on macOS and explicitly skipped | `API_DEPRECATED("no longer supported", macos(10.0, 10.5)) API_UNAVAILABLE(ios, watchos, tvos)` |
| `AVAudioUnitComponent.componentURL` | property | `AVAudioUnitComponent.h` | Deprecated on macOS and explicitly skipped | `NS_DEPRECATED(10_10, 10_11, NA, NA)` |
| `AVAudioUnitSampler.masterGain` | property | `AVAudioUnitSampler.h` | Deprecated in favor of overallGain and explicitly skipped | `API_DEPRECATED_WITH_REPLACEMENT("overallGain", ios(8.0, 15.0), macos(10.10, 12.0), tvos(9.0, 15.0))` |
| `AUAudioUnit.intendedSpatialExperience` | property | `AUAudioUnit.h` | Unavailable on macOS and explicitly skipped | `API_AVAILABLE(visionos(26.0)) API_UNAVAILABLE(ios, watchos, tvos, macos)` |
