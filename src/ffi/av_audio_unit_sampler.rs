use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_av_sampler_create(out_unit: *mut *mut c_void, out_error_msg: *mut *mut c_char)
        -> i32;
    pub fn au_av_sampler_release(ptr: *mut c_void);
    pub fn au_av_sampler_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_av_sampler_get_stereo_pan(ptr: *mut c_void) -> f32;
    pub fn au_av_sampler_set_stereo_pan(ptr: *mut c_void, stereo_pan: f32);
    pub fn au_av_sampler_get_overall_gain(ptr: *mut c_void) -> f32;
    pub fn au_av_sampler_set_overall_gain(ptr: *mut c_void, overall_gain: f32);
    pub fn au_av_sampler_get_global_tuning(ptr: *mut c_void) -> f32;
    pub fn au_av_sampler_set_global_tuning(ptr: *mut c_void, global_tuning: f32);
    pub fn au_av_sampler_load_sound_bank_instrument(
        ptr: *mut c_void,
        path: *const c_char,
        program: u8,
        bank_msb: u8,
        bank_lsb: u8,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_av_sampler_load_instrument(
        ptr: *mut c_void,
        path: *const c_char,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_av_sampler_as_midi_instrument(ptr: *mut c_void) -> *mut c_void;
}
