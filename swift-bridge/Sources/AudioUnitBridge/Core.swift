import AudioToolbox
import AVFAudio
import Foundation

// MARK: - Status codes

let AU_OK: Int32 = 0
let AU_INVALID_ARGUMENT: Int32 = -1
let AU_INSTANTIATE_FAILED: Int32 = -2
let AU_TIMED_OUT: Int32 = -3
let AU_PROPERTY_ERROR: Int32 = -4
let AU_UNKNOWN: Int32 = -99

// MARK: - Retained-pointer helpers

@inline(__always)
func auRetain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
func auBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@inline(__always)
func auRelease<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type) {
    Unmanaged<T>.fromOpaque(ptr).release()
}

// MARK: - CString helpers

@inline(__always)
func ffiString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    strdup(string)
}

@_cdecl("au_string_free")
public func au_string_free(_ ptr: UnsafeMutablePointer<CChar>?) {
    guard let ptr else { return }
    free(ptr)
}

// MARK: - AudioComponentDescription builder (internal helper)

@inline(__always)
func makeDesc(
    _ type: UInt32, _ subtype: UInt32, _ manufacturer: UInt32,
    _ flags: UInt32, _ flagsMask: UInt32
) -> AudioComponentDescription {
    AudioComponentDescription(
        componentType: type,
        componentSubType: subtype,
        componentManufacturer: manufacturer,
        componentFlags: flags,
        componentFlagsMask: flagsMask
    )
}

// MARK: - Component enumeration

/// Returns the number of components matching the given description.
/// Pass all-zeros to enumerate everything.
@_cdecl("au_component_count")
public func au_component_count(
    _ type: UInt32, _ subtype: UInt32, _ manufacturer: UInt32,
    _ flags: UInt32, _ flagsMask: UInt32
) -> UInt32 {
    var desc = makeDesc(type, subtype, manufacturer, flags, flagsMask)
    return AudioComponentCount(&desc)
}

/// Writes up to `maxCount` AudioComponent opaque handles into `out`.
/// Returns the number actually written.
@_cdecl("au_component_list")
public func au_component_list(
    _ type: UInt32, _ subtype: UInt32, _ manufacturer: UInt32,
    _ flags: UInt32, _ flagsMask: UInt32,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ maxCount: Int
) -> Int {
    var desc = makeDesc(type, subtype, manufacturer, flags, flagsMask)
    var count = 0
    var comp: AudioComponent? = AudioComponentFindNext(nil, &desc)
    while let c = comp, count < maxCount {
        out.advanced(by: count).pointee = UnsafeMutableRawPointer(c)
        count += 1
        comp = AudioComponentFindNext(c, &desc)
    }
    return count
}

// MARK: - Legacy AudioComponent info helpers

/// Returns the name of an AudioComponent as a heap-allocated C string (caller frees).
@_cdecl("au_component_copy_name")
public func au_component_copy_name(_ compPtr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let comp = AudioComponent(compPtr)
    var cfName: Unmanaged<CFString>?
    guard AudioComponentCopyName(comp, &cfName) == noErr,
          let name = cfName?.takeRetainedValue() as String? else {
        return nil
    }
    return ffiString(name)
}

/// Fills individual out-params with the AudioComponentDescription fields.
/// Returns 0 on success.
@_cdecl("au_component_get_description")
public func au_component_get_description(
    _ compPtr: UnsafeMutableRawPointer,
    _ outType: UnsafeMutablePointer<UInt32>,
    _ outSubtype: UnsafeMutablePointer<UInt32>,
    _ outManufacturer: UnsafeMutablePointer<UInt32>,
    _ outFlags: UnsafeMutablePointer<UInt32>,
    _ outFlagsMask: UnsafeMutablePointer<UInt32>
) -> Int32 {
    let comp = AudioComponent(compPtr)
    var desc = AudioComponentDescription()
    let status = AudioComponentGetDescription(comp, &desc)
    if status != noErr { return status }
    outType.pointee = desc.componentType
    outSubtype.pointee = desc.componentSubType
    outManufacturer.pointee = desc.componentManufacturer
    outFlags.pointee = desc.componentFlags
    outFlagsMask.pointee = desc.componentFlagsMask
    return AU_OK
}

