use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_av_generator_create(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_av_generator_release(ptr: *mut c_void);
    pub fn au_av_generator_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_av_generator_get_bypass(ptr: *mut c_void) -> bool;
    pub fn au_av_generator_set_bypass(ptr: *mut c_void, bypass: bool);
    pub fn au_av_generator_as_avunit(ptr: *mut c_void) -> *mut c_void;
}
