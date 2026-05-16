import AudioToolbox
import AVFAudio
import Foundation

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
    guard !components.isEmpty else { return nil }
    let buffer = UnsafeMutablePointer<UnsafeMutableRawPointer?>.allocate(capacity: components.count)
    for (index, component) in components.enumerated() {
        buffer.advanced(by: index).pointee = retainBox(component)
    }
    return buffer
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

@_cdecl("au_avc_component_release")
public func au_avc_component_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitComponent.self)
}
