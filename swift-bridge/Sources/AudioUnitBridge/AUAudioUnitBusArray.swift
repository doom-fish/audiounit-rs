import AudioToolbox
import AVFAudio
import Foundation

func encodeAuAudioUnitBusArray(_ busArray: AUAudioUnitBusArray) -> [String: Any] {
    let busses = (0 ..< busArray.count).map { encodeAuAudioUnitBus(busArray[$0]) }
    return [
        "count": busArray.count,
        "countChangeable": busArray.isCountChangeable,
        "busType": busArray.busType.rawValue,
        "busses": busses,
    ]
}

@_cdecl("au_bus_array_release")
public func au_bus_array_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AUAudioUnitBusArray.self)
}

@_cdecl("au_bus_array_snapshot_json")
public func au_bus_array_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAuAudioUnitBusArray(borrowBox(ptr, as: AUAudioUnitBusArray.self)))
}

@_cdecl("au_bus_array_bus_at")
public func au_bus_array_bus_at(_ ptr: UnsafeMutableRawPointer, _ index: Int) -> UnsafeMutableRawPointer? {
    let busArray = borrowBox(ptr, as: AUAudioUnitBusArray.self)
    guard index >= 0, index < busArray.count else { return nil }
    return retainBox(busArray[index])
}

@_cdecl("au_bus_array_set_bus_count")
public func au_bus_array_set_bus_count(
    _ ptr: UnsafeMutableRawPointer,
    _ count: Int,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    do {
        try borrowBox(ptr, as: AUAudioUnitBusArray.self).setBusCount(count)
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_PROPERTY_ERROR
    }
}
