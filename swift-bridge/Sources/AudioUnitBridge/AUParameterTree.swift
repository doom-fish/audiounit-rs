import AudioToolbox
import AVFAudio
import Foundation

func encodeParameterNode(_ node: AUParameterNode) -> [String: Any] {
    var dict: [String: Any] = [
        "identifier": node.identifier,
        "keyPath": node.keyPath,
        "displayName": node.displayName,
    ]

    if let group = node as? AUParameterGroup {
        dict["kind"] = group is AUParameterTree ? "tree" : "group"
        dict["children"] = group.children.map(encodeParameterNode)
        dict["allParameters"] = group.allParameters.map(encodeParameter)
    }

    if let parameter = node as? AUParameter {
        dict.merge(encodeParameter(parameter), uniquingKeysWith: { _, new in new })
    }

    return dict
}

@_cdecl("au_parameter_tree_release")
public func au_parameter_tree_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AUParameterTree.self)
}

@_cdecl("au_parameter_tree_snapshot_json")
public func au_parameter_tree_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeParameterNode(borrowBox(ptr, as: AUParameterTree.self)))
}

@_cdecl("au_parameter_tree_parameter_with_address")
public func au_parameter_tree_parameter_with_address(
    _ ptr: UnsafeMutableRawPointer,
    _ address: UInt64
) -> UnsafeMutableRawPointer? {
    let tree = borrowBox(ptr, as: AUParameterTree.self)
    guard let parameter = tree.parameter(withAddress: AUParameterAddress(address)) else { return nil }
    return retainBox(parameter)
}

@_cdecl("au_parameter_tree_parameter_with_id")
public func au_parameter_tree_parameter_with_id(
    _ ptr: UnsafeMutableRawPointer,
    _ parameterID: UInt32,
    _ scope: UInt32,
    _ element: UInt32
) -> UnsafeMutableRawPointer? {
    let tree = borrowBox(ptr, as: AUParameterTree.self)
    guard let parameter = tree.parameter(withID: parameterID, scope: scope, element: element) else { return nil }
    return retainBox(parameter)
}

@_cdecl("au_parameter_tree_root_group")
public func au_parameter_tree_root_group(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AUParameterTree.self) as AUParameterGroup)
}
