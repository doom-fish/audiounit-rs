//! Declarative macro for owned-handle boilerplate.
//!
//! Most wrapper types in this crate hold a single `*mut c_void` pointer to a
//! retained Swift / Objective-C object and hand-roll identical `unsafe impl
//! Send` and `Drop` (release) implementations. `handle_type!` consolidates that
//! boilerplate into a single audited place.
//!
//! The generated impls preserve the exact behavior of the previous
//! hand-written versions:
//! - `unsafe impl Send` asserts the handle is safe to move across threads
//!   (the Swift objects are internally synchronized).
//! - `Drop` calls the supplied `release` FFI fn unconditionally, matching the
//!   original `unsafe { ffi::..._release(self.ptr) }` bodies (no null guard,
//!   since the Swift bridge tolerates a null pointer).
//!
//! Types whose `Drop` carries extra logic, or that need additional marker
//! traits beyond the variants below, are intentionally left hand-written.

/// Generate `unsafe impl Send` and `Drop` impls for an owned pointer handle.
///
/// Variants:
/// - `Send` + `Drop`:
///   `handle_type!(Ty, field = ptr, release = path::release);`
/// - `Send` + `Sync` + `Drop`:
///   `handle_type!(Ty, field = ptr, release = path::release, sync);`
macro_rules! handle_type {
    ($ty:ty, field = $field:ident, release = $release:path $(,)?) => {
        unsafe impl Send for $ty {}

        impl Drop for $ty {
            fn drop(&mut self) {
                unsafe { $release(self.$field) };
            }
        }
    };

    ($ty:ty, field = $field:ident, release = $release:path, sync $(,)?) => {
        unsafe impl Sync for $ty {}

        $crate::handle::handle_type!($ty, field = $field, release = $release);
    };
}

pub(crate) use handle_type;
