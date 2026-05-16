import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnitEffect(_ effect: AVAudioUnitEffect) -> [String: Any] {
    var dict = encodeAvAudioUnit(effect)
    dict["bypass"] = effect.bypass
    return dict
}

@_cdecl("au_av_effect_create")
public func au_av_effect_create(
    _ type: UInt32,
    _ subtype: UInt32,
    _ manufacturer: UInt32,
    _ flags: UInt32,
    _ flagsMask: UInt32,
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    let desc = makeDesc(type, subtype, manufacturer, flags, flagsMask)
    let effect = AVAudioUnitEffect(audioComponentDescription: desc)
    outUnit.pointee = retainBox(effect)
    return AU_OK
}

@_cdecl("au_av_effect_release")
public func au_av_effect_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitEffect.self)
}

@_cdecl("au_av_effect_snapshot_json")
public func au_av_effect_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitEffect(borrowBox(ptr, as: AVAudioUnitEffect.self)))
}

@_cdecl("au_av_effect_get_bypass")
public func au_av_effect_get_bypass(_ ptr: UnsafeMutableRawPointer) -> Bool {
    borrowBox(ptr, as: AVAudioUnitEffect.self).bypass
}

@_cdecl("au_av_effect_set_bypass")
public func au_av_effect_set_bypass(_ ptr: UnsafeMutableRawPointer, _ bypass: Bool) {
    borrowBox(ptr, as: AVAudioUnitEffect.self).bypass = bypass
}

@_cdecl("au_av_effect_as_avunit")
public func au_av_effect_as_avunit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitEffect.self) as AVAudioUnit)
}
