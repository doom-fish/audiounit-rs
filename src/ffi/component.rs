use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_string_free(ptr: *mut c_char);
    pub fn au_component_count(component_type: u32, component_subtype: u32, component_manufacturer: u32, component_flags: u32, component_flags_mask: u32) -> u32;
    pub fn au_component_list(component_type: u32, component_subtype: u32, component_manufacturer: u32, component_flags: u32, component_flags_mask: u32, out: *mut *mut c_void, max_count: usize) -> usize;
    pub fn au_component_copy_name(comp: *mut c_void) -> *mut c_char;
    pub fn au_component_get_description(comp: *mut c_void, out_type: *mut u32, out_subtype: *mut u32, out_manufacturer: *mut u32, out_flags: *mut u32, out_flags_mask: *mut u32) -> i32;
    pub fn au_component_get_version(comp: *mut c_void) -> u32;
    pub fn au_avc_manager_components_matching(component_type: u32, component_subtype: u32, component_manufacturer: u32, component_flags: u32, component_flags_mask: u32, out_count: *mut usize) -> *mut *mut c_void;
    pub fn au_avc_component_array_free(buf: *mut *mut c_void, count: usize);
    pub fn au_avc_component_name(ptr: *mut c_void) -> *mut c_char;
    pub fn au_avc_component_type_name(ptr: *mut c_void) -> *mut c_char;
    pub fn au_avc_component_manufacturer_name(ptr: *mut c_void) -> *mut c_char;
    pub fn au_avc_component_version(ptr: *mut c_void) -> u32;
    pub fn au_avc_component_version_string(ptr: *mut c_void) -> *mut c_char;
    pub fn au_avc_component_has_custom_view(ptr: *mut c_void) -> bool;
    pub fn au_avc_component_sandbox_safe(ptr: *mut c_void) -> bool;
    pub fn au_avc_component_audio_component_description(ptr: *mut c_void, out_type: *mut u32, out_subtype: *mut u32, out_manufacturer: *mut u32, out_flags: *mut u32, out_flags_mask: *mut u32);
    pub fn au_avc_component_release(ptr: *mut c_void);
}
