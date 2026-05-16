import AudioToolbox
import AVFAudio
import CoreMIDI
import Foundation

private struct MusicalContextState {
    let currentTempo: Double?
    let timeSignatureNumerator: Double?
    let timeSignatureDenominator: Int?
    let currentBeatPosition: Double?
    let sampleOffsetToNextBeat: Int?
    let currentMeasureDownbeatPosition: Double?

    var jsonObject: [String: Any] {
        [
            "currentTempo": jsonValue(currentTempo),
            "timeSignatureNumerator": jsonValue(timeSignatureNumerator),
            "timeSignatureDenominator": jsonValue(timeSignatureDenominator),
            "currentBeatPosition": jsonValue(currentBeatPosition),
            "sampleOffsetToNextBeat": jsonValue(sampleOffsetToNextBeat),
            "currentMeasureDownbeatPosition": jsonValue(currentMeasureDownbeatPosition),
        ]
    }
}

private struct TransportStateState {
    let transportStateFlags: UInt64
    let currentSamplePosition: Double?
    let cycleStartBeatPosition: Double?
    let cycleEndBeatPosition: Double?

    var jsonObject: [String: Any] {
        [
            "transportStateFlags": transportStateFlags,
            "currentSamplePosition": jsonValue(currentSamplePosition),
            "cycleStartBeatPosition": jsonValue(cycleStartBeatPosition),
            "cycleEndBeatPosition": jsonValue(cycleEndBeatPosition),
        ]
    }
}

private final class MessageChannelHandle {
    let channel: any AUMessageChannel

    init(_ channel: any AUMessageChannel) {
        self.channel = channel
    }
}

private let auAdvancedLock = NSLock()
private var renderObserverEvents: [ObjectIdentifier: [Int: [[String: Any]]]] = [:]
private var midiOutputEvents: [ObjectIdentifier: [[String: Any]]] = [:]
private var midiOutputEventLists: [ObjectIdentifier: [[String: Any]]] = [:]
private var musicalContextStates: [ObjectIdentifier: MusicalContextState] = [:]
private var transportStateStates: [ObjectIdentifier: TransportStateState] = [:]

private func audioUnitKey(_ unit: AUAudioUnit) -> ObjectIdentifier {
    ObjectIdentifier(unit)
}

private func appendRenderObserverEvent(
    unitKey: ObjectIdentifier,
    token: Int,
    event: [String: Any]
) {
    var unitEvents = renderObserverEvents[unitKey] ?? [:]
    var events = unitEvents[token] ?? []
    events.append(event)
    unitEvents[token] = events
    renderObserverEvents[unitKey] = unitEvents
}

private func drainRenderObserverEvents(unitKey: ObjectIdentifier, token: Int) -> [[String: Any]] {
    var unitEvents = renderObserverEvents[unitKey] ?? [:]
    let events = unitEvents[token] ?? []
    unitEvents[token] = []
    renderObserverEvents[unitKey] = unitEvents
    return events
}

private func clearRenderObserverEvents(unitKey: ObjectIdentifier, token: Int) {
    renderObserverEvents[unitKey]?[token] = nil
}

private func drainMidiOutputEvents(unitKey: ObjectIdentifier) -> [[String: Any]] {
    let events = midiOutputEvents[unitKey] ?? []
    midiOutputEvents[unitKey] = []
    return events
}

private func drainMidiOutputEventLists(unitKey: ObjectIdentifier) -> [[String: Any]] {
    let events = midiOutputEventLists[unitKey] ?? []
    midiOutputEventLists[unitKey] = []
    return events
}

