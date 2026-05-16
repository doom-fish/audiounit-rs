import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnit(_ unit: AVAudioUnit) -> [String: Any] {
    [
        "audioComponentDescription": encodeComponentDescription(unit.audioComponentDescription),
        "name": unit.name,
        "manufacturerName": unit.manufacturerName,
        "version": Int(unit.version),
    ]
}

@_cdecl("au_instantiate_sync")
public func au_instantiate_sync(
    _ type: UInt32,
    _ subtype: UInt32,
    _ manufacturer: UInt32,
    _ flags: UInt32,
    _ flagsMask: UInt32,
    _ options: UInt32,
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil

    let result = instantiateAVAudioUnitSync(
        description: makeDesc(type, subtype, manufacturer, flags, flagsMask),
        options: AudioComponentInstantiationOptions(rawValue: options)
    )

    switch result {
    case let .success(unit):
        outUnit.pointee = retainBox(unit)
        return AU_OK
    case let .failure(error):
        setError(outErrorMsg, error.localizedDescription)
        if (error as NSError).code == Int(AU_TIMED_OUT) {
            return AU_TIMED_OUT
        }
        return AU_INSTANTIATE_FAILED
    }
}

@_cdecl("au_avunit_release")
public func au_avunit_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnit.self)
}

@_cdecl("au_avunit_audio_unit")
public func au_avunit_audio_unit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    UnsafeMutableRawPointer(borrowBox(ptr, as: AVAudioUnit.self).audioUnit)
}

@_cdecl("au_avunit_auaudiounit")
public func au_avunit_auaudiounit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnit.self).auAudioUnit)
}

@_cdecl("au_avunit_snapshot_json")
public func au_avunit_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnit(borrowBox(ptr, as: AVAudioUnit.self)))
}

@_cdecl("au_avunit_load_audio_unit_preset")
public func au_avunit_load_audio_unit_preset(
    _ ptr: UnsafeMutableRawPointer,
    _ path: UnsafePointer<CChar>?,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    guard let path else {
        setError(outErrorMsg, "preset path was null")
        return AU_INVALID_ARGUMENT
    }

    let url = URL(fileURLWithPath: String(cString: path))
    do {
        try borrowBox(ptr, as: AVAudioUnit.self).loadPreset(at: url)
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_UNAVAILABLE
    }
}
