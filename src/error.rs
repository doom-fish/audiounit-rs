//! Error type for the `audiounit` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuError {
    InvalidArgument(String),
    InstantiateFailed(String),
    TimedOut(String),
    PropertyError(String),
    Unknown { code: i32, message: String },
}

impl fmt::Display for AuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(m) => write!(f, "invalid argument: {m}"),
            Self::InstantiateFailed(m) => write!(f, "AudioUnit instantiation failed: {m}"),
            Self::TimedOut(m) => write!(f, "timed out: {m}"),
            Self::PropertyError(m) => write!(f, "AudioUnit property error: {m}"),
            Self::Unknown { code, message } => write!(f, "audiounit error {code}: {message}"),
        }
    }
}

impl std::error::Error for AuError {}

pub(crate) fn from_status(status: i32, msg: *mut core::ffi::c_char) -> AuError {
    use crate::ffi::status;
    let message = if msg.is_null() {
        String::new()
    } else {
        let s = unsafe { core::ffi::CStr::from_ptr(msg) }
            .to_string_lossy()
            .into_owned();
        unsafe { crate::ffi::au_string_free(msg) };
        s
    };
    match status {
        status::INVALID_ARGUMENT => AuError::InvalidArgument(message),
        status::INSTANTIATE_FAILED => AuError::InstantiateFailed(message),
        status::TIMED_OUT => AuError::TimedOut(message),
        status::PROPERTY_ERROR => AuError::PropertyError(message),
        code => AuError::Unknown { code, message },
    }
}
