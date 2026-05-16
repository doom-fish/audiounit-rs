import AudioToolbox
import AVFAudio
import Foundation

public typealias AvAudioUnitComponentTestCallback = @convention(c) (
    UnsafeMutableRawPointer,
    UnsafeMutablePointer<Bool>?,
    UnsafeMutableRawPointer?
) -> Bool

private func retainComponentBuffer(_ components: [AVAudioUnitComponent]) -> UnsafeMutablePointer<UnsafeMutableRawPointer?>? {
    guard !components.isEmpty else { return nil }
    let buffer = UnsafeMutablePointer<UnsafeMutableRawPointer?>.allocate(capacity: components.count)
    for (index, component) in components.enumerated() {
        buffer.advanced(by: index).pointee = retainBox(component)
    }
    return buffer
}

private func stringContains(_ haystack: String, needle: String) -> Bool {
    haystack.localizedCaseInsensitiveContains(needle)
}

private func evaluateComponentPredicate(_ component: AVAudioUnitComponent, json: Any) -> Bool {
    guard let dictionary = json as? [String: Any],
          let kind = dictionary["kind"] as? String else {
        return false
    }

    switch kind {
    case "true":
        return true
    case "nameContains":
        guard let value = dictionary["value"] as? String else { return false }
        return stringContains(component.name, needle: value)
    case "typeNameContains":
        guard let value = dictionary["value"] as? String else { return false }
        return stringContains(component.typeName, needle: value)
    case "manufacturerNameContains":
        guard let value = dictionary["value"] as? String else { return false }
        return stringContains(component.manufacturerName, needle: value)
    case "userTagContains":
        guard let value = dictionary["value"] as? String else { return false }
        return component.userTagNames.contains { stringContains($0, needle: value) }
    case "allTagContains":
        guard let value = dictionary["value"] as? String else { return false }
        return component.allTagNames.contains { stringContains($0, needle: value) }
    case "hasCustomView":
        return component.hasCustomView == (dictionary["value"] as? Bool ?? false)
    case "sandboxSafe":
        return component.isSandboxSafe == (dictionary["value"] as? Bool ?? false)
    case "all":
        let predicates = dictionary["predicates"] as? [Any] ?? []
        return predicates.allSatisfy { evaluateComponentPredicate(component, json: $0) }
    case "any":
        let predicates = dictionary["predicates"] as? [Any] ?? []
        return predicates.contains { evaluateComponentPredicate(component, json: $0) }
    case "not":
        guard let predicate = dictionary["predicate"] else { return false }
        return !evaluateComponentPredicate(component, json: predicate)
    default:
        return false
    }
}

@_cdecl("au_component_count")
public func au_component_count(
    _ type: UInt32,
    _ subtype: UInt32,
    _ manufacturer: UInt32,
    _ flags: UInt32,
    _ flagsMask: UInt32
) -> UInt32 {
    var desc = makeDesc(type, subtype, manufacturer, flags, flagsMask)
    return AudioComponentCount(&desc)
}

@_cdecl("au_component_list")
public func au_component_list(
    _ type: UInt32,
    _ subtype: UInt32,
    _ manufacturer: UInt32,
    _ flags: UInt32,
    _ flagsMask: UInt32,
    _ out: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ maxCount: Int
) -> Int {
    var desc = makeDesc(type, subtype, manufacturer, flags, flagsMask)
    var count = 0
    var comp: AudioComponent? = AudioComponentFindNext(nil, &desc)
    while let current = comp, count < maxCount {
        out.advanced(by: count).pointee = UnsafeMutableRawPointer(current)
        count += 1
        comp = AudioComponentFindNext(current, &desc)
    }
    return count
}

