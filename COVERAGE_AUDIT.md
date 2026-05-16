# audiounit-rs coverage audit (vs MacOSX26.2.sdk)

Scope note: `AudioUnit.framework` in MacOSX26.2.sdk is a thin umbrella that reexports `AudioToolbox.framework` headers. This audit treats those reexported `AudioToolbox` declarations as the `AudioUnit.framework` public surface.

Sampling note: the combined `AudioUnit.framework` + `AVFAudio.framework` AUAudioUnit-family surface is large, so this audit samples **173 high-signal public symbols** across modern `AUAudioUnit` / parameter APIs, legacy `AudioUnit` / `MusicDevice` control APIs, VoiceProcessing I/O properties, and the `AVAudioUnit` family.

SDK_PUBLIC_SYMBOLS: 173
VERIFIED: 113
GAPS: 56
EXEMPT: 4
COVERAGE_PCT: 66.86%

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

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `AudioComponentInstanceNew` | function | `AudioComponent.h` | No direct raw component instantiation lifecycle on the public Rust surface |
| `AudioComponentInstanceDispose` | function | `AudioComponent.h` | No direct raw component instantiation lifecycle on the public Rust surface |
| `AudioOutputUnitStart` | function | `AudioOutputUnit.h` | No explicit start helper for legacy output units |
| `AudioOutputUnitStop` | function | `AudioOutputUnit.h` | No explicit stop helper for legacy output units |
| `AudioUnitInitialize` | function | `AudioUnit.h` | Legacy initialize and uninitialize are not wrapped |
| `AudioUnitUninitialize` | function | `AudioUnit.h` | Legacy initialize and uninitialize are not wrapped |
| `AudioUnitRender` | function | `AudioUnit.h` | No render invocation wrapper beyond property-based callback setup |
| `AudioUnitAddPropertyListener` | function | `AudioUnit.h` | Property listener registration is not exposed |
| `AudioUnitRemovePropertyListenerWithUserData` | function | `AudioUnit.h` | Property listener removal is not exposed |
| `AudioUnitScheduleParameters` | function | `AudioUnit.h` | No raw parameter scheduling wrapper |
| `MusicDeviceMIDIEvent` | function | `MusicDevice.h` | No raw MusicDevice C API exposure |
| `MusicDeviceMIDIEventList` | function | `MusicDevice.h` | No raw MusicDevice MIDI 2.0 event-list wrapper |
| `MusicDeviceStartNote` | function | `MusicDevice.h` | No raw MusicDevice note lifecycle wrapper |
| `MusicDeviceStopNote` | function | `MusicDevice.h` | No raw MusicDevice note lifecycle wrapper |
| `MusicDeviceSysEx` | function | `MusicDevice.h` | No raw MusicDevice SysEx wrapper |
| `AUAudioUnit.component` | property | `AUAudioUnit.h` | Raw AudioComponent handle is not exposed |
| `AUAudioUnit.renderBlock` | property | `AUAudioUnit.h` | Info snapshot reports presence only; render block is not exposed |
| `AUAudioUnit.scheduleParameterBlock` | property | `AUAudioUnit.h` | Info snapshot reports presence only; scheduleParameterBlock is not exposed |
| `AUAudioUnit.tokenByAddingRenderObserver:` | method | `AUAudioUnit.h` | Render observer token lifecycle is not exposed |
| `AUAudioUnit.removeRenderObserver:` | method | `AUAudioUnit.h` | Render observer token lifecycle is not exposed |
| `AUAudioUnit.scheduleMIDIEventBlock` | property | `AUAudioUnit.h` | Info snapshot reports presence only; MIDI scheduling block is not exposed |
| `AUAudioUnit.scheduleMIDIEventListBlock` | property | `AUAudioUnit.h` | Info snapshot reports presence only; MIDI event-list block is not exposed |
| `AUAudioUnit.MIDIOutputEventBlock` | property | `AUAudioUnit.h` | Host MIDI output block is not exposed |
| `AUAudioUnit.MIDIOutputEventListBlock` | property | `AUAudioUnit.h` | Host MIDI output event-list block is not exposed |
| `AUAudioUnit.musicalContextBlock` | property | `AUAudioUnit.h` | Host musical context callbacks are not exposed |
| `AUAudioUnit.transportStateBlock` | property | `AUAudioUnit.h` | Transport state callbacks are not exposed |
| `AUAudioUnit.profileStateForCable:channel:` | method | `AUAudioUnit.h` | MIDICI profile queries are not exposed |
| `AUAudioUnit.enableProfile:cable:onChannel:error:` | method | `AUAudioUnit.h` | MIDICI profile enable API is not exposed |
| `AUAudioUnit.disableProfile:cable:onChannel:error:` | method | `AUAudioUnit.h` | MIDICI profile disable API is not exposed |
| `AUAudioUnit.messageChannelFor:` | method | `AUAudioUnit.h` | AUMessageChannel bridging is not exposed |
| `AUAudioUnit.canPerformInput` | property | `AUAudioUnit.h` | Input and output category APIs are not wrapped |
| `AUAudioUnit.canPerformOutput` | property | `AUAudioUnit.h` | Input and output category APIs are not wrapped |
| `AUAudioUnit.setDeviceID:error:` | method | `AUAudioUnit.h` | Input and output unit device selection is not wrapped |
| `AUAudioUnit.startHardwareAndReturnError:` | method | `AUAudioUnit.h` | Input and output unit hardware start is not wrapped |
| `AUAudioUnit.stopHardware` | method | `AUAudioUnit.h` | Input and output unit hardware stop is not wrapped |
| `AUAudioUnit.intendedSpatialExperience` | property | `AUAudioUnit.h` | Spatial-audio intent API is not exposed |
| `AUParameterNode.tokenByAddingParameterObserver:` | method | `AUParameters.h` | Observer token lifecycle is not exposed |
| `AUParameterNode.tokenByAddingParameterRecordingObserver:` | method | `AUParameters.h` | Recording observer API is not exposed |
| `AUParameterNode.tokenByAddingParameterAutomationObserver:` | method | `AUParameters.h` | Automation observer API is not exposed |
| `AUParameterNode.removeParameterObserver:` | method | `AUParameters.h` | Observer removal API is not exposed |
| `AVAudioUnit.loadAudioUnitPresetAtURL:error:` | method | `AVAudioUnit.h` | Public Rust method exists but the Swift bridge returns an AU_UNAVAILABLE stub |
| `AVAudioUnitComponentManager.componentsMatchingPredicate:` | method | `AVAudioUnitComponent.h` | Predicate-based enumeration is not wrapped |
| `AVAudioUnitComponentManager.componentsPassingTest:` | method | `AVAudioUnitComponent.h` | Block-based enumeration is not wrapped |
| `AVAudioUnitSampler` | class | `AVAudioUnitSampler.h` | No sampler subclass wrapper |
| `AVAudioUnitEQ` | class | `AVAudioUnitEQ.h` | No EQ subclass wrapper |
| `AVAudioUnitDelay` | class | `AVAudioUnitDelay.h` | No delay subclass wrapper |
| `AVAudioUnitDistortion` | class | `AVAudioUnitDistortion.h` | No distortion subclass wrapper |
| `AVAudioUnitReverb` | class | `AVAudioUnitReverb.h` | No reverb subclass wrapper |
| `AVAudioUnitTimeEffect` | class | `AVAudioUnitTimeEffect.h` | No time-effect subclass wrapper |
| `AVAudioUnitTimePitch` | class | `AVAudioUnitTimePitch.h` | No time-pitch subclass wrapper |
| `AVAudioUnitVarispeed` | class | `AVAudioUnitVarispeed.h` | No varispeed subclass wrapper |
| `AVAudioUnitComponent.userTagNames` | property | `AVAudioUnitComponent.h` | Public Rust tags() placeholder stays empty; user tags are not bridged |
| `AVAudioUnitComponent.allTagNames` | property | `AVAudioUnitComponent.h` | Public Rust tags() placeholder stays empty; all tags are not bridged |
| `AVAudioUnitComponent.availableArchitectures` | property | `AVAudioUnitComponent.h` | Architecture metadata is not exposed |
| `AVAudioUnitComponent.configurationDictionary` | property | `AVAudioUnitComponent.h` | Configuration dictionary is not exposed |
| `AVAudioUnitComponent.supportsNumberInputChannels:outputChannels:` | method | `AVAudioUnitComponent.h` | Channel compatibility probe is not exposed |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `MusicDevicePrepareInstrument` | function | `MusicDevice.h` | Deprecated on macOS and explicitly skipped | `API_DEPRECATED("no longer supported", macos(10.0, 10.5)) API_UNAVAILABLE(ios, watchos, tvos)` |
| `MusicDeviceReleaseInstrument` | function | `MusicDevice.h` | Deprecated on macOS and explicitly skipped | `API_DEPRECATED("no longer supported", macos(10.0, 10.5)) API_UNAVAILABLE(ios, watchos, tvos)` |
| `AVAudioUnitComponent.componentURL` | property | `AVAudioUnitComponent.h` | Deprecated on macOS and explicitly skipped | `NS_DEPRECATED(10_10, 10_11, NA, NA)` |
| `AVAudioUnitSampler.masterGain` | property | `AVAudioUnitSampler.h` | Deprecated in favor of overallGain and explicitly skipped | `API_DEPRECATED_WITH_REPLACEMENT("overallGain", ios(8.0, 15.0), macos(10.10, 12.0), tvos(9.0, 15.0))` |
