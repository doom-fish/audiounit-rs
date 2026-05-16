import AudioToolbox
import AVFAudio
import Foundation

@_cdecl("au_parameter_group_release")
public func au_parameter_group_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AUParameterGroup.self)
}

@_cdecl("au_parameter_group_snapshot_json")
public func au_parameter_group_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeParameterNode(borrowBox(ptr, as: AUParameterGroup.self)))
}

@_cdecl("au_parameter_group_children_json")
public func au_parameter_group_children_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let group = borrowBox(ptr, as: AUParameterGroup.self)
    return jsonCString(group.children.map(encodeParameterNode))
}

@_cdecl("au_parameter_group_all_parameters_json")
public func au_parameter_group_all_parameters_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let group = borrowBox(ptr, as: AUParameterGroup.self)
    return jsonCString(group.allParameters.map(encodeParameter))
}
