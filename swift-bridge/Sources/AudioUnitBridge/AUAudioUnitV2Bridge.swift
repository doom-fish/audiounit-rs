import AudioToolbox
import AVFAudio
import Foundation

@_cdecl("au_v2_bridge_from_auaudiounit")
public func au_v2_bridge_from_auaudiounit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let unit = borrowBox(ptr, as: AUAudioUnit.self)
    guard let bridge = unit as? AUAudioUnitV2Bridge else { return nil }
    return retainBox(bridge)
}

@_cdecl("au_v2_bridge_release")
public func au_v2_bridge_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AUAudioUnitV2Bridge.self)
}

@_cdecl("au_v2_bridge_audio_unit")
public func au_v2_bridge_audio_unit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    UnsafeMutableRawPointer(borrowBox(ptr, as: AUAudioUnitV2Bridge.self).audioUnit)
}

@_cdecl("au_v2_bridge_snapshot_json")
public func au_v2_bridge_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let bridge = borrowBox(ptr, as: AUAudioUnitV2Bridge.self)
    return jsonCString([
        "audioUnitPointer": Int(bitPattern: UnsafeMutableRawPointer(bridge.audioUnit)),
        "componentDescription": encodeComponentDescription(bridge.componentDescription),
    ])
}