/// Returns version of the component.
@_cdecl("au_component_get_version")
public func au_component_get_version(_ compPtr: UnsafeMutableRawPointer) -> UInt32 {
    let comp = AudioComponent(compPtr)
    var version: UInt32 = 0
    AudioComponentGetVersion(comp, &version)
    return version
}

// MARK: - AVAudioUnitComponentManager helpers

/// Returns heap-allocated array of retained AVAudioUnitComponent pointers.
/// Writes count into `outCount`. Caller frees with `au_avc_component_array_free`.
@_cdecl("au_avc_manager_components_matching")
public func au_avc_manager_components_matching(
    _ type: UInt32, _ subtype: UInt32, _ manufacturer: UInt32,
    _ flags: UInt32, _ flagsMask: UInt32,
    _ outCount: UnsafeMutablePointer<Int>
) -> UnsafeMutablePointer<UnsafeMutableRawPointer?>? {
    let desc = makeDesc(type, subtype, manufacturer, flags, flagsMask)
    let mgr = AVAudioUnitComponentManager.shared()
    let components = mgr.components(matching: desc)
    let n = components.count
    outCount.pointee = n
    if n == 0 { return nil }
    let buf = UnsafeMutablePointer<UnsafeMutableRawPointer?>.allocate(capacity: n)
    for (i, comp) in components.enumerated() {
        buf.advanced(by: i).pointee = auRetain(comp)
    }
    return buf
}

/// Frees the array returned by `au_avc_manager_components_matching`.
@_cdecl("au_avc_component_array_free")
public func au_avc_component_array_free(
    _ buf: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) {
    guard let buf else { return }
    for i in 0..<count {
        if let ptr = buf.advanced(by: i).pointee {
            auRelease(ptr, as: AVAudioUnitComponent.self)
        }
    }
    buf.deallocate()
}

// MARK: - AVAudioUnitComponent property accessors

@_cdecl("au_avc_component_name")
public func au_avc_component_name(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let c: AVAudioUnitComponent = auBorrow(ptr, as: AVAudioUnitComponent.self)
    return ffiString(c.name)
}

@_cdecl("au_avc_component_type_name")
public func au_avc_component_type_name(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let c: AVAudioUnitComponent = auBorrow(ptr, as: AVAudioUnitComponent.self)
    return ffiString(c.typeName)
}

@_cdecl("au_avc_component_manufacturer_name")
public func au_avc_component_manufacturer_name(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let c: AVAudioUnitComponent = auBorrow(ptr, as: AVAudioUnitComponent.self)
    return ffiString(c.manufacturerName)
}

@_cdecl("au_avc_component_version")
public func au_avc_component_version(_ ptr: UnsafeMutableRawPointer) -> UInt32 {
    let c: AVAudioUnitComponent = auBorrow(ptr, as: AVAudioUnitComponent.self)
    return UInt32(c.version)
}

@_cdecl("au_avc_component_version_string")
public func au_avc_component_version_string(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let c: AVAudioUnitComponent = auBorrow(ptr, as: AVAudioUnitComponent.self)
    return ffiString(c.versionString)
}

@_cdecl("au_avc_component_has_custom_view")
public func au_avc_component_has_custom_view(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let c: AVAudioUnitComponent = auBorrow(ptr, as: AVAudioUnitComponent.self)
    return c.hasCustomView
}

@_cdecl("au_avc_component_sandbox_safe")
public func au_avc_component_sandbox_safe(_ ptr: UnsafeMutableRawPointer) -> Bool {
    let c: AVAudioUnitComponent = auBorrow(ptr, as: AVAudioUnitComponent.self)
    return c.isSandboxSafe
}

