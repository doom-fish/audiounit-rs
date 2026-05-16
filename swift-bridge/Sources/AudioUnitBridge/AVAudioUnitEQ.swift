import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnitEqBand(_ band: AVAudioUnitEQFilterParameters) -> [String: Any] {
    [
        "filterType": band.filterType.rawValue,
        "frequency": band.frequency,
        "bandwidth": band.bandwidth,
        "gain": band.gain,
        "bypass": band.bypass,
    ]
}

func encodeAvAudioUnitEq(_ eq: AVAudioUnitEQ) -> [String: Any] {
    var dict = encodeAvAudioUnitEffect(eq)
    dict["globalGain"] = eq.globalGain
    dict["bandCount"] = eq.bands.count
    dict["bands"] = eq.bands.map(encodeAvAudioUnitEqBand)
    return dict
}

@_cdecl("au_av_eq_create")
public func au_av_eq_create(
    _ numberOfBands: Int,
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    outUnit.pointee = retainBox(AVAudioUnitEQ(numberOfBands: numberOfBands))
    return AU_OK
}

@_cdecl("au_av_eq_release")
public func au_av_eq_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitEQ.self)
}

@_cdecl("au_av_eq_snapshot_json")
public func au_av_eq_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitEq(borrowBox(ptr, as: AVAudioUnitEQ.self)))
}

@_cdecl("au_av_eq_get_global_gain")
public func au_av_eq_get_global_gain(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitEQ.self).globalGain
}

@_cdecl("au_av_eq_set_global_gain")
public func au_av_eq_set_global_gain(_ ptr: UnsafeMutableRawPointer, _ globalGain: Float) {
    borrowBox(ptr, as: AVAudioUnitEQ.self).globalGain = globalGain
}

@_cdecl("au_av_eq_band_count")
public func au_av_eq_band_count(_ ptr: UnsafeMutableRawPointer) -> Int {
    borrowBox(ptr, as: AVAudioUnitEQ.self).bands.count
}

@_cdecl("au_av_eq_band_at")
public func au_av_eq_band_at(_ ptr: UnsafeMutableRawPointer, _ index: Int) -> UnsafeMutableRawPointer? {
    let bands = borrowBox(ptr, as: AVAudioUnitEQ.self).bands
    guard index >= 0, index < bands.count else { return nil }
    return retainBox(bands[index])
}

@_cdecl("au_av_eq_as_effect")
public func au_av_eq_as_effect(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitEQ.self) as AVAudioUnitEffect)
}

@_cdecl("au_av_eq_band_release")
public func au_av_eq_band_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitEQFilterParameters.self)
}

@_cdecl("au_av_eq_band_snapshot_json")
public func au_av_eq_band_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitEqBand(borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self)))
}

@_cdecl("au_av_eq_band_get_filter_type")
public func au_av_eq_band_get_filter_type(_ ptr: UnsafeMutableRawPointer) -> Int64 {
    Int64(borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).filterType.rawValue)
}

@_cdecl("au_av_eq_band_set_filter_type")
public func au_av_eq_band_set_filter_type(_ ptr: UnsafeMutableRawPointer, _ filterType: Int64) {
    let rawValue = Int(filterType)
    borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).filterType = AVAudioUnitEQFilterType(rawValue: rawValue) ?? .parametric
}

@_cdecl("au_av_eq_band_get_frequency")
public func au_av_eq_band_get_frequency(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).frequency
}

@_cdecl("au_av_eq_band_set_frequency")
public func au_av_eq_band_set_frequency(_ ptr: UnsafeMutableRawPointer, _ frequency: Float) {
    borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).frequency = frequency
}

@_cdecl("au_av_eq_band_get_bandwidth")
public func au_av_eq_band_get_bandwidth(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).bandwidth
}

@_cdecl("au_av_eq_band_set_bandwidth")
public func au_av_eq_band_set_bandwidth(_ ptr: UnsafeMutableRawPointer, _ bandwidth: Float) {
    borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).bandwidth = bandwidth
}

@_cdecl("au_av_eq_band_get_gain")
public func au_av_eq_band_get_gain(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).gain
}

@_cdecl("au_av_eq_band_set_gain")
public func au_av_eq_band_set_gain(_ ptr: UnsafeMutableRawPointer, _ gain: Float) {
    borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).gain = gain
}

@_cdecl("au_av_eq_band_get_bypass")
public func au_av_eq_band_get_bypass(_ ptr: UnsafeMutableRawPointer) -> Bool {
    borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).bypass
}

@_cdecl("au_av_eq_band_set_bypass")
public func au_av_eq_band_set_bypass(_ ptr: UnsafeMutableRawPointer, _ bypass: Bool) {
    borrowBox(ptr, as: AVAudioUnitEQFilterParameters.self).bypass = bypass
}
