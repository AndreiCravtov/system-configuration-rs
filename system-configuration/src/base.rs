use core::fmt;
use std::ffi::CStr;
use std::num::NonZeroI32;
use std::{error, result};
use core_foundation::base::OSStatus;
use sys::system_configuration::SCErrorString;

/// A `Result` type commonly returned by functions.
pub type Result<T, E = Error> = result::Result<T, E>;

/// A System Configuration framework error.
#[derive(Copy, Clone)]
pub struct Error(NonZeroI32);

impl fmt::Debug for Error {
    #[cold]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = fmt.debug_struct("Error");
        builder.field("code", &self.0)
            .field("message", &self.message())
            .finish()
    }
}

impl Error {
    /// Creates a new [`Error`] from a status code. The code must not be zero.
    #[inline]
    #[must_use]
    pub fn from_code(code: OSStatus) -> Self {
        Self(NonZeroI32::new(code).unwrap_or_else(|| NonZeroI32::new(1).unwrap()))
    }

    /// Returns a string describing the current error.
    #[inline(always)]
    #[must_use]
    pub fn message(self) -> String {
        self.inner_message()
    }

    #[cold]
    fn inner_message(self) -> String {
        let cstr = unsafe {
            let cstr_ptr = SCErrorString(self.code());
            assert!(!cstr_ptr.is_null(), "error string must never be null");

            CStr::from_ptr(cstr_ptr)
        };
        cstr.to_str().expect("should always be valid UTF-8").to_string()
    }

    /// Returns the code of the current error.
    #[inline(always)]
    #[must_use]
    pub fn code(self) -> OSStatus {
        self.0.get() as _
    }
}

impl From<OSStatus> for Error {
    #[inline(always)]
    #[must_use]
    fn from(code: OSStatus) -> Self {
        Self::from_code(code)
    }
}

impl fmt::Display for Error {
    #[cold]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.message())
    }
}

impl error::Error for Error {}