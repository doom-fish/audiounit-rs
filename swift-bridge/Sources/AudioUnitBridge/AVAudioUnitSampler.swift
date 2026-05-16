import AudioToolbox
import AVFAudio
import Foundation

func encodeAvAudioUnitSampler(_ sampler: AVAudioUnitSampler) -> [String: Any] {
    var dict = encodeAvAudioUnitMIDIInstrument(sampler)
    dict["stereoPan"] = sampler.stereoPan
    dict["overallGain"] = sampler.overallGain
    dict["globalTuning"] = sampler.globalTuning
    return dict
}

@_cdecl("au_av_sampler_create")
public func au_av_sampler_create(
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    outUnit.pointee = retainBox(AVAudioUnitSampler())
    return AU_OK
}

@_cdecl("au_av_sampler_release")
public func au_av_sampler_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitSampler.self)
}

@_cdecl("au_av_sampler_snapshot_json")
public func au_av_sampler_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitSampler(borrowBox(ptr, as: AVAudioUnitSampler.self)))
}

@_cdecl("au_av_sampler_get_stereo_pan")
public func au_av_sampler_get_stereo_pan(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitSampler.self).stereoPan
}

@_cdecl("au_av_sampler_set_stereo_pan")
public func au_av_sampler_set_stereo_pan(_ ptr: UnsafeMutableRawPointer, _ stereoPan: Float) {
    borrowBox(ptr, as: AVAudioUnitSampler.self).stereoPan = stereoPan
}

@_cdecl("au_av_sampler_get_overall_gain")
public func au_av_sampler_get_overall_gain(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitSampler.self).overallGain
}

@_cdecl("au_av_sampler_set_overall_gain")
public func au_av_sampler_set_overall_gain(_ ptr: UnsafeMutableRawPointer, _ overallGain: Float) {
    borrowBox(ptr, as: AVAudioUnitSampler.self).overallGain = overallGain
}

@_cdecl("au_av_sampler_get_global_tuning")
public func au_av_sampler_get_global_tuning(_ ptr: UnsafeMutableRawPointer) -> Float {
    borrowBox(ptr, as: AVAudioUnitSampler.self).globalTuning
}

@_cdecl("au_av_sampler_set_global_tuning")
public func au_av_sampler_set_global_tuning(_ ptr: UnsafeMutableRawPointer, _ globalTuning: Float) {
    borrowBox(ptr, as: AVAudioUnitSampler.self).globalTuning = globalTuning
}

@_cdecl("au_av_sampler_load_sound_bank_instrument")
public func au_av_sampler_load_sound_bank_instrument(
    _ ptr: UnsafeMutableRawPointer,
    _ path: UnsafePointer<CChar>?,
    _ program: UInt8,
    _ bankMSB: UInt8,
    _ bankLSB: UInt8,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    guard let path else {
        setError(outErrorMsg, "sound bank path was null")
        return AU_INVALID_ARGUMENT
    }

    let url = URL(fileURLWithPath: String(cString: path))
    do {
        try borrowBox(ptr, as: AVAudioUnitSampler.self).loadSoundBankInstrument(
            at: url,
            program: program,
            bankMSB: bankMSB,
            bankLSB: bankLSB
        )
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_UNAVAILABLE
    }
}

@_cdecl("au_av_sampler_load_instrument")
public func au_av_sampler_load_instrument(
    _ ptr: UnsafeMutableRawPointer,
    _ path: UnsafePointer<CChar>?,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outErrorMsg.pointee = nil
    guard let path else {
        setError(outErrorMsg, "instrument path was null")
        return AU_INVALID_ARGUMENT
    }

    let url = URL(fileURLWithPath: String(cString: path))
    do {
        try borrowBox(ptr, as: AVAudioUnitSampler.self).loadInstrument(at: url)
        return AU_OK
    } catch {
        setError(outErrorMsg, error.localizedDescription)
        return AU_UNAVAILABLE
    }
}

@_cdecl("au_av_sampler_as_midi_instrument")
public func au_av_sampler_as_midi_instrument(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitSampler.self) as AVAudioUnitMIDIInstrument)
}
