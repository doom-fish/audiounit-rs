import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnitDelay(_ delay: AVAudioUnitDelay) -> [String: Any] {
    var dict = encodeAvAudioUnitEffect(delay)
    dict["delayTime"] = delay.delayTime
    dict["feedback"] = delay.feedback
    dict["lowPassCutoff"] = delay.lowPassCutoff
    dict["wetDryMix"] = delay.wetDryMix
    return dict
}

@_cdecl("au_av_delay_create")
public func au_av_delay_create(
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    outUnit.pointee = retainBox(AVAudioUnitDelay())
    return AU_OK
}

@_cdecl("au_av_delay_release")
public func au_av_delay_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitDelay.self)
}

@_cdecl("au_av_delay_snapshot_json")
public func au_av_delay_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitDelay(borrowBox(ptr, as: AVAudioUnitDelay.self)))
}

@_cdecl("au_av_delay_get_delay_time")
public func au_av_delay_get_delay_time(_ ptr: UnsafeMutableRawPointer) -> Double {
    borrowBox(ptr, as: AVAudioUnitDelay.self).delayTime
}

@_cdecl("au_av_delay_set_delay_time")
public func au_av_delay_set_delay_time(_ ptr: UnsafeMutableRawPointer, _ delayTime: Double) {
    borrowBox(ptr, as: AVAudioUnitDelay.self).delayTime = delayTime
}

@_cdecl("au_av_delay_get_feedback")
public func au_av_delay_get_feedback(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitDelay.self).feedback
}

@_cdecl("au_av_delay_set_feedback")
public func au_av_delay_set_feedback(_ ptr: UnsafeMutableRawPointer, _ feedback: Float) {
    borrowBox(ptr, as: AVAudioUnitDelay.self).feedback = feedback
}

@_cdecl("au_av_delay_get_low_pass_cutoff")
public func au_av_delay_get_low_pass_cutoff(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitDelay.self).lowPassCutoff
}

@_cdecl("au_av_delay_set_low_pass_cutoff")
public func au_av_delay_set_low_pass_cutoff(_ ptr: UnsafeMutableRawPointer, _ lowPassCutoff: Float) {
    borrowBox(ptr, as: AVAudioUnitDelay.self).lowPassCutoff = lowPassCutoff
}

@_cdecl("au_av_delay_get_wet_dry_mix")
public func au_av_delay_get_wet_dry_mix(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitDelay.self).wetDryMix
}

@_cdecl("au_av_delay_set_wet_dry_mix")
public func au_av_delay_set_wet_dry_mix(_ ptr: UnsafeMutableRawPointer, _ wetDryMix: Float) {
    borrowBox(ptr, as: AVAudioUnitDelay.self).wetDryMix = wetDryMix
}

@_cdecl("au_av_delay_as_effect")
public func au_av_delay_as_effect(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitDelay.self) as AVAudioUnitEffect)
}