private func makeMusicalContextState(from object: Any?) -> MusicalContextState? {
    guard let dictionary = object as? [String: Any] else { return nil }
    return MusicalContextState(
        currentTempo: (dictionary["currentTempo"] as? NSNumber)?.doubleValue,
        timeSignatureNumerator: (dictionary["timeSignatureNumerator"] as? NSNumber)?.doubleValue,
        timeSignatureDenominator: (dictionary["timeSignatureDenominator"] as? NSNumber)?.intValue,
        currentBeatPosition: (dictionary["currentBeatPosition"] as? NSNumber)?.doubleValue,
        sampleOffsetToNextBeat: (dictionary["sampleOffsetToNextBeat"] as? NSNumber)?.intValue,
        currentMeasureDownbeatPosition: (dictionary["currentMeasureDownbeatPosition"] as? NSNumber)?.doubleValue
    )
}

private func makeTransportStateState(from object: Any?) -> TransportStateState? {
    guard let dictionary = object as? [String: Any] else { return nil }
    return TransportStateState(
        transportStateFlags: (dictionary["transportStateFlags"] as? NSNumber)?.uint64Value ?? 0,
        currentSamplePosition: (dictionary["currentSamplePosition"] as? NSNumber)?.doubleValue,
        cycleStartBeatPosition: (dictionary["cycleStartBeatPosition"] as? NSNumber)?.doubleValue,
        cycleEndBeatPosition: (dictionary["cycleEndBeatPosition"] as? NSNumber)?.doubleValue
    )
}

private func encodeMIDICIProfile(_ profile: MIDICIProfile) -> [String: Any] {
    [
        "name": profile.name,
        "profileId": [UInt8](profile.profileID),
    ]
}

private func encodeMIDICIProfileState(_ state: MIDICIProfileState) -> [String: Any] {
    [
        "enabledProfiles": state.enabledProfiles.map(encodeMIDICIProfile),
        "disabledProfiles": state.disabledProfiles.map(encodeMIDICIProfile),
    ]
}

private func makeMIDICIProfile(
    profileID: UnsafePointer<UInt8>,
    length: Int,
    name: UnsafePointer<CChar>?
) -> MIDICIProfile? {
    guard length == 5 else { return nil }
    let data = Data(bytes: profileID, count: length)
    return MIDICIProfile(data: data, name: name.map(String.init(cString:)) ?? "Rust MIDI-CI Profile")
}

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

