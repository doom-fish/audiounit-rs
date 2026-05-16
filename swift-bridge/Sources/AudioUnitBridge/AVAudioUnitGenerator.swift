import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnitGenerator(_ generator: AVAudioUnitGenerator) -> [String: Any] {
    var dict = encodeAvAudioUnit(generator)
    dict["bypass"] = generator.bypass
    return dict
}

@_cdecl("au_av_generator_create")
public func au_av_generator_create(
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
    let generator = AVAudioUnitGenerator(audioComponentDescription: desc)
    outUnit.pointee = retainBox(generator)
    return AU_OK
}

@_cdecl("au_av_generator_release")
public func au_av_generator_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitGenerator.self)
}

@_cdecl("au_av_generator_snapshot_json")
public func au_av_generator_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitGenerator(borrowBox(ptr, as: AVAudioUnitGenerator.self)))
}

@_cdecl("au_av_generator_get_bypass")
public func au_av_generator_get_bypass(_ ptr: UnsafeMutableRawPointer) -> Bool {
    borrowBox(ptr, as: AVAudioUnitGenerator.self).bypass
}

@_cdecl("au_av_generator_set_bypass")
public func au_av_generator_set_bypass(_ ptr: UnsafeMutableRawPointer, _ bypass: Bool) {
    borrowBox(ptr, as: AVAudioUnitGenerator.self).bypass = bypass
}

@_cdecl("au_av_generator_as_avunit")
public func au_av_generator_as_avunit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitGenerator.self) as AVAudioUnit)
}