/// Fills individual out-params with the AudioComponentDescription fields.
@_cdecl("au_avc_component_audio_component_description")
public func au_avc_component_audio_component_description(
    _ ptr: UnsafeMutableRawPointer,
    _ outType: UnsafeMutablePointer<UInt32>,
    _ outSubtype: UnsafeMutablePointer<UInt32>,
    _ outManufacturer: UnsafeMutablePointer<UInt32>,
    _ outFlags: UnsafeMutablePointer<UInt32>,
    _ outFlagsMask: UnsafeMutablePointer<UInt32>
) {
    let c: AVAudioUnitComponent = auBorrow(ptr, as: AVAudioUnitComponent.self)
    let d = c.audioComponentDescription
    outType.pointee = d.componentType
    outSubtype.pointee = d.componentSubType
    outManufacturer.pointee = d.componentManufacturer
    outFlags.pointee = d.componentFlags
    outFlagsMask.pointee = d.componentFlagsMask
}

@_cdecl("au_avc_component_release")
public func au_avc_component_release(_ ptr: UnsafeMutableRawPointer) {
    auRelease(ptr, as: AVAudioUnitComponent.self)
}

// MARK: - AVAudioUnit instantiation (sync via DispatchSemaphore)

/// Instantiate an AVAudioUnit synchronously.
/// On error, writes a description into outErrorMsg (caller frees with au_string_free).
@_cdecl("au_instantiate_sync")
public func au_instantiate_sync(
    _ type: UInt32, _ subtype: UInt32, _ manufacturer: UInt32,
    _ flags: UInt32, _ flagsMask: UInt32,
    _ options: UInt32,
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil

    let desc = makeDesc(type, subtype, manufacturer, flags, flagsMask)
    var resultUnit: AVAudioUnit?
    var resultError: Error?
    let sem = DispatchSemaphore(value: 0)

    AVAudioUnit.instantiate(
        with: desc,
        options: AudioComponentInstantiationOptions(rawValue: options)
    ) { unit, error in
        resultUnit = unit
        resultError = error
        sem.signal()
    }

    let waitResult = sem.wait(timeout: .now() + .seconds(15))
    if waitResult == .timedOut {
        outErrorMsg.pointee = ffiString("AVAudioUnit instantiation timed out after 15s")
        return AU_TIMED_OUT
    }

    if let error = resultError {
        outErrorMsg.pointee = ffiString(error.localizedDescription)
        return AU_INSTANTIATE_FAILED
    }

    guard let unit = resultUnit else {
        outErrorMsg.pointee = ffiString("AVAudioUnit instantiation returned nil without an error")
        return AU_INSTANTIATE_FAILED
    }

    outUnit.pointee = auRetain(unit)
    return AU_OK
}

@_cdecl("au_avunit_release")
public func au_avunit_release(_ ptr: UnsafeMutableRawPointer) {
    auRelease(ptr, as: AVAudioUnit.self)
}

/// Returns the underlying legacy AudioUnit pointer (not retained; valid while AVAudioUnit is alive).
@_cdecl("au_avunit_audio_unit")
public func au_avunit_audio_unit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    let unit: AVAudioUnit = auBorrow(ptr, as: AVAudioUnit.self)
    return UnsafeMutableRawPointer(unit.audioUnit)
}

/// Returns the AUAudioUnit (retained). Caller frees with au_auaudiounit_release.
@_cdecl("au_avunit_auaudiounit")
public func au_avunit_auaudiounit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    let unit: AVAudioUnit = auBorrow(ptr, as: AVAudioUnit.self)
    return auRetain(unit.auAudioUnit)
}

@_cdecl("au_auaudiounit_release")
public func au_auaudiounit_release(_ ptr: UnsafeMutableRawPointer) {
    auRelease(ptr, as: AUAudioUnit.self)
}

// MARK: - AUParameterTree / AUParameterNode / AUParameter

@_cdecl("au_auaudiounit_parameter_tree")
public func au_auaudiounit_parameter_tree(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let unit: AUAudioUnit = auBorrow(ptr, as: AUAudioUnit.self)
    guard let tree = unit.parameterTree else { return nil }
    return auRetain(tree)
}

@_cdecl("au_parameter_tree_release")
public func au_parameter_tree_release(_ ptr: UnsafeMutableRawPointer) {
    auRelease(ptr, as: AUParameterTree.self)
}

