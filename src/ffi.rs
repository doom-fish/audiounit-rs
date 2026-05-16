//! Raw FFI declarations matching the Swift bridge.

#![allow(missing_docs, non_camel_case_types)]

use core::ffi::{c_char, c_void};

extern "C" {
    // --- String helpers ---
    pub fn au_string_free(ptr: *mut c_char);

    // --- AudioComponent enumeration (legacy C API) ---
    /// All five desc fields passed individually (Swift @_cdecl cannot accept structs).
    pub fn au_component_count(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
    ) -> u32;
    pub fn au_component_list(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
        out: *mut *mut c_void,
        max_count: usize,
    ) -> usize;
    pub fn au_component_copy_name(comp: *mut c_void) -> *mut c_char;
    pub fn au_component_get_description(
        comp: *mut c_void,
        out_type: *mut u32,
        out_subtype: *mut u32,
        out_manufacturer: *mut u32,
        out_flags: *mut u32,
        out_flags_mask: *mut u32,
    ) -> i32;
    pub fn au_component_get_version(comp: *mut c_void) -> u32;

    // --- AVAudioUnitComponentManager ---
    pub fn au_avc_manager_components_matching(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
        out_count: *mut usize,
    ) -> *mut *mut c_void;
    pub fn au_avc_component_array_free(buf: *mut *mut c_void, count: usize);

    // --- AVAudioUnitComponent properties ---
    pub fn au_avc_component_name(ptr: *mut c_void) -> *mut c_char;
    pub fn au_avc_component_type_name(ptr: *mut c_void) -> *mut c_char;
    pub fn au_avc_component_manufacturer_name(ptr: *mut c_void) -> *mut c_char;
    pub fn au_avc_component_version(ptr: *mut c_void) -> u32;
    pub fn au_avc_component_version_string(ptr: *mut c_void) -> *mut c_char;
    pub fn au_avc_component_has_custom_view(ptr: *mut c_void) -> bool;
    pub fn au_avc_component_sandbox_safe(ptr: *mut c_void) -> bool;
    pub fn au_avc_component_audio_component_description(
        ptr: *mut c_void,
        out_type: *mut u32,
        out_subtype: *mut u32,
        out_manufacturer: *mut u32,
        out_flags: *mut u32,
        out_flags_mask: *mut u32,
    );
    pub fn au_avc_component_release(ptr: *mut c_void);

    // --- AVAudioUnit instantiation ---
    pub fn au_instantiate_sync(
        component_type: u32,
        component_subtype: u32,
        component_manufacturer: u32,
        component_flags: u32,
        component_flags_mask: u32,
        options: u32,
        out_unit: *mut *mut c_void,
        out_error_msg: *mut *mut c_char,
    ) -> i32;
    pub fn au_avunit_release(ptr: *mut c_void);
    pub fn au_avunit_audio_unit(ptr: *mut c_void) -> *mut c_void;
    pub fn au_avunit_auaudiounit(ptr: *mut c_void) -> *mut c_void;
    pub fn au_auaudiounit_release(ptr: *mut c_void);

    // --- AUParameterTree ---
    pub fn au_auaudiounit_parameter_tree(ptr: *mut c_void) -> *mut c_void;
    pub fn au_parameter_tree_release(ptr: *mut c_void);
    pub fn au_parameter_tree_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_tree_parameter_with_address(
        tree: *mut c_void,
        address: u64,
    ) -> *mut c_void;

    // --- AUParameter ---
    pub fn au_parameter_release(ptr: *mut c_void);
    pub fn au_parameter_get_value(ptr: *mut c_void) -> f32;
    pub fn au_parameter_set_value(ptr: *mut c_void, value: f32);
    pub fn au_parameter_identifier(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_display_name(ptr: *mut c_void) -> *mut c_char;
    pub fn au_parameter_address(ptr: *mut c_void) -> u64;
    pub fn au_parameter_min_value(ptr: *mut c_void) -> f32;
    pub fn au_parameter_max_value(ptr: *mut c_void) -> f32;
    pub fn au_parameter_unit(ptr: *mut c_void) -> u32;
    pub fn au_parameter_string_from_value(ptr: *mut c_void, value: f32) -> *mut c_char;
}

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const INSTANTIATE_FAILED: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
    pub const PROPERTY_ERROR: i32 = -4;
    pub const UNKNOWN: i32 = -99;
}