@_cdecl("au_auaudiounit_component")
public func au_auaudiounit_component(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    UnsafeMutableRawPointer(borrowBox(ptr, as: AUAudioUnit.self).component)
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

@_cdecl("au_auaudiounit_render")
public func au_auaudiounit_render(
    _ ptr: UnsafeMutableRawPointer,
    _ actionFlags: UnsafeMutablePointer<AudioUnitRenderActionFlags>?,
    _ timeStamp: UnsafePointer<AudioTimeStamp>?,
    _ frameCount: UInt32,
    _ outputBusNumber: Int,
    _ outputData: UnsafeMutablePointer<AudioBufferList>?
) -> Int32 {
    guard let timeStamp, let outputData else {
        return AU_INVALID_ARGUMENT
    }
    var flags = actionFlags?.pointee ?? []
    let status = borrowBox(ptr, as: AUAudioUnit.self).renderBlock(
        &flags,
        timeStamp,
        frameCount,
        outputBusNumber,
        outputData,
        nil
    )
    actionFlags?.pointee = flags
    return status
}

@_cdecl("au_auaudiounit_schedule_parameter")
public func au_auaudiounit_schedule_parameter(
    _ ptr: UnsafeMutableRawPointer,
    _ eventSampleTime: Int64,
    _ rampDurationSampleFrames: UInt32,
    _ parameterAddress: UInt64,
    _ value: Float
) -> Int32 {
    borrowBox(ptr, as: AUAudioUnit.self).scheduleParameterBlock(
        AUEventSampleTime(eventSampleTime),
        rampDurationSampleFrames,
        AUParameterAddress(parameterAddress),
        value
    )
    return AU_OK
}

@_cdecl("au_auaudiounit_add_render_observer_capture")
public func au_auaudiounit_add_render_observer_capture(
    _ ptr: UnsafeMutableRawPointer,
    _ outToken: UnsafeMutablePointer<Int>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    let unitKey = audioUnitKey(unit)
    var token = 0
    token = unit.token(byAddingRenderObserver: { actionFlags, timeStamp, frameCount, outputBusNumber in
        let event: [String: Any] = [
            "actionFlags": actionFlags.rawValue,
            "sampleTime": timeStamp.pointee.mSampleTime,
            "hostTime": timeStamp.pointee.mHostTime,
            "frameCount": Int(frameCount),
            "outputBusNumber": outputBusNumber,
        ]
        auAdvancedLock.lock()
        appendRenderObserverEvent(unitKey: unitKey, token: token, event: event)
        auAdvancedLock.unlock()
    })
    outToken.pointee = token
    auAdvancedLock.lock()
    _ = renderObserverEvents[unitKey, default: [:]][token]
    auAdvancedLock.unlock()
    return AU_OK
}

@_cdecl("au_auaudiounit_take_render_observer_events_json")
public func au_auaudiounit_take_render_observer_events_json(
    _ ptr: UnsafeMutableRawPointer,
    _ token: Int
) -> UnsafeMutablePointer<CChar>? {
    let unitKey = audioUnitKey(borrowBox(ptr, as: AUAudioUnit.self))
    auAdvancedLock.lock()
    let events = drainRenderObserverEvents(unitKey: unitKey, token: token)
    auAdvancedLock.unlock()
    return jsonCString(events)
}

@_cdecl("au_auaudiounit_remove_render_observer_capture")
public func au_auaudiounit_remove_render_observer_capture(
    _ ptr: UnsafeMutableRawPointer,
    _ token: Int
) {
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    let unitKey = audioUnitKey(unit)
    unit.removeRenderObserver(token)
    auAdvancedLock.lock()
    clearRenderObserverEvents(unitKey: unitKey, token: token)
    auAdvancedLock.unlock()
}

@_cdecl("au_auaudiounit_schedule_midi_event")
public func au_auaudiounit_schedule_midi_event(
    _ ptr: UnsafeMutableRawPointer,
    _ eventSampleTime: Int64,
    _ cable: UInt8,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int
) -> Int32 {
    guard let block = borrowBox(ptr, as: AUAudioUnit.self).scheduleMIDIEventBlock else {
        return AU_UNAVAILABLE
    }
    guard let bytes, length > 0 else {
        return AU_INVALID_ARGUMENT
    }
    block(AUEventSampleTime(eventSampleTime), cable, length, bytes)
    return AU_OK
}

@_cdecl("au_auaudiounit_schedule_midi_event_list")
public func au_auaudiounit_schedule_midi_event_list(
    _ ptr: UnsafeMutableRawPointer,
    _ eventSampleTime: Int64,
    _ cable: UInt8,
    _ eventList: UnsafeRawPointer?
) -> Int32 {
    guard let block = borrowBox(ptr, as: AUAudioUnit.self).scheduleMIDIEventListBlock else {
        return AU_UNAVAILABLE
    }
    guard let eventList else {
        return AU_INVALID_ARGUMENT
    }
    return block(
        AUEventSampleTime(eventSampleTime),
        cable,
        eventList.assumingMemoryBound(to: MIDIEventList.self)
    )
}

@_cdecl("au_auaudiounit_set_midi_output_event_capture_enabled")
public func au_auaudiounit_set_midi_output_event_capture_enabled(
    _ ptr: UnsafeMutableRawPointer,
    _ enabled: Bool
) {
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    let unitKey = audioUnitKey(unit)
    auAdvancedLock.lock()
    if !enabled {
        midiOutputEvents[unitKey] = []
    }
    auAdvancedLock.unlock()
    unit.midiOutputEventBlock = enabled ? { eventSampleTime, cable, length, midiBytes in
        let bytes = Array(UnsafeBufferPointer(start: midiBytes, count: length))
        auAdvancedLock.lock()
        var events = midiOutputEvents[unitKey] ?? []
        events.append([
            "eventSampleTime": eventSampleTime,
            "cable": Int(cable),
            "bytes": bytes,
        ])
        midiOutputEvents[unitKey] = events
        auAdvancedLock.unlock()
        return noErr
    } : nil
}

@_cdecl("au_auaudiounit_take_midi_output_events_json")
public func au_auaudiounit_take_midi_output_events_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let unitKey = audioUnitKey(borrowBox(ptr, as: AUAudioUnit.self))
    auAdvancedLock.lock()
    let events = drainMidiOutputEvents(unitKey: unitKey)
    auAdvancedLock.unlock()
    return jsonCString(events)
}

