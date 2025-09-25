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
mod base;
mod status_temp;
mod base2;

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
    
    pub const fn u32_into_u8_unchecked(value: u32) -> u8 {
        if value <= u8::MAX as u32 {
            value as u8
        } else {
            panic!()
        }
    }
}
