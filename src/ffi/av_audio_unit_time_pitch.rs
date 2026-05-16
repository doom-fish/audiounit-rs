use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_av_time_pitch_create(
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_av_time_pitch_release(ptr: *mut c_void);
    pub fn au_av_time_pitch_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_av_time_pitch_get_rate(ptr: *mut c_void) -> f32;
    pub fn au_av_time_pitch_set_rate(ptr: *mut c_void, rate: f32);
    pub fn au_av_time_pitch_get_pitch(ptr: *mut c_void) -> f32;
    pub fn au_av_time_pitch_set_pitch(ptr: *mut c_void, pitch: f32);
    pub fn au_av_time_pitch_get_overlap(ptr: *mut c_void) -> f32;
    pub fn au_av_time_pitch_set_overlap(ptr: *mut c_void, overlap: f32);
    pub fn au_av_time_pitch_as_time_effect(ptr: *mut c_void) -> *mut c_void;
}
