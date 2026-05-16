import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnitReverb(_ reverb: AVAudioUnitReverb) -> [String: Any] {
    var dict = encodeAvAudioUnitEffect(reverb)
    dict["wetDryMix"] = reverb.wetDryMix
    return dict
}

@_cdecl("au_av_reverb_create")
public func au_av_reverb_create(
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    outUnit.pointee = retainBox(AVAudioUnitReverb())
    return AU_OK
}

@_cdecl("au_av_reverb_release")
public func au_av_reverb_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitReverb.self)
}

@_cdecl("au_av_reverb_snapshot_json")
public func au_av_reverb_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitReverb(borrowBox(ptr, as: AVAudioUnitReverb.self)))
}

@_cdecl("au_av_reverb_load_factory_preset")
public func au_av_reverb_load_factory_preset(_ ptr: UnsafeMutableRawPointer, _ preset: Int64) {
    let value = AVAudioUnitReverbPreset(rawValue: Int(preset)) ?? .mediumHall
    borrowBox(ptr, as: AVAudioUnitReverb.self).loadFactoryPreset(value)
}

@_cdecl("au_av_reverb_get_wet_dry_mix")
public func au_av_reverb_get_wet_dry_mix(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitReverb.self).wetDryMix
}

@_cdecl("au_av_reverb_set_wet_dry_mix")
public func au_av_reverb_set_wet_dry_mix(_ ptr: UnsafeMutableRawPointer, _ wetDryMix: Float) {
    borrowBox(ptr, as: AVAudioUnitReverb.self).wetDryMix = wetDryMix
}

@_cdecl("au_av_reverb_as_effect")
public func au_av_reverb_as_effect(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitReverb.self) as AVAudioUnitEffect)
}
