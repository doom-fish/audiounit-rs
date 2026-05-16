import AudioToolbox
import AVFAudio
import Foundation

func encodeAuAudioUnit(_ unit: AUAudioUnit) -> [String: Any] {
    let midiOutputNames = (unit.value(forKey: "MIDIOutputNames") as? [String]) ?? []
    let audioUnitMIDIProtocol = (unit.value(forKey: "AudioUnitMIDIProtocol") as? NSNumber)?.uint32Value ?? 0
    let hostMIDIProtocol = (unit.value(forKey: "hostMIDIProtocol") as? NSNumber)?.uint32Value ?? 0
    let migrateFromPlugin = ((unit.value(forKey: "migrateFromPlugin") as? [Any]) ?? []).map(String.init(describing:))
    let channelMap = ((unit.value(forKey: "channelMap") as? [NSNumber]) ?? []).map(\.intValue)
    let channelCapabilities = ((unit.channelCapabilities ?? []) as [NSNumber]).map(\.intValue)

    return [
        "componentDescription": encodeComponentDescription(unit.componentDescription),
        "componentName": jsonValue(unit.componentName),
        "audioUnitName": jsonValue(unit.audioUnitName),
        "manufacturerName": jsonValue(unit.manufacturerName),
        "audioUnitShortName": jsonValue(unit.audioUnitShortName),
        "componentVersion": unit.componentVersion,
        "renderResourcesAllocated": unit.renderResourcesAllocated,
        "maximumFramesToRender": Int(unit.maximumFramesToRender),
        "hasRenderBlock": true,
        "hasScheduleParameterBlock": true,
        "allParameterValues": (unit.value(forKey: "allParameterValues") as? Bool) ?? false,
        "musicDeviceOrEffect": unit.isMusicDeviceOrEffect,
        "virtualMIDICableCount": unit.virtualMIDICableCount,
        "hasScheduleMIDIEventBlock": unit.scheduleMIDIEventBlock != nil,
        "hasScheduleMIDIEventListBlock": unit.scheduleMIDIEventListBlock != nil,
        "midiOutputNames": midiOutputNames,
        "providesUserInterface": unit.providesUserInterface,
        "audioUnitMIDIProtocol": audioUnitMIDIProtocol,
        "hostMIDIProtocol": hostMIDIProtocol,
        "fullStatePlist": jsonValue(plistString(unit.fullState)),
        "fullStateForDocumentPlist": jsonValue(plistString(unit.fullStateForDocument)),
        "factoryPresets": (unit.factoryPresets ?? []).map(encodePreset),
        "userPresets": unit.userPresets.map(encodePreset),
        "supportsUserPresets": unit.supportsUserPresets,
        "isLoadedInProcess": unit.isLoadedInProcess,
        "currentPreset": jsonValue(unit.currentPreset.map(encodePreset)),
        "latency": unit.latency,
        "tailTime": unit.tailTime,
        "renderQuality": unit.renderQuality,
        "shouldBypassEffect": unit.shouldBypassEffect,
        "canProcessInPlace": unit.canProcessInPlace,
        "renderingOffline": unit.isRenderingOffline,
        "channelCapabilities": channelCapabilities,
        "contextName": jsonValue(unit.contextName),
        "migrateFromPlugin": migrateFromPlugin,
        "supportsMPE": unit.supportsMPE,
        "channelMap": channelMap,
        "inputBusCount": unit.inputBusses.count,
        "outputBusCount": unit.outputBusses.count,
        "parameterTreeAvailable": unit.parameterTree != nil,
        "isV2Bridge": unit is AUAudioUnitV2Bridge,
    ]
}

@_cdecl("au_auaudiounit_instantiate_sync")
public func au_auaudiounit_instantiate_sync(
    _ type: UInt32,
    _ subtype: UInt32,
    _ manufacturer: UInt32,
    _ flags: UInt32,
    _ flagsMask: UInt32,
    _ options: UInt32,
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil

    let result = instantiateAUAudioUnitSync(
        description: makeDesc(type, subtype, manufacturer, flags, flagsMask),
        options: AudioComponentInstantiationOptions(rawValue: options)
    )

    switch result {
    case let .success(unit):
        outUnit.pointee = retainBox(unit)
        return AU_OK
    case let .failure(error):
        setError(outErrorMsg, error.localizedDescription)
        if (error as NSError).code == Int(AU_TIMED_OUT) {
            return AU_TIMED_OUT
        }
        return AU_INSTANTIATE_FAILED
    }
}

