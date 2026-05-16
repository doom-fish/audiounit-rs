import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnitVarispeed(_ effect: AVAudioUnitVarispeed) -> [String: Any] {
    var dict = encodeAvAudioUnitTimeEffect(effect)
    dict["rate"] = effect.rate
    return dict
}

@_cdecl("au_av_varispeed_create")
public func au_av_varispeed_create(
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    outUnit.pointee = retainBox(AVAudioUnitVarispeed())
    return AU_OK
}

@_cdecl("au_av_varispeed_release")
public func au_av_varispeed_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitVarispeed.self)
}

@_cdecl("au_av_varispeed_snapshot_json")
public func au_av_varispeed_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitVarispeed(borrowBox(ptr, as: AVAudioUnitVarispeed.self)))
}

@_cdecl("au_av_varispeed_get_rate")
public func au_av_varispeed_get_rate(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitVarispeed.self).rate
}

@_cdecl("au_av_varispeed_set_rate")
public func au_av_varispeed_set_rate(_ ptr: UnsafeMutableRawPointer, _ rate: Float) {
    borrowBox(ptr, as: AVAudioUnitVarispeed.self).rate = rate
}

@_cdecl("au_av_varispeed_as_time_effect")
public func au_av_varispeed_as_time_effect(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitVarispeed.self) as AVAudioUnitTimeEffect)
}