@_cdecl("au_auaudiounit_set_midi_output_event_list_capture_enabled")
public func au_auaudiounit_set_midi_output_event_list_capture_enabled(
    _ ptr: UnsafeMutableRawPointer,
    _ enabled: Bool
) {
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    let unitKey = audioUnitKey(unit)
    auAdvancedLock.lock()
    if !enabled {
        midiOutputEventLists[unitKey] = []
    }
    auAdvancedLock.unlock()
    unit.midiOutputEventListBlock = enabled ? { eventSampleTime, cable, eventList in
        auAdvancedLock.lock()
        var events = midiOutputEventLists[unitKey] ?? []
        events.append([
            "eventSampleTime": eventSampleTime,
            "cable": Int(cable),
            "protocol": eventList.pointee.protocol.rawValue,
            "numPackets": Int(eventList.pointee.numPackets),
        ])
        midiOutputEventLists[unitKey] = events
        auAdvancedLock.unlock()
        return noErr
    } : nil
}

@_cdecl("au_auaudiounit_take_midi_output_event_lists_json")
public func au_auaudiounit_take_midi_output_event_lists_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let unitKey = audioUnitKey(borrowBox(ptr, as: AUAudioUnit.self))
    auAdvancedLock.lock()
    let events = drainMidiOutputEventLists(unitKey: unitKey)
    auAdvancedLock.unlock()
    return jsonCString(events)
}

@_cdecl("au_auaudiounit_set_musical_context_json")
public func au_auaudiounit_set_musical_context_json(
    _ ptr: UnsafeMutableRawPointer,
    _ value: UnsafePointer<CChar>?,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    let unitKey = audioUnitKey(unit)
    guard let value else {
        auAdvancedLock.lock()
        musicalContextStates[unitKey] = nil
        auAdvancedLock.unlock()
        unit.musicalContextBlock = nil
        return AU_OK
    }
    guard let state = makeMusicalContextState(from: jsonObject(from: value)) else {
        setError(outErrorMsg, "musical context payload must be a JSON object")
        return AU_INVALID_ARGUMENT
    }
    auAdvancedLock.lock()
    musicalContextStates[unitKey] = state
    auAdvancedLock.unlock()
    unit.musicalContextBlock = { currentTempo, timeSignatureNumerator, timeSignatureDenominator, currentBeatPosition, sampleOffsetToNextBeat, currentMeasureDownbeatPosition in
        auAdvancedLock.lock()
        let current = musicalContextStates[unitKey]
        auAdvancedLock.unlock()
        guard let current else { return false }
        if let currentTempo, let value = current.currentTempo {
            currentTempo.pointee = value
        }
        if let timeSignatureNumerator, let value = current.timeSignatureNumerator {
            timeSignatureNumerator.pointee = value
        }
        if let timeSignatureDenominator, let value = current.timeSignatureDenominator {
            timeSignatureDenominator.pointee = value
        }
        if let currentBeatPosition, let value = current.currentBeatPosition {
            currentBeatPosition.pointee = value
        }
        if let sampleOffsetToNextBeat, let value = current.sampleOffsetToNextBeat {
            sampleOffsetToNextBeat.pointee = value
        }
        if let currentMeasureDownbeatPosition, let value = current.currentMeasureDownbeatPosition {
            currentMeasureDownbeatPosition.pointee = value
        }
        return true
    }
    return AU_OK
}