/// Returns JSON describing the full parameter tree. Caller frees with au_string_free.
@_cdecl("au_parameter_tree_json")
public func au_parameter_tree_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let tree: AUParameterTree = auBorrow(ptr, as: AUParameterTree.self)
    let json = encodeNode(tree)
    guard let data = try? JSONSerialization.data(withJSONObject: json, options: []),
          let s = String(data: data, encoding: .utf8) else {
        return ffiString("{}")
    }
    return ffiString(s)
}

private func encodeNode(_ node: AUParameterNode) -> [String: Any] {
    var dict: [String: Any] = [
        "identifier": node.identifier,
        "keyPath": node.keyPath,
        "displayName": node.displayName,
    ]
    if let group = node as? AUParameterGroup {
        dict["kind"] = "group"
        dict["children"] = group.children.map { encodeNode($0) }
    } else if let param = node as? AUParameter {
        dict["kind"] = "parameter"
        dict["address"] = param.address
        dict["minValue"] = param.minValue
        dict["maxValue"] = param.maxValue
        dict["unit"] = param.unit.rawValue
        dict["value"] = param.value
    } else {
        dict["kind"] = "node"
    }
    return dict
}

// MARK: - Parameter get/set

/// Returns an opaque AUParameter pointer (retained) for the given address, or nil.
@_cdecl("au_parameter_tree_parameter_with_address")
public func au_parameter_tree_parameter_with_address(
    _ treePtr: UnsafeMutableRawPointer,
    _ address: UInt64
) -> UnsafeMutableRawPointer? {
    let tree: AUParameterTree = auBorrow(treePtr, as: AUParameterTree.self)
    guard let param = tree.parameter(withAddress: AUParameterAddress(address)) else { return nil }
    return auRetain(param)
}

@_cdecl("au_parameter_release")
public func au_parameter_release(_ ptr: UnsafeMutableRawPointer) {
    auRelease(ptr, as: AUParameter.self)
}

@_cdecl("au_parameter_get_value")
public func au_parameter_get_value(_ ptr: UnsafeMutableRawPointer) -> Float {
    let param: AUParameter = auBorrow(ptr, as: AUParameter.self)
    return param.value
}

@_cdecl("au_parameter_set_value")
public func au_parameter_set_value(_ ptr: UnsafeMutableRawPointer, _ value: Float) {
    let param: AUParameter = auBorrow(ptr, as: AUParameter.self)
    param.value = value
}

@_cdecl("au_parameter_identifier")
public func au_parameter_identifier(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let param: AUParameter = auBorrow(ptr, as: AUParameter.self)
    return ffiString(param.identifier)
}

@_cdecl("au_parameter_display_name")
public func au_parameter_display_name(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let param: AUParameter = auBorrow(ptr, as: AUParameter.self)
    return ffiString(param.displayName)
}

@_cdecl("au_parameter_address")
public func au_parameter_address(_ ptr: UnsafeMutableRawPointer) -> UInt64 {
    let param: AUParameter = auBorrow(ptr, as: AUParameter.self)
    return UInt64(param.address)
}

@_cdecl("au_parameter_min_value")
public func au_parameter_min_value(_ ptr: UnsafeMutableRawPointer) -> Float {
    let param: AUParameter = auBorrow(ptr, as: AUParameter.self)
    return param.minValue
}

@_cdecl("au_parameter_max_value")
public func au_parameter_max_value(_ ptr: UnsafeMutableRawPointer) -> Float {
    let param: AUParameter = auBorrow(ptr, as: AUParameter.self)
    return param.maxValue
}

@_cdecl("au_parameter_unit")
public func au_parameter_unit(_ ptr: UnsafeMutableRawPointer) -> UInt32 {
    let param: AUParameter = auBorrow(ptr, as: AUParameter.self)
    return param.unit.rawValue
}

/// Returns string representation of the current value. Caller frees with au_string_free.
@_cdecl("au_parameter_string_from_value")
public func au_parameter_string_from_value(
    _ ptr: UnsafeMutableRawPointer,
    _ value: Float
) -> UnsafeMutablePointer<CChar>? {
    let param: AUParameter = auBorrow(ptr, as: AUParameter.self)
    return withUnsafePointer(to: value) { vPtr in
        ffiString(param.string(fromValue: vPtr))
    }
}
