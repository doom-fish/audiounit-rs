import AudioToolbox
import AVFAudio
import Foundation

private let parameterObserverLock = NSLock()
private var parameterValueObserverEvents: [ObjectIdentifier: [UInt: [[String: Any]]]] = [:]
private var parameterRecordingObserverEvents: [ObjectIdentifier: [UInt: [[String: Any]]]] = [:]
private var parameterAutomationObserverEvents: [ObjectIdentifier: [UInt: [[String: Any]]]] = [:]

private func parameterTreeKey(_ tree: AUParameterTree) -> ObjectIdentifier {
    ObjectIdentifier(tree)
}

private func parameterObserverTokenKey(_ token: AUParameterObserverToken?) -> UInt {
    guard let token else { return 0 }
    return UInt(bitPattern: token)
}

private func appendParameterObserverEvent(
    storage: inout [ObjectIdentifier: [UInt: [[String: Any]]]],
    treeKey: ObjectIdentifier,
    tokenKey: UInt,
    event: [String: Any]
) {
    var treeStorage = storage[treeKey] ?? [:]
    var events = treeStorage[tokenKey] ?? []
    events.append(event)
    treeStorage[tokenKey] = events
    storage[treeKey] = treeStorage
}

private func appendParameterObserverEvents(
    storage: inout [ObjectIdentifier: [UInt: [[String: Any]]]],
    treeKey: ObjectIdentifier,
    tokenKey: UInt,
    events newEvents: [[String: Any]]
) {
    var treeStorage = storage[treeKey] ?? [:]
    var events = treeStorage[tokenKey] ?? []
    events.append(contentsOf: newEvents)
    treeStorage[tokenKey] = events
    storage[treeKey] = treeStorage
}

private func drainParameterObserverEvents(
    storage: inout [ObjectIdentifier: [UInt: [[String: Any]]]],
    treeKey: ObjectIdentifier,
    tokenKey: UInt
) -> [[String: Any]] {
    var treeStorage = storage[treeKey] ?? [:]
    let events = treeStorage[tokenKey] ?? []
    treeStorage[tokenKey] = []
    storage[treeKey] = treeStorage
    return events
}

private func removeParameterObserverToken(_ treeKey: ObjectIdentifier, _ tokenKey: UInt) {
    parameterValueObserverEvents[treeKey]?[tokenKey] = nil
    parameterRecordingObserverEvents[treeKey]?[tokenKey] = nil
    parameterAutomationObserverEvents[treeKey]?[tokenKey] = nil
}

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

@_cdecl("au_parameter_tree_retain")
public func au_parameter_tree_retain(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AUParameterTree.self))
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

@_cdecl("au_parameter_tree_add_parameter_observer_capture")
public func au_parameter_tree_add_parameter_observer_capture(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let tree = borrowBox(ptr, as: AUParameterTree.self)
    let treeKey = parameterTreeKey(tree)
    var token: AUParameterObserverToken?
    token = tree.token(byAddingParameterObserver: { address, value in
        parameterObserverLock.lock()
        appendParameterObserverEvent(
            storage: &parameterValueObserverEvents,
            treeKey: treeKey,
            tokenKey: parameterObserverTokenKey(token),
            event: ["address": address, "value": value]
        )
        parameterObserverLock.unlock()
    })
    guard let token else { return nil }
    parameterObserverLock.lock()
    _ = parameterValueObserverEvents[treeKey, default: [:]][parameterObserverTokenKey(token)]
    parameterObserverLock.unlock()
    return token
}

@_cdecl("au_parameter_tree_add_parameter_recording_observer_capture")
public func au_parameter_tree_add_parameter_recording_observer_capture(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let tree = borrowBox(ptr, as: AUParameterTree.self)
    let treeKey = parameterTreeKey(tree)
    var token: AUParameterObserverToken?
    token = tree.token(byAddingParameterRecordingObserver: { numberEvents, events in
        guard numberEvents > 0 else { return }
        var captured: [[String: Any]] = []
        captured.reserveCapacity(numberEvents)
        for index in 0 ..< numberEvents {
            let event = events.advanced(by: index).pointee
            captured.append([
                "hostTime": event.hostTime,
                "address": event.address,
                "value": event.value,
            ])
        }
        parameterObserverLock.lock()
        appendParameterObserverEvents(
            storage: &parameterRecordingObserverEvents,
            treeKey: treeKey,
            tokenKey: parameterObserverTokenKey(token),
            events: captured
        )
        parameterObserverLock.unlock()
    })
    guard let token else { return nil }
    parameterObserverLock.lock()
    _ = parameterRecordingObserverEvents[treeKey, default: [:]][parameterObserverTokenKey(token)]
    parameterObserverLock.unlock()
    return token
}

