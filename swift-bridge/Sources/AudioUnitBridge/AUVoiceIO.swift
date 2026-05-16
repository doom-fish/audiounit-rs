import AudioToolbox
import AVFAudio
import Foundation

private func voiceProperty(_ unit: AudioUnit, id: AudioUnitPropertyID) -> UInt32? {
    var value: UInt32 = 0
    var size = UInt32(MemoryLayout<UInt32>.size)
    let status = AudioUnitGetProperty(unit, id, AudioUnitScope(kAudioUnitScope_Global), 0, &value, &size)
    guard status == noErr else { return nil }
    return value
}

private func setVoiceProperty(_ unit: AudioUnit, id: AudioUnitPropertyID, value: UInt32) -> OSStatus {
    var mutableValue = value
    return AudioUnitSetProperty(unit, id, AudioUnitScope(kAudioUnitScope_Global), 0, &mutableValue, UInt32(MemoryLayout<UInt32>.size))
}

private func voiceDuckingConfiguration(_ unit: AudioUnit) -> AUVoiceIOOtherAudioDuckingConfiguration? {
    let defaultLevel = AUVoiceIOOtherAudioDuckingLevel(rawValue: 0)!
    var value = AUVoiceIOOtherAudioDuckingConfiguration(mEnableAdvancedDucking: DarwinBoolean(false), mDuckingLevel: defaultLevel)
    var size = UInt32(MemoryLayout<AUVoiceIOOtherAudioDuckingConfiguration>.size)
    let status = AudioUnitGetProperty(unit, kAUVoiceIOProperty_OtherAudioDuckingConfiguration, AudioUnitScope(kAudioUnitScope_Global), 0, &value, &size)
    guard status == noErr else { return nil }
    return value
}

private func setVoiceDuckingConfiguration(_ unit: AudioUnit, value: AUVoiceIOOtherAudioDuckingConfiguration) -> OSStatus {
    var mutableValue = value
    return AudioUnitSetProperty(unit, kAUVoiceIOProperty_OtherAudioDuckingConfiguration, AudioUnitScope(kAudioUnitScope_Global), 0, &mutableValue, UInt32(MemoryLayout<AUVoiceIOOtherAudioDuckingConfiguration>.size))
}

@_cdecl("au_voice_io_create")
public func au_voice_io_create(
    _ options: UInt32,
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    let result = instantiateAVAudioUnitSync(
        description: makeDesc(UInt32(kAudioUnitType_Output), UInt32(kAudioUnitSubType_VoiceProcessingIO), UInt32(kAudioUnitManufacturer_Apple), 0, 0),
        options: AudioComponentInstantiationOptions(rawValue: options)
    )
    switch result {
    case let .success(unit):
        outUnit.pointee = retainBox(unit)
        return AU_OK
    case let .failure(error):
        setError(outErrorMsg, error.localizedDescription)
        if (error as NSError).code == Int(AU_TIMED_OUT) {
            return AU_TIMED_OUT
        }
        return AU_INSTANTIATE_FAILED
    }
}

@_cdecl("au_voice_io_release")
public func au_voice_io_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnit.self)
}

@_cdecl("au_voice_io_as_avunit")
public func au_voice_io_as_avunit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnit.self))
}

@_cdecl("au_voice_io_snapshot_json")
public func au_voice_io_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let unit = borrowBox(ptr, as: AVAudioUnit.self)
    let audioUnit = unit.audioUnit
    let ducking = voiceDuckingConfiguration(audioUnit)
    return jsonCString([
        "avAudioUnit": encodeAvAudioUnit(unit),
        "bypassVoiceProcessing": voiceProperty(audioUnit, id: kAUVoiceIOProperty_BypassVoiceProcessing) == 1,
        "voiceProcessingEnableAGC": voiceProperty(audioUnit, id: kAUVoiceIOProperty_VoiceProcessingEnableAGC) == 1,
        "muteOutput": voiceProperty(audioUnit, id: kAUVoiceIOProperty_MuteOutput) == 1,
        "otherAudioDuckingConfiguration": jsonValue(ducking.map { [
            "enableAdvancedDucking": $0.mEnableAdvancedDucking.boolValue,
            "duckingLevel": Int($0.mDuckingLevel.rawValue),
        ] }),
    ])
}