@_cdecl("au_component_copy_name")
public func au_component_copy_name(_ compPtr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let comp = AudioComponent(compPtr)
    var name: Unmanaged<CFString>?
    guard AudioComponentCopyName(comp, &name) == noErr,
          let string = name?.takeRetainedValue() as String? else {
        return nil
    }
    return ffiString(string)
}

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
    guard status == noErr else { return status }
    outType.pointee = desc.componentType
    outSubtype.pointee = desc.componentSubType
    outManufacturer.pointee = desc.componentManufacturer
    outFlags.pointee = desc.componentFlags
    outFlagsMask.pointee = desc.componentFlagsMask
    return AU_OK
}

@_cdecl("au_component_get_version")
public func au_component_get_version(_ compPtr: UnsafeMutableRawPointer) -> UInt32 {
    let comp = AudioComponent(compPtr)
    var version: UInt32 = 0
    AudioComponentGetVersion(comp, &version)
    return version
}

@_cdecl("au_avc_manager_components_matching")
public func au_avc_manager_components_matching(
    _ type: UInt32,
    _ subtype: UInt32,
    _ manufacturer: UInt32,
    _ flags: UInt32,
    _ flagsMask: UInt32,
    _ outCount: UnsafeMutablePointer<Int>
) -> UnsafeMutablePointer<UnsafeMutableRawPointer?>? {
    let desc = makeDesc(type, subtype, manufacturer, flags, flagsMask)
    let components = AVAudioUnitComponentManager.shared().components(matching: desc)
    outCount.pointee = components.count
    return retainComponentBuffer(components)
}

@_cdecl("au_avc_manager_components_matching_predicate")
public func au_avc_manager_components_matching_predicate(
    _ predicateJSON: UnsafePointer<CChar>?,
    _ outComponents: UnsafeMutablePointer<UnsafeMutablePointer<UnsafeMutableRawPointer?>?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outComponents.pointee = nil
    outCount.pointee = 0
    outErrorMsg.pointee = nil

    guard let predicateObject = jsonObject(from: predicateJSON) else {
        setError(outErrorMsg, "component predicate JSON was invalid")
        return AU_INVALID_ARGUMENT
    }

    let predicate = NSPredicate { object, _ in
        guard let component = object as? AVAudioUnitComponent else { return false }
        return evaluateComponentPredicate(component, json: predicateObject)
    }
    let components = AVAudioUnitComponentManager.shared().components(matching: predicate)
    outCount.pointee = components.count
    outComponents.pointee = retainComponentBuffer(components)
    return AU_OK
}

@_cdecl("au_avc_manager_components_passing_test")
public func au_avc_manager_components_passing_test(
    _ callback: AvAudioUnitComponentTestCallback?,
    _ context: UnsafeMutableRawPointer?,
    _ outComponents: UnsafeMutablePointer<UnsafeMutablePointer<UnsafeMutableRawPointer?>?>,
    _ outCount: UnsafeMutablePointer<Int>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outComponents.pointee = nil
    outCount.pointee = 0
    outErrorMsg.pointee = nil

    guard let callback else {
        setError(outErrorMsg, "component test callback was null")
        return AU_INVALID_ARGUMENT
    }

    let components = AVAudioUnitComponentManager.shared().components(passingTest: { component, stop in
        let retained = retainBox(component)
        defer { releaseBox(retained, as: AVAudioUnitComponent.self) }

        var rustStop = false
        let include = callback(retained, &rustStop, context)
        if rustStop {
            stop.pointee = ObjCBool(true)
        }
        return include
    })

    outCount.pointee = components.count
    outComponents.pointee = retainComponentBuffer(components)
    return AU_OK
}

@_cdecl("au_avc_component_array_free")
public func au_avc_component_array_free(
    _ buffer: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) {
    _ = count
    guard let buffer else { return }
    buffer.deallocate()
}

@_cdecl("au_avc_component_name")
public func au_avc_component_name(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(borrowBox(ptr, as: AVAudioUnitComponent.self).name)
}

@_cdecl("au_avc_component_type_name")
public func au_avc_component_type_name(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(borrowBox(ptr, as: AVAudioUnitComponent.self).typeName)
}

