import AudioToolbox
import AVFAudio
import Foundation

func encodeParameter(_ parameter: AUParameter) -> [String: Any] {
    [
        "kind": "parameter",
        "identifier": parameter.identifier,
        "keyPath": parameter.keyPath,
        "displayName": parameter.displayName,
        "minValue": parameter.minValue,
        "maxValue": parameter.maxValue,
        "unit": parameter.unit.rawValue,
        "unitName": jsonValue(parameter.unitName),
        "flags": parameter.flags.rawValue,
        "address": parameter.address,
        "valueStrings": jsonValue(parameter.valueStrings),
        "dependentParameters": jsonValue(parameter.dependentParameters?.map(\.uint64Value)),
        "value": parameter.value,
    ]
}

@_cdecl("au_parameter_release")
public func au_parameter_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AUParameter.self)
}

@_cdecl("au_parameter_snapshot_json")
public func au_parameter_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeParameter(borrowBox(ptr, as: AUParameter.self)))
}

@_cdecl("au_parameter_get_value")
public func au_parameter_get_value(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AUParameter.self).value
}

@_cdecl("au_parameter_set_value")
public func au_parameter_set_value(_ ptr: UnsafeMutableRawPointer, _ value: Float) {
    borrowBox(ptr, as: AUParameter.self).value = value
}

@_cdecl("au_parameter_identifier")
public func au_parameter_identifier(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(borrowBox(ptr, as: AUParameter.self).identifier)
}

@_cdecl("au_parameter_display_name")
public func au_parameter_display_name(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(borrowBox(ptr, as: AUParameter.self).displayName)
}

@_cdecl("au_parameter_display_name_with_length")
public func au_parameter_display_name_with_length(
    _ ptr: UnsafeMutableRawPointer,
    _ length: Int
) -> UnsafeMutablePointer<CChar>? {
    ffiString(borrowBox(ptr, as: AUParameter.self).displayName(withLength: length))
}

@_cdecl("au_parameter_address")
public func au_parameter_address(_ ptr: UnsafeMutableRawPointer) -> UInt64 {
    UInt64(borrowBox(ptr, as: AUParameter.self).address)
}

@_cdecl("au_parameter_min_value")
public func au_parameter_min_value(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AUParameter.self).minValue
}

@_cdecl("au_parameter_max_value")
public func au_parameter_max_value(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AUParameter.self).maxValue
}

@_cdecl("au_parameter_unit")
public func au_parameter_unit(_ ptr: UnsafeMutableRawPointer) -> UInt32 {
    borrowBox(ptr, as: AUParameter.self).unit.rawValue
}

@_cdecl("au_parameter_string_from_value")
public func au_parameter_string_from_value(
    _ ptr: UnsafeMutableRawPointer,
    _ value: Float
) -> UnsafeMutablePointer<CChar>? {
    withUnsafePointer(to: value) { pointer in
        ffiString(borrowBox(ptr, as: AUParameter.self).string(fromValue: pointer))
    }
}

@_cdecl("au_parameter_value_from_string")
public func au_parameter_value_from_string(
    _ ptr: UnsafeMutableRawPointer,
    _ value: UnsafePointer<CChar>
) -> Float {
    borrowBox(ptr, as: AUParameter.self).value(from: String(cString: value))
}

@_cdecl("au_parameter_set_value_at_host_time")
public func au_parameter_set_value_at_host_time(
    _ ptr: UnsafeMutableRawPointer,
    _ value: Float,
    _ hostTime: UInt64
) {
    borrowBox(ptr, as: AUParameter.self).setValue(value, originator: nil, atHostTime: hostTime)
}

@_cdecl("au_parameter_set_value_with_event")
public func au_parameter_set_value_with_event(
    _ ptr: UnsafeMutableRawPointer,
    _ value: Float,
    _ hostTime: UInt64,
    _ eventType: UInt32
) {
    let event = AUParameterAutomationEventType(rawValue: eventType) ?? .value
    borrowBox(ptr, as: AUParameter.self).setValue(value, originator: nil, atHostTime: hostTime, eventType: event)
}