@_cdecl("au_auaudiounit_musical_context_json")
public func au_auaudiounit_musical_context_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let unitKey = audioUnitKey(borrowBox(ptr, as: AUAudioUnit.self))
    auAdvancedLock.lock()
    let state = musicalContextStates[unitKey]
    auAdvancedLock.unlock()
    return state.map { jsonCString($0.jsonObject) } ?? ffiString("null")
}

@_cdecl("au_auaudiounit_set_transport_state_json")
public func au_auaudiounit_set_transport_state_json(
    _ ptr: UnsafeMutableRawPointer,
    _ value: UnsafePointer<CChar>?,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    let unitKey = audioUnitKey(unit)
    guard let value else {
        auAdvancedLock.lock()
        transportStateStates[unitKey] = nil
        auAdvancedLock.unlock()
        unit.transportStateBlock = nil
        return AU_OK
    }
    guard let state = makeTransportStateState(from: jsonObject(from: value)) else {
        setError(outErrorMsg, "transport state payload must be a JSON object")
        return AU_INVALID_ARGUMENT
    }
    auAdvancedLock.lock()
    transportStateStates[unitKey] = state
    auAdvancedLock.unlock()
    unit.transportStateBlock = { transportStateFlags, currentSamplePosition, cycleStartBeatPosition, cycleEndBeatPosition in
        auAdvancedLock.lock()
        let current = transportStateStates[unitKey]
        auAdvancedLock.unlock()
        guard let current else { return false }
        if let transportStateFlags {
            transportStateFlags.pointee = AUHostTransportStateFlags(rawValue: UInt(current.transportStateFlags))
        }
        if let currentSamplePosition, let value = current.currentSamplePosition {
            currentSamplePosition.pointee = value
        }
        if let cycleStartBeatPosition, let value = current.cycleStartBeatPosition {
            cycleStartBeatPosition.pointee = value
        }
        if let cycleEndBeatPosition, let value = current.cycleEndBeatPosition {
            cycleEndBeatPosition.pointee = value
        }
        return true
    }
    return AU_OK
}

@_cdecl("au_auaudiounit_transport_state_json")
public func au_auaudiounit_transport_state_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let unitKey = audioUnitKey(borrowBox(ptr, as: AUAudioUnit.self))
    auAdvancedLock.lock()
    let state = transportStateStates[unitKey]
    auAdvancedLock.unlock()
    return state.map { jsonCString($0.jsonObject) } ?? ffiString("null")
}

@_cdecl("au_auaudiounit_profile_state_for_cable_channel_json")
public func au_auaudiounit_profile_state_for_cable_channel_json(
    _ ptr: UnsafeMutableRawPointer,
    _ cable: UInt8,
    _ channel: UInt8,
    _ outJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outJson.pointee = nil
    outErrorMsg.pointee = nil
    let state = borrowBox(ptr, as: AUAudioUnit.self).profileState(forCable: cable, channel: channel)
    outJson.pointee = jsonCString(encodeMIDICIProfileState(state))
    return AU_OK
}

@_cdecl("au_auaudiounit_enable_profile")
public func au_auaudiounit_enable_profile(
    _ ptr: UnsafeMutableRawPointer,
    _ profileID: UnsafePointer<UInt8>?,
    _ length: Int,
    _ name: UnsafePointer<CChar>?,
    _ cable: UInt8,
    _ channel: UInt8,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    guard let profileID, let profile = makeMIDICIProfile(profileID: profileID, length: length, name: name) else {
        setError(outErrorMsg, "MIDI-CI profile identifiers must be exactly 5 bytes")
        return AU_INVALID_ARGUMENT
    }
    do {
        try borrowBox(ptr, as: AUAudioUnit.self).enable(profile, cable: cable, onChannel: channel)
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_PROPERTY_ERROR
    }
}

