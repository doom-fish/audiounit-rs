use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_av_delay_create(out_unit: *mut *mut c_void, out_error_msg: *mut *mut c_char) -> i32;
    pub fn au_av_delay_release(ptr: *mut c_void);
    pub fn au_av_delay_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_av_delay_get_delay_time(ptr: *mut c_void) -> f64;
    pub fn au_av_delay_set_delay_time(ptr: *mut c_void, delay_time: f64);
    pub fn au_av_delay_get_feedback(ptr: *mut c_void) -> f32;
    pub fn au_av_delay_set_feedback(ptr: *mut c_void, feedback: f32);
    pub fn au_av_delay_get_low_pass_cutoff(ptr: *mut c_void) -> f32;
    pub fn au_av_delay_set_low_pass_cutoff(ptr: *mut c_void, low_pass_cutoff: f32);
    pub fn au_av_delay_get_wet_dry_mix(ptr: *mut c_void) -> f32;
    pub fn au_av_delay_set_wet_dry_mix(ptr: *mut c_void, wet_dry_mix: f32);
    pub fn au_av_delay_as_effect(ptr: *mut c_void) -> *mut c_void;
}
