use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_voice_io_create(
        options: u32,
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_voice_io_release(ptr: *mut c_void);
    pub fn au_voice_io_as_avunit(ptr: *mut c_void) -> *mut c_void;
    pub fn au_voice_io_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_voice_io_get_bypass_voice_processing(ptr: *mut c_void) -> bool;
    pub fn au_voice_io_set_bypass_voice_processing(ptr: *mut c_void, value: bool) -> i32;
    pub fn au_voice_io_get_enable_agc(ptr: *mut c_void) -> bool;
    pub fn au_voice_io_set_enable_agc(ptr: *mut c_void, value: bool) -> i32;
    pub fn au_voice_io_get_mute_output(ptr: *mut c_void) -> bool;
    pub fn au_voice_io_set_mute_output(ptr: *mut c_void, value: bool) -> i32;
    pub fn au_voice_io_get_other_audio_ducking_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_voice_io_set_other_audio_ducking(
        ptr: *mut c_void,
        enable_advanced: bool,
        ducking_level: u32,
    ) -> i32;
}
