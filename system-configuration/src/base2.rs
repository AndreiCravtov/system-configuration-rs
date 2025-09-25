use std::ffi::CStr;
use core_foundation::{base::TCFType, error::CFError};
use sys::core_foundation_sys::base::OSStatus;
use sys::system_configuration::{SCCopyLastError, SCError, SCErrorString};

/// Returns the most recent status or error code generated as the result of calling a function
/// defined by the System Configuration framework. The code is represented by a Core Foundation
/// [`CFError`] opaque type.
///
/// TODO: rest of docs
///
/// See [`SCCopyLastError`] for details.
///
/// [`SCCopyLastError`]: https://developer.apple.com/documentation/systemconfiguration/sccopylasterror()?language=objc
pub fn last_error() -> CFError {
    unsafe { CFError::wrap_under_create_rule(SCCopyLastError()) }
}

/// Returns most recent status or error code generated as the result of calling a function defined
/// by the System Configuration framework. See [Status and Error Codes] for descriptions of these
/// codes.
///
/// TODO: rest of docs
///
/// See [`SCError`] for details.
///
/// [Status and Error Codes]: https://developer.apple.com/documentation/systemconfiguration/1518026-status-and-error-codes?language=objc
/// [`SCError`]: https://developer.apple.com/documentation/systemconfiguration/scerror()?language=objc
pub fn last_error_code() -> OSStatus {
    unsafe { SCError() }
}

/// Returns a string describing the specified status code or error code.
///
/// TODO: rest of docs
pub fn error_string(status: OSStatus) -> String {
    let cstr = unsafe {
        let cstr_ptr = SCErrorString(status);
        assert!(!cstr_ptr.is_null(), "pointer to error string is never null");
        CStr::from_ptr(cstr_ptr)
    };
    cstr.to_str().expect("error string is always valid UTF-8").to_string()
}