@_cdecl("au_auaudiounit_disable_profile")
public func au_auaudiounit_disable_profile(
    _ ptr: UnsafeMutableRawPointer,
    _ profileID: UnsafePointer<UInt8>?,
    _ length: Int,
    _ name: UnsafePointer<CChar>?,
    _ cable: UInt8,
    _ channel: UInt8,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    guard let profileID, let profile = makeMIDICIProfile(profileID: profileID, length: length, name: name) else {
        setError(outErrorMsg, "MIDI-CI profile identifiers must be exactly 5 bytes")
        return AU_INVALID_ARGUMENT
    }
    do {
        try borrowBox(ptr, as: AUAudioUnit.self).disableProfile(profile, cable: cable, onChannel: channel)
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_PROPERTY_ERROR
    }
}

@_cdecl("au_auaudiounit_message_channel")
public func au_auaudiounit_message_channel(
    _ ptr: UnsafeMutableRawPointer,
    _ name: UnsafePointer<CChar>?,
    _ outChannel: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outChannel.pointee = nil
    outErrorMsg.pointee = nil
    guard let name else {
        setError(outErrorMsg, "message channel name was null")
        return AU_INVALID_ARGUMENT
    }
    let channel = borrowBox(ptr, as: AUAudioUnit.self).messageChannel(for: String(cString: name))
    outChannel.pointee = retainBox(MessageChannelHandle(channel))
    return AU_OK
}

@_cdecl("au_message_channel_release")
public func au_message_channel_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: MessageChannelHandle.self)
}

@_cdecl("au_message_channel_call_audio_unit_json")
public func au_message_channel_call_audio_unit_json(
    _ ptr: UnsafeMutableRawPointer,
    _ messageJson: UnsafePointer<CChar>?,
    _ outJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outJson.pointee = nil
    outErrorMsg.pointee = nil
    guard let message = jsonObject(from: messageJson) as? [String: Any] else {
        setError(outErrorMsg, "message channel payload must be a JSON object")
        return AU_INVALID_ARGUMENT
    }
    let channel = borrowBox(ptr, as: MessageChannelHandle.self).channel
    guard let response = channel.callAudioUnit?(message) else {
        setError(outErrorMsg, "message channel does not implement callAudioUnit:")
        return AU_UNAVAILABLE
    }
    outJson.pointee = jsonCString(jsonCompatible(response))
    return AU_OK
}

@_cdecl("au_auaudiounit_can_perform_input")
public func au_auaudiounit_can_perform_input(_ ptr: UnsafeMutableRawPointer) -> Bool {
    borrowBox(ptr, as: AUAudioUnit.self).canPerformInput
}

@_cdecl("au_auaudiounit_can_perform_output")
public func au_auaudiounit_can_perform_output(_ ptr: UnsafeMutableRawPointer) -> Bool {
    borrowBox(ptr, as: AUAudioUnit.self).canPerformOutput
}

@_cdecl("au_auaudiounit_set_device_id")
public func au_auaudiounit_set_device_id(
    _ ptr: UnsafeMutableRawPointer,
    _ deviceID: UInt32,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    do {
        try borrowBox(ptr, as: AUAudioUnit.self).setDeviceID(deviceID)
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_PROPERTY_ERROR
    }
}

@_cdecl("au_auaudiounit_start_hardware")
public func au_auaudiounit_start_hardware(
    _ ptr: UnsafeMutableRawPointer,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    do {
        try borrowBox(ptr, as: AUAudioUnit.self).startHardware()
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_PROPERTY_ERROR
    }
}

@_cdecl("au_auaudiounit_stop_hardware")
public func au_auaudiounit_stop_hardware(_ ptr: UnsafeMutableRawPointer) {
    borrowBox(ptr, as: AUAudioUnit.self).stopHardware()
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
