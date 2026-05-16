import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnitTimePitch(_ effect: AVAudioUnitTimePitch) -> [String: Any] {
    var dict = encodeAvAudioUnitTimeEffect(effect)
    dict["rate"] = effect.rate
    dict["pitch"] = effect.pitch
    dict["overlap"] = effect.overlap
    return dict
}

@_cdecl("au_av_time_pitch_create")
public func au_av_time_pitch_create(
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    outUnit.pointee = retainBox(AVAudioUnitTimePitch())
    return AU_OK
}

@_cdecl("au_av_time_pitch_release")
public func au_av_time_pitch_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitTimePitch.self)
}

@_cdecl("au_av_time_pitch_snapshot_json")
public func au_av_time_pitch_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitTimePitch(borrowBox(ptr, as: AVAudioUnitTimePitch.self)))
}

@_cdecl("au_av_time_pitch_get_rate")
public func au_av_time_pitch_get_rate(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitTimePitch.self).rate
}

@_cdecl("au_av_time_pitch_set_rate")
public func au_av_time_pitch_set_rate(_ ptr: UnsafeMutableRawPointer, _ rate: Float) {
    borrowBox(ptr, as: AVAudioUnitTimePitch.self).rate = rate
}

@_cdecl("au_av_time_pitch_get_pitch")
public func au_av_time_pitch_get_pitch(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitTimePitch.self).pitch
}

@_cdecl("au_av_time_pitch_set_pitch")
public func au_av_time_pitch_set_pitch(_ ptr: UnsafeMutableRawPointer, _ pitch: Float) {
    borrowBox(ptr, as: AVAudioUnitTimePitch.self).pitch = pitch
}

@_cdecl("au_av_time_pitch_get_overlap")
public func au_av_time_pitch_get_overlap(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitTimePitch.self).overlap
}

@_cdecl("au_av_time_pitch_set_overlap")
public func au_av_time_pitch_set_overlap(_ ptr: UnsafeMutableRawPointer, _ overlap: Float) {
    borrowBox(ptr, as: AVAudioUnitTimePitch.self).overlap = overlap
}

@_cdecl("au_av_time_pitch_as_time_effect")
public func au_av_time_pitch_as_time_effect(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitTimePitch.self) as AVAudioUnitTimeEffect)
}
