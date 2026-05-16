import AudioToolbox
import AVFAudio
import CoreMIDI
import Foundation

func encodeAvAudioUnitMIDIInstrument(_ instrument: AVAudioUnitMIDIInstrument) -> [String: Any] {
    var dict = encodeAvAudioUnit(instrument)
    dict["supportsMIDIEventList"] = true
    return dict
}

@_cdecl("au_av_midi_instrument_create")
public func au_av_midi_instrument_create(
    _ type: UInt32,
    _ subtype: UInt32,
    _ manufacturer: UInt32,
    _ flags: UInt32,
    _ flagsMask: UInt32,
    _ outUnit: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMsg: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    outUnit.pointee = nil
    outErrorMsg.pointee = nil
    let desc = makeDesc(type, subtype, manufacturer, flags, flagsMask)
    let instrument = AVAudioUnitMIDIInstrument(audioComponentDescription: desc)
    outUnit.pointee = retainBox(instrument)
    return AU_OK
}

@_cdecl("au_av_midi_instrument_release")
public func au_av_midi_instrument_release(_ ptr: UnsafeMutableRawPointer) {
    releaseBox(ptr, as: AVAudioUnitMIDIInstrument.self)
}

@_cdecl("au_av_midi_instrument_snapshot_json")
public func au_av_midi_instrument_snapshot_json(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    jsonCString(encodeAvAudioUnitMIDIInstrument(borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self)))
}

@_cdecl("au_av_midi_instrument_as_avunit")
public func au_av_midi_instrument_as_avunit(_ ptr: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer {
    retainBox(borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self) as AVAudioUnit)
}

@_cdecl("au_av_midi_instrument_start_note")
public func au_av_midi_instrument_start_note(_ ptr: UnsafeMutableRawPointer, _ note: UInt8, _ velocity: UInt8, _ channel: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).startNote(note, withVelocity: velocity, onChannel: channel)
}

@_cdecl("au_av_midi_instrument_stop_note")
public func au_av_midi_instrument_stop_note(_ ptr: UnsafeMutableRawPointer, _ note: UInt8, _ channel: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).stopNote(note, onChannel: channel)
}

@_cdecl("au_av_midi_instrument_send_controller")
public func au_av_midi_instrument_send_controller(_ ptr: UnsafeMutableRawPointer, _ controller: UInt8, _ value: UInt8, _ channel: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).sendController(controller, withValue: value, onChannel: channel)
}

@_cdecl("au_av_midi_instrument_send_pitch_bend")
public func au_av_midi_instrument_send_pitch_bend(_ ptr: UnsafeMutableRawPointer, _ value: UInt16, _ channel: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).sendPitchBend(value, onChannel: channel)
}

@_cdecl("au_av_midi_instrument_send_pressure")
public func au_av_midi_instrument_send_pressure(_ ptr: UnsafeMutableRawPointer, _ pressure: UInt8, _ channel: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).sendPressure(pressure, onChannel: channel)
}

@_cdecl("au_av_midi_instrument_send_pressure_for_key")
public func au_av_midi_instrument_send_pressure_for_key(_ ptr: UnsafeMutableRawPointer, _ key: UInt8, _ value: UInt8, _ channel: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).sendPressure(forKey: key, withValue: value, onChannel: channel)
}

@_cdecl("au_av_midi_instrument_send_program_change")
public func au_av_midi_instrument_send_program_change(_ ptr: UnsafeMutableRawPointer, _ program: UInt8, _ channel: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).sendProgramChange(program, onChannel: channel)
}

@_cdecl("au_av_midi_instrument_send_program_change_bank")
public func au_av_midi_instrument_send_program_change_bank(_ ptr: UnsafeMutableRawPointer, _ program: UInt8, _ bankMSB: UInt8, _ bankLSB: UInt8, _ channel: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).sendProgramChange(program, bankMSB: bankMSB, bankLSB: bankLSB, onChannel: channel)
}

@_cdecl("au_av_midi_instrument_send_midi_event2")
public func au_av_midi_instrument_send_midi_event2(_ ptr: UnsafeMutableRawPointer, _ status: UInt8, _ data1: UInt8, _ data2: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).sendMIDIEvent(status, data1: data1, data2: data2)
}

@_cdecl("au_av_midi_instrument_send_midi_event1")
public func au_av_midi_instrument_send_midi_event1(_ ptr: UnsafeMutableRawPointer, _ status: UInt8, _ data1: UInt8) {
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).sendMIDIEvent(status, data1: data1)
}

@_cdecl("au_av_midi_instrument_send_sysex")
public func au_av_midi_instrument_send_sysex(_ ptr: UnsafeMutableRawPointer, _ bytes: UnsafePointer<UInt8>?, _ length: Int) {
    let data: Data
    if let bytes {
        data = Data(bytes: bytes, count: length)
    } else {
        data = Data()
    }
    borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).sendMIDISysExEvent(data)
}

@_cdecl("au_av_midi_instrument_send_event_list")
public func au_av_midi_instrument_send_event_list(_ ptr: UnsafeMutableRawPointer, _ list: UnsafeRawPointer?) {
    guard let list else { return }
    list.assumingMemoryBound(to: MIDIEventList.self).withMemoryRebound(to: MIDIEventList.self, capacity: 1) { rebound in
        borrowBox(ptr, as: AVAudioUnitMIDIInstrument.self).send(rebound)
    }
}
