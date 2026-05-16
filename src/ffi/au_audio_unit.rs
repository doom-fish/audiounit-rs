use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn au_auaudiounit_instantiate_sync(component_type: u32, component_subtype: u32, component_manufacturer: u32, component_flags: u32, component_flags_mask: u32, options: u32, out_unit: *mut *mut c_void, out_error_msg: *mut *mut c_char) -> i32;
    pub fn au_auaudiounit_release(ptr: *mut c_void);
    pub fn au_auaudiounit_snapshot_json(ptr: *mut c_void) -> *mut c_char;
    pub fn au_auaudiounit_allocate_render_resources(ptr: *mut c_void, out_error_msg: *mut *mut c_char) -> i32;
    pub fn au_auaudiounit_deallocate_render_resources(ptr: *mut c_void);
    pub fn au_auaudiounit_reset(ptr: *mut c_void);
    pub fn au_auaudiounit_input_busses(ptr: *mut c_void) -> *mut c_void;
    pub fn au_auaudiounit_output_busses(ptr: *mut c_void) -> *mut c_void;
    pub fn au_auaudiounit_parameter_tree(ptr: *mut c_void) -> *mut c_void;
    pub fn au_auaudiounit_parameters_for_overview_json(ptr: *mut c_void, count: usize) -> *mut c_char;
    pub fn au_auaudiounit_set_maximum_frames_to_render(ptr: *mut c_void, value: u32);
    pub fn au_auaudiounit_set_render_quality(ptr: *mut c_void, value: isize);
    pub fn au_auaudiounit_set_should_bypass_effect(ptr: *mut c_void, value: bool);
    pub fn au_auaudiounit_set_rendering_offline(ptr: *mut c_void, value: bool);
    pub fn au_auaudiounit_set_context_name(ptr: *mut c_void, value: *const c_char);
    pub fn au_auaudiounit_set_current_preset(ptr: *mut c_void, number: isize, name: *const c_char);
    pub fn au_auaudiounit_set_channel_map(ptr: *mut c_void, values: *const i32, count: usize);
}
