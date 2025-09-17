use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use security_framework::base::Error;
use security_framework_sys::authorization::{
    errAuthorizationSuccess, kAuthorizationFlagDefaults, AuthorizationCreate, AuthorizationRef,
};
use std::mem::MaybeUninit;
use std::ptr;
use system_configuration::preferences::SCPreferences;
use system_configuration_sys::preferences::SCPreferencesCreateWithAuthorization;

#[cfg(target_os = "macos")]
pub fn main() {
    // constants
    let proc_name = CFString::new("sys-changer-app");
    let my_networkset_name = CFString::new("sys-changer-app-networkset");

    // grab authorization & create preference set with it
    let mut handle = MaybeUninit::<AuthorizationRef>::uninit();
    let status = unsafe {
        AuthorizationCreate(
            ptr::null(),
            ptr::null(),
            kAuthorizationFlagDefaults,
            handle.as_mut_ptr(),
        )
    };
    if status != errAuthorizationSuccess {
        Result::<(), Error>::Err(Error::from_code(status)).unwrap();
    }
    let authorization = unsafe { handle.assume_init() };
    let preferences = unsafe {
        SCPreferences::wrap_under_create_rule(SCPreferencesCreateWithAuthorization(
            ptr::null(),
            proc_name.as_concrete_TypeRef(),
            ptr::null(),
            authorization,
        ))
    };
}

#[cfg(not(target_os = "macos"))]
pub fn main() {
    panic!("Non-macOS systems are not supported");
}
