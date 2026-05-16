import AudioToolbox
import AVFAudio
import Foundation

final class BridgeAUAudioUnitFactory: NSObject, AUAudioUnitFactory {
    func beginRequest(with _: NSExtensionContext) {}

    func createAudioUnit(with componentDescription: AudioComponentDescription) throws -> AUAudioUnit {
        switch instantiateAUAudioUnitSync(description: componentDescription, options: []) {
        case let .success(unit):
            return unit
        case let .failure(error):
            throw error
        }
    }
}

@_cdecl("au_factory_create")
public func au_factory_create() -> UnsafeMutableRawPointer {
    retainBox(BridgeAUAudioUnitFactory())
}

@_cdecl("au_factory_release")
public func au_factory_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: BridgeAUAudioUnitFactory.self)
}

@_cdecl("au_factory_create_audio_unit")
public func au_factory_create_audio_unit(
    _ ptr: UnsafeMutableRawPointer,
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
    let factory = borrowBox(ptr, as: BridgeAUAudioUnitFactory.self)
    do {
        let unit = try factory.createAudioUnit(with: makeDesc(type, subtype, manufacturer, flags, flagsMask))
        outUnit.pointee = retainBox(unit)
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_INSTANTIATE_FAILED
    }
}
