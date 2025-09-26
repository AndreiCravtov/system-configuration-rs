// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # SystemConfiguration bindings
//!
//! This crate is a high level binding to the Apple [SystemConfiguration] framework. For low level
//! FFI bindings, check out the [`system-configuration-sys`] crate.
//!
//! This crate only implements a small part of the [SystemConfiguration] framework so far. If you
//! need a yet unimplemented part, feel free to submit a pull request!
//!
//! [SystemConfiguration]: https://developer.apple.com/documentation/systemconfiguration?language=objc
//! [`system-configuration-sys`]: https://crates.io/crates/system-configuration-sys

/// CoreFoundation wrappers
#[macro_use]
pub extern crate core_foundation;
/// Low-level SystemConfiguration bindings
pub extern crate system_configuration_sys as sys;
extern crate core;

pub mod dynamic_store;
pub mod network_configuration;
pub mod network_reachability;
pub mod preferences;
pub mod status;

// this corresponds to the base bindings in SystemConfiguration.h
pub mod base {
    use std::ffi::CStr;
    use core_foundation::{base::TCFType, error::CFError};
    use sys::core_foundation_sys::base::OSStatus;
    use sys::system_configuration::{SCCopyLastError, SCError, SCErrorString};

    /// Returns the most recent status or error code generated as the result of calling a function
    /// defined by the System Configuration framework. The code is represented by a Core Foundation
    /// [`CFError`] opaque type.
    ///
    /// See [`SCCopyLastError`] for details.
    ///
    /// [`SCCopyLastError`]: https://developer.apple.com/documentation/systemconfiguration/sccopylasterror()?language=objc
    pub fn get_last_error() -> CFError {
        unsafe { CFError::wrap_under_create_rule(SCCopyLastError()) }
    }

    /// Returns most recent status or error code generated as the result of calling a function
    /// defined by the System Configuration framework. See [Status and Error Codes] for descriptions
    /// of these codes.
    ///
    /// See [`SCError`] for details.
    ///
    /// [Status and Error Codes]: https://developer.apple.com/documentation/systemconfiguration/1518026-status-and-error-codes?language=objc
    /// [`SCError`]: https://developer.apple.com/documentation/systemconfiguration/scerror()?language=objc
    pub fn get_last_error_code() -> OSStatus {
        unsafe { SCError() }
    }

    /// Returns a string describing the specified status code or error code.
    ///
    /// See [`SCErrorString`] for details.
    ///
    /// [`SCErrorString`]: https://developer.apple.com/documentation/systemconfiguration/scerrorstring(_:)?language=objc
    pub fn get_error_string(status: OSStatus) -> String {
        let cstr = unsafe {
            let cstr_ptr = SCErrorString(status);
            assert!(!cstr_ptr.is_null(), "pointer to error string is never null");
            CStr::from_ptr(cstr_ptr)
        };
        cstr.to_str().expect("error string is always valid UTF-8").to_string()
    }
}

pub(crate) mod helpers {
    use core_foundation::array::CFArray;
    use core_foundation::base::TCFType;

    pub fn create_empty_array<T>() -> CFArray<T> {
        use std::ptr::null;
        unsafe {
            CFArray::wrap_under_create_rule(core_foundation::array::CFArrayCreate(
                null() as *const _,
                null() as *const _,
                0,
                null() as *const _,
            ))
        }
    }

    pub const fn u32_into_u16_unchecked(value: u32) -> u16 {
        if value <= u16::MAX as u32 {
            value as u16
        } else {
            panic!()
        }
    }
}