@_cdecl("au_voice_io_get_bypass_voice_processing")
public func au_voice_io_get_bypass_voice_processing(_ ptr: UnsafeMutableRawPointer) -> Bool {
    voiceProperty(borrowBox(ptr, as: AVAudioUnit.self).audioUnit, id: kAUVoiceIOProperty_BypassVoiceProcessing) == 1
}

@_cdecl("au_voice_io_set_bypass_voice_processing")
public func au_voice_io_set_bypass_voice_processing(_ ptr: UnsafeMutableRawPointer, _ value: Bool) -> Int32 {
    let status = setVoiceProperty(borrowBox(ptr, as: AVAudioUnit.self).audioUnit, id: kAUVoiceIOProperty_BypassVoiceProcessing, value: value ? 1 : 0)
    return status == noErr ? AU_OK : AU_PROPERTY_ERROR
}

@_cdecl("au_voice_io_get_enable_agc")
public func au_voice_io_get_enable_agc(_ ptr: UnsafeMutableRawPointer) -> Bool {
    voiceProperty(borrowBox(ptr, as: AVAudioUnit.self).audioUnit, id: kAUVoiceIOProperty_VoiceProcessingEnableAGC) == 1
}

@_cdecl("au_voice_io_set_enable_agc")
public func au_voice_io_set_enable_agc(_ ptr: UnsafeMutableRawPointer, _ value: Bool) -> Int32 {
    let status = setVoiceProperty(borrowBox(ptr, as: AVAudioUnit.self).audioUnit, id: kAUVoiceIOProperty_VoiceProcessingEnableAGC, value: value ? 1 : 0)
    return status == noErr ? AU_OK : AU_PROPERTY_ERROR
}

@_cdecl("au_voice_io_get_mute_output")
public func au_voice_io_get_mute_output(_ ptr: UnsafeMutableRawPointer) -> Bool {
    voiceProperty(borrowBox(ptr, as: AVAudioUnit.self).audioUnit, id: kAUVoiceIOProperty_MuteOutput) == 1
}

@_cdecl("au_voice_io_set_mute_output")
public func au_voice_io_set_mute_output(_ ptr: UnsafeMutableRawPointer, _ value: Bool) -> Int32 {
    let status = setVoiceProperty(borrowBox(ptr, as: AVAudioUnit.self).audioUnit, id: kAUVoiceIOProperty_MuteOutput, value: value ? 1 : 0)
    return status == noErr ? AU_OK : AU_PROPERTY_ERROR
}

@_cdecl("au_voice_io_get_other_audio_ducking_json")
public func au_voice_io_get_other_audio_ducking_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    guard let ducking = voiceDuckingConfiguration(borrowBox(ptr, as: AVAudioUnit.self).audioUnit) else {
        return nil
    }
    return jsonCString([
        "enableAdvancedDucking": ducking.mEnableAdvancedDucking.boolValue,
        "duckingLevel": Int(ducking.mDuckingLevel.rawValue),
    ])
}

@_cdecl("au_voice_io_set_other_audio_ducking")
public func au_voice_io_set_other_audio_ducking(_ ptr: UnsafeMutableRawPointer, _ enableAdvanced: Bool, _ duckingLevel: UInt32) -> Int32 {
    let defaultLevel = AUVoiceIOOtherAudioDuckingLevel(rawValue: 0)!
    let config = AUVoiceIOOtherAudioDuckingConfiguration(
        mEnableAdvancedDucking: DarwinBoolean(enableAdvanced),
        mDuckingLevel: AUVoiceIOOtherAudioDuckingLevel(rawValue: duckingLevel) ?? defaultLevel
    )
    let status = setVoiceDuckingConfiguration(borrowBox(ptr, as: AVAudioUnit.self).audioUnit, value: config)
    return status == noErr ? AU_OK : AU_PROPERTY_ERROR
}