@_cdecl("au_parameter_tree_add_parameter_automation_observer_capture")
public func au_parameter_tree_add_parameter_automation_observer_capture(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let tree = borrowBox(ptr, as: AUParameterTree.self)
    let treeKey = parameterTreeKey(tree)
    var token: AUParameterObserverToken?
    token = tree.token(byAddingParameterAutomationObserver: { numberEvents, events in
        guard numberEvents > 0 else { return }
        var captured: [[String: Any]] = []
        captured.reserveCapacity(numberEvents)
        for index in 0 ..< numberEvents {
            let event = events.advanced(by: index).pointee
            captured.append([
                "hostTime": event.hostTime,
                "address": event.address,
                "value": event.value,
                "eventType": event.eventType.rawValue,
            ])
        }
        parameterObserverLock.lock()
        appendParameterObserverEvents(
            storage: &parameterAutomationObserverEvents,
            treeKey: treeKey,
            tokenKey: parameterObserverTokenKey(token),
            events: captured
        )
        parameterObserverLock.unlock()
    })
    guard let token else { return nil }
    parameterObserverLock.lock()
    _ = parameterAutomationObserverEvents[treeKey, default: [:]][parameterObserverTokenKey(token)]
    parameterObserverLock.unlock()
    return token
}

@_cdecl("au_parameter_tree_take_parameter_observer_events_json")
public func au_parameter_tree_take_parameter_observer_events_json(
    _ ptr: UnsafeMutableRawPointer,
    _ token: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    let treeKey = parameterTreeKey(borrowBox(ptr, as: AUParameterTree.self))
    parameterObserverLock.lock()
    let events = drainParameterObserverEvents(
        storage: &parameterValueObserverEvents,
        treeKey: treeKey,
        tokenKey: parameterObserverTokenKey(token)
    )
    parameterObserverLock.unlock()
    return jsonCString(events)
}

@_cdecl("au_parameter_tree_take_parameter_recording_events_json")
public func au_parameter_tree_take_parameter_recording_events_json(
    _ ptr: UnsafeMutableRawPointer,
    _ token: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    let treeKey = parameterTreeKey(borrowBox(ptr, as: AUParameterTree.self))
    parameterObserverLock.lock()
    let events = drainParameterObserverEvents(
        storage: &parameterRecordingObserverEvents,
        treeKey: treeKey,
        tokenKey: parameterObserverTokenKey(token)
    )
    parameterObserverLock.unlock()
    return jsonCString(events)
}

@_cdecl("au_parameter_tree_take_parameter_automation_events_json")
public func au_parameter_tree_take_parameter_automation_events_json(
    _ ptr: UnsafeMutableRawPointer,
    _ token: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    let treeKey = parameterTreeKey(borrowBox(ptr, as: AUParameterTree.self))
    parameterObserverLock.lock()
    let events = drainParameterObserverEvents(
        storage: &parameterAutomationObserverEvents,
        treeKey: treeKey,
        tokenKey: parameterObserverTokenKey(token)
    )
    parameterObserverLock.unlock()
    return jsonCString(events)
}

@_cdecl("au_parameter_tree_remove_parameter_observer")
public func au_parameter_tree_remove_parameter_observer(
    _ ptr: UnsafeMutableRawPointer,
    _ token: UnsafeMutableRawPointer?
) {
    let tree = borrowBox(ptr, as: AUParameterTree.self)
    let treeKey = parameterTreeKey(tree)
    guard let token else { return }
    tree.removeParameterObserver(token)
    parameterObserverLock.lock()
    removeParameterObserverToken(treeKey, parameterObserverTokenKey(token))
    parameterObserverLock.unlock()
}
