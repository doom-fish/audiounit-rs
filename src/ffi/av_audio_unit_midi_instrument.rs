use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_av_midi_instrument_create(component_type: u32, component_subtype: u32, component_manufacturer: u32, component_flags: u32, component_flags_mask: u32, out_unit: *mut *mut c_void, out_error_msg: *mut *mut c_char) -> i32;
    pub fn au_av_midi_instrument_release(ptr: *mut c_void);
    pub fn au_av_midi_instrument_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_av_midi_instrument_as_avunit(ptr: *mut c_void) -> *mut c_void;
    pub fn au_av_midi_instrument_start_note(ptr: *mut c_void, note: u8, velocity: u8, channel: u8);
    pub fn au_av_midi_instrument_stop_note(ptr: *mut c_void, note: u8, channel: u8);
    pub fn au_av_midi_instrument_send_controller(ptr: *mut c_void, controller: u8, value: u8, channel: u8);
    pub fn au_av_midi_instrument_send_pitch_bend(ptr: *mut c_void, value: u16, channel: u8);
    pub fn au_av_midi_instrument_send_pressure(ptr: *mut c_void, pressure: u8, channel: u8);
    pub fn au_av_midi_instrument_send_pressure_for_key(ptr: *mut c_void, key: u8, value: u8, channel: u8);
    pub fn au_av_midi_instrument_send_program_change(ptr: *mut c_void, program: u8, channel: u8);
    pub fn au_av_midi_instrument_send_program_change_bank(ptr: *mut c_void, program: u8, bank_msb: u8, bank_lsb: u8, channel: u8);
    pub fn au_av_midi_instrument_send_midi_event2(ptr: *mut c_void, status: u8, data1: u8, data2: u8);
    pub fn au_av_midi_instrument_send_midi_event1(ptr: *mut c_void, status: u8, data1: u8);
    pub fn au_av_midi_instrument_send_sysex(ptr: *mut c_void, bytes: *const u8, length: usize);
    pub fn au_av_midi_instrument_send_event_list(ptr: *mut c_void, list: *const c_void);
}
