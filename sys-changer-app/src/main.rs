use core_foundation::base::TCFType;
use core_foundation::error::CFError;
use core_foundation::string::CFString;
use security_framework::base::Error;
use security_framework_sys::authorization::{
    errAuthorizationSuccess, kAuthorizationFlagDefaults, AuthorizationCreate, AuthorizationRef,
};
use std::mem::MaybeUninit;
use std::ptr;
use system_configuration::preferences::SCPreferences;
use system_configuration_sys::preferences::{
    SCPreferencesCreateWithAuthorization, SCPreferencesGetValue,
};
use system_configuration_sys::system_configuration::SCCopyLastError;

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

    unsafe {
        let list = SCPreferencesGetValue(
            preferences.as_concrete_TypeRef(),
            CFString::new("does_not_exist").as_concrete_TypeRef(),
        );
        if list.is_null() {
            let error = CFError::wrap_under_create_rule(SCCopyLastError());
            println!("SCPreferencesGetValue returned null w/ error: {:?}", error);
        }
    };
}

#[cfg(not(target_os = "macos"))]
pub fn main() {
    panic!("Non-macOS systems are not supported");
}

#[cfg(target_os = "macos")]
mod helper {
    use system_configuration::preferences::SCPreferences;

    /// Creates a shallow copy of the provided network set, using the `SCPreferencesPath*` APIs.
    ///
    /// The resulting network set will have the exact same __everything__, except for a different
    /// user-defined name compared to the old network set.
    fn shallow_clone_network_set(prefs: &SCPreferences) {}
}