@_cdecl("au_auaudiounit_release")
public func au_auaudiounit_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AUAudioUnit.self)
}

@_cdecl("au_auaudiounit_snapshot_json")
public func au_auaudiounit_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAuAudioUnit(borrowBox(ptr, as: AUAudioUnit.self)))
}

@_cdecl("au_auaudiounit_allocate_render_resources")
public func au_auaudiounit_allocate_render_resources(
    _ ptr: UnsafeMutableRawPointer,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    do {
        try borrowBox(ptr, as: AUAudioUnit.self).allocateRenderResources()
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_PROPERTY_ERROR
    }
}

@_cdecl("au_auaudiounit_deallocate_render_resources")
public func au_auaudiounit_deallocate_render_resources(_ ptr: UnsafeMutableRawPointer) {
    borrowBox(ptr, as: AUAudioUnit.self).deallocateRenderResources()
}

@_cdecl("au_auaudiounit_reset")
public func au_auaudiounit_reset(_ ptr: UnsafeMutableRawPointer) {
    borrowBox(ptr, as: AUAudioUnit.self).reset()
}

@_cdecl("au_auaudiounit_input_busses")
public func au_auaudiounit_input_busses(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AUAudioUnit.self).inputBusses)
}

@_cdecl("au_auaudiounit_output_busses")
public func au_auaudiounit_output_busses(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AUAudioUnit.self).outputBusses)
}

@_cdecl("au_auaudiounit_parameter_tree")
public func au_auaudiounit_parameter_tree(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    guard let tree = borrowBox(ptr, as: AUAudioUnit.self).parameterTree else { return nil }
    return retainBox(tree)
}

@_cdecl("au_auaudiounit_parameters_for_overview_json")
public func au_auaudiounit_parameters_for_overview_json(
    _ ptr: UnsafeMutableRawPointer,
    _ count: Int
) -> UnsafeMutablePointer<CChar>? {
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    let overview = unit.parametersForOverview(withCount: count).map(\.uint64Value)
    return jsonCString(overview)
}

@_cdecl("au_auaudiounit_set_maximum_frames_to_render")
public func au_auaudiounit_set_maximum_frames_to_render(_ ptr: UnsafeMutableRawPointer, _ value: UInt32) {
    borrowBox(ptr, as: AUAudioUnit.self).maximumFramesToRender = AUAudioFrameCount(value)
}

@_cdecl("au_auaudiounit_set_render_quality")
public func au_auaudiounit_set_render_quality(_ ptr: UnsafeMutableRawPointer, _ value: Int) {
    borrowBox(ptr, as: AUAudioUnit.self).renderQuality = value
}

@_cdecl("au_auaudiounit_set_should_bypass_effect")
public func au_auaudiounit_set_should_bypass_effect(_ ptr: UnsafeMutableRawPointer, _ value: Bool) {
    borrowBox(ptr, as: AUAudioUnit.self).shouldBypassEffect = value
}

@_cdecl("au_auaudiounit_set_rendering_offline")
public func au_auaudiounit_set_rendering_offline(_ ptr: UnsafeMutableRawPointer, _ value: Bool) {
    borrowBox(ptr, as: AUAudioUnit.self).isRenderingOffline = value
}

@_cdecl("au_auaudiounit_set_context_name")
public func au_auaudiounit_set_context_name(_ ptr: UnsafeMutableRawPointer, _ value: UnsafePointer<CChar>?) {
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    unit.contextName = value.map(String.init(cString:))
}

@_cdecl("au_auaudiounit_set_current_preset")
public func au_auaudiounit_set_current_preset(
    _ ptr: UnsafeMutableRawPointer,
    _ number: Int,
    _ name: UnsafePointer<CChar>?
) {
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    guard let name else {
        unit.currentPreset = nil
        return
    }
    let preset = AUAudioUnitPreset()
    preset.number = number
    preset.name = String(cString: name)
    unit.currentPreset = preset
}

@_cdecl("au_auaudiounit_set_channel_map")
public func au_auaudiounit_set_channel_map(
    _ ptr: UnsafeMutableRawPointer,
    _ values: UnsafePointer<Int32>?,
    _ count: Int
) {
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    guard let values else {
        unit.channelMap = nil
        return
    }
    let map = (0 ..< count).map { NSNumber(value: values.advanced(by: $0).pointee) }
    unit.channelMap = map
}
