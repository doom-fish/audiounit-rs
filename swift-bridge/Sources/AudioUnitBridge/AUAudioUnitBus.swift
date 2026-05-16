import AudioToolbox
import AVFAudio
import Foundation

func encodeAuAudioUnitBus(_ bus: AUAudioUnitBus) -> [String: Any] {
    [
        "format": encodeAudioFormat(bus.format),
        "shouldAllocateBuffer": bus.shouldAllocateBuffer,
        "enabled": bus.isEnabled,
        "name": jsonValue(bus.name),
        "index": bus.index,
        "busType": bus.busType.rawValue,
        "supportedChannelLayoutTags": (bus.supportedChannelLayoutTags ?? []).map(\.intValue),
        "contextPresentationLatency": bus.contextPresentationLatency,
    ]
}

@_cdecl("au_bus_release")
public func au_bus_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AUAudioUnitBus.self)
}

@_cdecl("au_bus_snapshot_json")
public func au_bus_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAuAudioUnitBus(borrowBox(ptr, as: AUAudioUnitBus.self)))
}

@_cdecl("au_bus_set_standard_format")
public func au_bus_set_standard_format(
    _ ptr: UnsafeMutableRawPointer,
    _ sampleRate: Double,
    _ channelCount: UInt32,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    let bus = borrowBox(ptr, as: AUAudioUnitBus.self)
    guard let format = AVAudioFormat(standardFormatWithSampleRate: sampleRate, channels: AVAudioChannelCount(channelCount)) else {
        setError(outErrorMsg, "failed to build AVAudioFormat")
        return AU_INVALID_ARGUMENT
    }
    do {
        try bus.setFormat(format)
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_PROPERTY_ERROR
    }
}

@_cdecl("au_bus_set_should_allocate_buffer")
public func au_bus_set_should_allocate_buffer(_ ptr: UnsafeMutableRawPointer, _ value: Bool) {
    borrowBox(ptr, as: AUAudioUnitBus.self).shouldAllocateBuffer = value
}

@_cdecl("au_bus_set_enabled")
public func au_bus_set_enabled(_ ptr: UnsafeMutableRawPointer, _ value: Bool) {
    borrowBox(ptr, as: AUAudioUnitBus.self).isEnabled = value
}

@_cdecl("au_bus_set_name")
public func au_bus_set_name(_ ptr: UnsafeMutableRawPointer, _ value: UnsafePointer<CChar>?) {
    let bus = borrowBox(ptr, as: AUAudioUnitBus.self)
    bus.name = value.map(String.init(cString:))
}

@_cdecl("au_bus_set_context_presentation_latency")
public func au_bus_set_context_presentation_latency(_ ptr: UnsafeMutableRawPointer, _ value: Double) {
    borrowBox(ptr, as: AUAudioUnitBus.self).contextPresentationLatency = value
}

@_cdecl("au_bus_owner_audio_unit")
public func au_bus_owner_audio_unit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AUAudioUnitBus.self).ownerAudioUnit)
}
