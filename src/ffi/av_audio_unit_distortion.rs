use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_av_distortion_create(
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_av_distortion_release(ptr: *mut c_void);
    pub fn au_av_distortion_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_av_distortion_load_factory_preset(ptr: *mut c_void, preset: i64);
    pub fn au_av_distortion_get_pre_gain(ptr: *mut c_void) -> f32;
    pub fn au_av_distortion_set_pre_gain(ptr: *mut c_void, pre_gain: f32);
    pub fn au_av_distortion_get_wet_dry_mix(ptr: *mut c_void) -> f32;
    pub fn au_av_distortion_set_wet_dry_mix(ptr: *mut c_void, wet_dry_mix: f32);
    pub fn au_av_distortion_as_effect(ptr: *mut c_void) -> *mut c_void;
}
