//! Error type for the `audiounit` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuError {
    InvalidArgument(String),
    InstantiateFailed(String),
    TimedOut(String),
    PropertyError(String),
    Unavailable(String),
    Serialization(String),
    Unknown { code: i32, message: String },
}

impl fmt::Display for AuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::InstantiateFailed(message) => {
                write!(f, "AudioUnit instantiation failed: {message}")
            }
            Self::TimedOut(message) => write!(f, "timed out: {message}"),
            Self::PropertyError(message) => {
                write!(f, "AudioUnit property error: {message}")
            }
            Self::Unavailable(message) => write!(f, "unavailable: {message}"),
            Self::Serialization(message) => write!(f, "serialization error: {message}"),
            Self::Unknown { code, message } => {
                write!(f, "audiounit error {code}: {message}")
            }
        }
    }
}

impl std::error::Error for AuError {}

pub(crate) fn from_status(status: i32, msg: *mut core::ffi::c_char) -> AuError {
    use crate::ffi::status;
    let message = if msg.is_null() {
        String::new()
    } else {
        // SAFETY: msg is checked for null above; Swift bridge guarantees it points to a valid,
        // null-terminated C string if not null.
        let s = unsafe { core::ffi::CStr::from_ptr(msg) }
            .to_string_lossy()
            .into_owned();
        // SAFETY: msg is guaranteed to be a valid allocation from Swift bridge; freeing it here
        // is necessary to prevent memory leaks since the bridge allocates the string.
        unsafe { crate::ffi::au_string_free(msg) };
        s
    };
    match status {
        status::INVALID_ARGUMENT => AuError::InvalidArgument(message),
        status::INSTANTIATE_FAILED => AuError::InstantiateFailed(message),
        status::TIMED_OUT => AuError::TimedOut(message),
        status::PROPERTY_ERROR => AuError::PropertyError(message),
        status::UNAVAILABLE => AuError::Unavailable(message),
        code => AuError::Unknown { code, message },
    }
}
