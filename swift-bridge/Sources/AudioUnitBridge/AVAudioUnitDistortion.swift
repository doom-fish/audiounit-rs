import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnitDistortion(_ distortion: AVAudioUnitDistortion) -> [String: Any] {
    var dict = encodeAvAudioUnitEffect(distortion)
    dict["preGain"] = distortion.preGain
    dict["wetDryMix"] = distortion.wetDryMix
    return dict
}

@_cdecl("au_av_distortion_create")
public func au_av_distortion_create(
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    outUnit.pointee = retainBox(AVAudioUnitDistortion())
    return AU_OK
}

@_cdecl("au_av_distortion_release")
public func au_av_distortion_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitDistortion.self)
}

@_cdecl("au_av_distortion_snapshot_json")
public func au_av_distortion_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitDistortion(borrowBox(ptr, as: AVAudioUnitDistortion.self)))
}

@_cdecl("au_av_distortion_load_factory_preset")
public func au_av_distortion_load_factory_preset(_ ptr: UnsafeMutableRawPointer, _ preset: Int64) {
    let value = AVAudioUnitDistortionPreset(rawValue: Int(preset)) ?? .drumsBitBrush
    borrowBox(ptr, as: AVAudioUnitDistortion.self).loadFactoryPreset(value)
}

@_cdecl("au_av_distortion_get_pre_gain")
public func au_av_distortion_get_pre_gain(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitDistortion.self).preGain
}

@_cdecl("au_av_distortion_set_pre_gain")
public func au_av_distortion_set_pre_gain(_ ptr: UnsafeMutableRawPointer, _ preGain: Float) {
    borrowBox(ptr, as: AVAudioUnitDistortion.self).preGain = preGain
}

@_cdecl("au_av_distortion_get_wet_dry_mix")
public func au_av_distortion_get_wet_dry_mix(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitDistortion.self).wetDryMix
}

@_cdecl("au_av_distortion_set_wet_dry_mix")
public func au_av_distortion_set_wet_dry_mix(_ ptr: UnsafeMutableRawPointer, _ wetDryMix: Float) {
    borrowBox(ptr, as: AVAudioUnitDistortion.self).wetDryMix = wetDryMix
}

@_cdecl("au_av_distortion_as_effect")
public func au_av_distortion_as_effect(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitDistortion.self) as AVAudioUnitEffect)
}
