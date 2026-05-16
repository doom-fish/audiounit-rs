import AudioToolbox
import AVFAudio
import Foundation

let AU_OK: Int32 = 0
let AU_INVALID_ARGUMENT: Int32 = -1
let AU_INSTANTIATE_FAILED: Int32 = -2
let AU_TIMED_OUT: Int32 = -3
let AU_PROPERTY_ERROR: Int32 = -4
let AU_UNAVAILABLE: Int32 = -5
let AU_UNKNOWN: Int32 = -99

final class Box<T> {
    let value: T

    init(_ value: T) {
        self.value = value
    }
}

@inline(__always)
func retainBox<T>(_ value: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(Box(value)).toOpaque()
}

@inline(__always)
func borrowBox<T>(_ ptr: UnsafeMutableRawPointer, as _: T.Type) -> T {
    Unmanaged<Box<T>>.fromOpaque(ptr).takeUnretainedValue().value
}

@inline(__always)
func releaseBox<T>(_ ptr: UnsafeMutableRawPointer, as _: T.Type) {
    Unmanaged<Box<T>>.fromOpaque(ptr).release()
}

@inline(__always)
func ffiString(_ string: String?) -> UnsafeMutablePointer<CChar>? {
    guard let string else { return nil }
    return strdup(string)
}

@_cdecl("au_string_free")
public func au_string_free(_ ptr: UnsafeMutablePointer<CChar>?) {
    guard let ptr else { return }
    free(ptr)
}

@inline(__always)
func makeDesc(
    _ type: UInt32,
    _ subtype: UInt32,
    _ manufacturer: UInt32,
    _ flags: UInt32,
    _ flagsMask: UInt32
) -> AudioComponentDescription {
    AudioComponentDescription(
        componentType: type,
        componentSubType: subtype,
        componentManufacturer: manufacturer,
        componentFlags: flags,
        componentFlagsMask: flagsMask
    )
}

func makeBridgeError(_ description: String, code: Int = Int(AU_UNKNOWN)) -> NSError {
    NSError(
        domain: "fish.doom.audiounit.bridge",
        code: code,
        userInfo: [NSLocalizedDescriptionKey: description]
    )
}

func setError(_ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>, _ message: String) {
    outError.pointee = ffiString(message)
}

func encodeComponentDescription(_ desc: AudioComponentDescription) -> [String: Any] {
    [
        "componentType": desc.componentType,
        "componentSubType": desc.componentSubType,
        "componentManufacturer": desc.componentManufacturer,
        "componentFlags": desc.componentFlags,
        "componentFlagsMask": desc.componentFlagsMask,
    ]
}

func encodeAudioFormat(_ format: AVAudioFormat) -> [String: Any] {
    [
        "sampleRate": format.sampleRate,
        "channelCount": Int(format.channelCount),
        "commonFormat": Int(format.commonFormat.rawValue),
        "interleaved": format.isInterleaved,
        "standard": format.isStandard,
        "settingsDescription": String(describing: format.settings),
    ]
}

func encodePreset(_ preset: AUAudioUnitPreset) -> [String: Any] {
    [
        "number": preset.number,
        "name": preset.name,
    ]
}

func jsonCString(_ object: Any) -> UnsafeMutablePointer<CChar>? {
    guard JSONSerialization.isValidJSONObject(object),
          let data = try? JSONSerialization.data(withJSONObject: object, options: []),
          let string = String(data: data, encoding: .utf8) else {
        return ffiString("null")
    }
    return ffiString(string)
}

func plistString(_ object: Any?) -> String? {
    guard let object,
          let data = try? PropertyListSerialization.data(fromPropertyList: object, format: .xml, options: 0),
          let string = String(data: data, encoding: .utf8) else {
        return nil
    }
    return string
}

func jsonValue(_ value: Any?) -> Any {
    value ?? NSNull()
}

func jsonCompatible(_ value: Any?) -> Any {
    guard let value else { return NSNull() }

    switch value {
    case let bool as Bool:
        return bool
    case let string as String:
        return string
    case let number as NSNumber:
        return number
    case let array as [Any]:
        return array.map(jsonCompatible)
    case let dictionary as [String: Any]:
        return dictionary.mapValues(jsonCompatible)
    case let dictionary as NSDictionary:
        var converted: [String: Any] = [:]
        for (key, value) in dictionary {
            converted[String(describing: key)] = jsonCompatible(value)
        }
        return converted
    case let data as Data:
        return ["type": "data", "base64": data.base64EncodedString()]
    case let url as URL:
        return url.path
    case let date as Date:
        return ISO8601DateFormatter().string(from: date)
    default:
        return String(describing: value)
    }
}

func jsonObject(from json: UnsafePointer<CChar>?) -> Any? {
    guard let json,
          let data = String(cString: json).data(using: .utf8) else {
        return nil
    }
    return try? JSONSerialization.jsonObject(with: data, options: [])
}

func instantiateAVAudioUnitSync(
    description: AudioComponentDescription,
    options: AudioComponentInstantiationOptions,
    timeoutSeconds: TimeInterval = 15
) -> Result<AVAudioUnit, Error> {
    let semaphore = DispatchSemaphore(value: 0)
    var result: Result<AVAudioUnit, Error>?

    AVAudioUnit.instantiate(with: description, options: options) { unit, error in
        if let unit {
            result = .success(unit)
        } else if let error {
            result = .failure(error)
        } else {
            result = .failure(makeBridgeError("AVAudioUnit instantiation returned nil without an error", code: Int(AU_INSTANTIATE_FAILED)))
        }
        semaphore.signal()
    }

    if semaphore.wait(timeout: .now() + timeoutSeconds) == .timedOut {
        return .failure(makeBridgeError("AVAudioUnit instantiation timed out after \(Int(timeoutSeconds))s", code: Int(AU_TIMED_OUT)))
    }

    return result ?? .failure(makeBridgeError("AVAudioUnit instantiation completed without a result", code: Int(AU_UNKNOWN)))
}

func instantiateAUAudioUnitSync(
    description: AudioComponentDescription,
    options: AudioComponentInstantiationOptions,
    timeoutSeconds: TimeInterval = 15
) -> Result<AUAudioUnit, Error> {
    let semaphore = DispatchSemaphore(value: 0)
    var result: Result<AUAudioUnit, Error>?

    AUAudioUnit.instantiate(with: description, options: options) { unit, error in
        if let unit {
            result = .success(unit)
        } else if let error {
            result = .failure(error)
        } else {
            result = .failure(makeBridgeError("AUAudioUnit instantiation returned nil without an error", code: Int(AU_INSTANTIATE_FAILED)))
        }
        semaphore.signal()
    }

    if semaphore.wait(timeout: .now() + timeoutSeconds) == .timedOut {
        return .failure(makeBridgeError("AUAudioUnit instantiation timed out after \(Int(timeoutSeconds))s", code: Int(AU_TIMED_OUT)))
    }

    return result ?? .failure(makeBridgeError("AUAudioUnit instantiation completed without a result", code: Int(AU_UNKNOWN)))
}