@_cdecl("au_avc_component_manufacturer_name")
public func au_avc_component_manufacturer_name(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(borrowBox(ptr, as: AVAudioUnitComponent.self).manufacturerName)
}

@_cdecl("au_avc_component_version")
public func au_avc_component_version(_ ptr: UnsafeMutableRawPointer) -> UInt32 {
    UInt32(borrowBox(ptr, as: AVAudioUnitComponent.self).version)
}

@_cdecl("au_avc_component_version_string")
public func au_avc_component_version_string(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    ffiString(borrowBox(ptr, as: AVAudioUnitComponent.self).versionString)
}

@_cdecl("au_avc_component_has_custom_view")
public func au_avc_component_has_custom_view(_ ptr: UnsafeMutableRawPointer) -> Bool {
    borrowBox(ptr, as: AVAudioUnitComponent.self).hasCustomView
}

@_cdecl("au_avc_component_sandbox_safe")
public func au_avc_component_sandbox_safe(_ ptr: UnsafeMutableRawPointer) -> Bool {
    borrowBox(ptr, as: AVAudioUnitComponent.self).isSandboxSafe
}

@_cdecl("au_avc_component_audio_component_description")
public func au_avc_component_audio_component_description(
    _ ptr: UnsafeMutableRawPointer,
    _ outType: UnsafeMutablePointer<UInt32>,
    _ outSubtype: UnsafeMutablePointer<UInt32>,
    _ outManufacturer: UnsafeMutablePointer<UInt32>,
    _ outFlags: UnsafeMutablePointer<UInt32>,
    _ outFlagsMask: UnsafeMutablePointer<UInt32>
) {
    let desc = borrowBox(ptr, as: AVAudioUnitComponent.self).audioComponentDescription
    outType.pointee = desc.componentType
    outSubtype.pointee = desc.componentSubType
    outManufacturer.pointee = desc.componentManufacturer
    outFlags.pointee = desc.componentFlags
    outFlagsMask.pointee = desc.componentFlagsMask
}

@_cdecl("au_avc_component_user_tag_names_json")
public func au_avc_component_user_tag_names_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(borrowBox(ptr, as: AVAudioUnitComponent.self).userTagNames)
}

@_cdecl("au_avc_component_set_user_tag_names_json")
public func au_avc_component_set_user_tag_names_json(
    _ ptr: UnsafeMutableRawPointer,
    _ tagsJSON: UnsafePointer<CChar>?,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    guard let tags = jsonObject(from: tagsJSON) as? [String] else {
        setError(outErrorMsg, "component tag JSON was invalid")
        return AU_INVALID_ARGUMENT
    }
    borrowBox(ptr, as: AVAudioUnitComponent.self).userTagNames = tags
    return AU_OK
}

@_cdecl("au_avc_component_all_tag_names_json")
public func au_avc_component_all_tag_names_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(borrowBox(ptr, as: AVAudioUnitComponent.self).allTagNames)
}

@_cdecl("au_avc_component_available_architectures_json")
public func au_avc_component_available_architectures_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let architectures = borrowBox(ptr, as: AVAudioUnitComponent.self).availableArchitectures.map { Int64($0.intValue) }
    return jsonCString(architectures)
}

@_cdecl("au_avc_component_configuration_dictionary_json")
public func au_avc_component_configuration_dictionary_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let dictionary = borrowBox(ptr, as: AVAudioUnitComponent.self).configurationDictionary
    return jsonCString(jsonCompatible(dictionary))
}

@_cdecl("au_avc_component_supports_number_input_channels")
public func au_avc_component_supports_number_input_channels(
    _ ptr: UnsafeMutableRawPointer,
    _ inputChannels: Int,
    _ outputChannels: Int
) -> Bool {
    borrowBox(ptr, as: AVAudioUnitComponent.self).supportsNumberInputChannels(inputChannels, outputChannels: outputChannels)
}

@_cdecl("au_avc_component_release")
public func au_avc_component_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitComponent.self)
}
