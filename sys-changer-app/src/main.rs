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
use system_configuration_sys::network_configuration::SCNetworkSetGetSetID;
use system_configuration_sys::preferences::SCPreferencesCreateWithAuthorization;
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
        let list = SCNetworkSetGetSetID(ptr::null());
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
    use core_foundation::base::{CFType, TCFType};
    use core_foundation::boolean::CFBoolean;
    use core_foundation::dictionary::{CFDictionary, CFMutableDictionary};
    use core_foundation::error::CFError;
    use core_foundation::string::CFString;
    use std::io::BufRead;
    use system_configuration::network_configuration::SCNetworkSet;
    use system_configuration::preferences::SCPreferences;
    use system_configuration_sys::preferences_path::{
        SCPreferencesPathCreateUniqueChild, SCPreferencesPathGetValue, SCPreferencesPathSetValue,
    };
    use system_configuration_sys::schema_definitions::{kSCPrefSets, kSCPropUserDefinedName};
    use system_configuration_sys::system_configuration::SCCopyLastError;

    /// Creates a shallow copy of the provided network set, using the `SCPreferencesPath*` APIs.
    ///
    /// The resulting network set will have the exact same __everything__, except for a different
    /// user-defined name compared to the old network set.
    fn shallow_clone_network_set(
        prefs: &SCPreferences,
        old_set: &SCNetworkSet,
        new_set_name: CFString,
    ) -> SCNetworkSet {
        // constants
        let sets_key = unsafe { CFString::wrap_under_get_rule(kSCPrefSets) };
        let user_defined_name_key =
            unsafe { CFString::wrap_under_get_rule(kSCPropUserDefinedName) };

        // grab info from old set
        let old_set_id = old_set.id().unwrap();
        let old_set_path: CFString = (&*format!("/{}/{}", sets_key, old_set_id)).into();
        let old_set_values = get_path_dictionary(prefs, &old_set_path).unwrap();

        // create new values & alter name to match new one -> mark it as owned
        let mut new_set_values = CFMutableDictionary::<CFString, CFType>::from(&old_set_values);
        new_set_values.set(user_defined_name_key, new_set_name.into_CFType());
        marked_as_owned(&mut new_set_values);

        // create unique child path in `/Sets` prefix & associate it w/ the values dictionary
        let sets_path: CFString = (&*format!("/{}", sets_key)).into();
        let new_set_path = unsafe {
            CFString::wrap_under_create_rule(SCPreferencesPathCreateUniqueChild(
                prefs.as_concrete_TypeRef(),
                sets_path.as_concrete_TypeRef(),
            ))
        };
        unsafe {
            if SCPreferencesPathSetValue(
                prefs.as_concrete_TypeRef(),
                new_set_path.as_concrete_TypeRef(),
                new_set_values.as_concrete_TypeRef(),
            ) == 0
            {
                panic!(
                    "Encountered error: {}",
                    CFError::wrap_under_create_rule(SCCopyLastError())
                );
            }
        };

        // extract new set ID from path to be able to fetch new network set
        let new_set_id: CFString = new_set_path
            .to_string()
            .as_str()
            .split("/")
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .into();
        let new_set = prefs.find_network_set(new_set_id).unwrap();
        new_set
    }

    /// Returns the dictionary associated with the specified path, or nothing if the path does not exist.
    fn get_path_dictionary(
        prefs: &SCPreferences,
        path: &CFString,
    ) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let dictionary_ref =
                SCPreferencesPathGetValue(prefs.as_concrete_TypeRef(), path.as_concrete_TypeRef());
            if !dictionary_ref.is_null() {
                Some(CFDictionary::wrap_under_get_rule(dictionary_ref))
            } else {
                None
            }
        }
    }

    /// Insert `ThisNetworkSetWasCreatedByExo: true` flag to indicate this is a manged network set.
    fn marked_as_owned(dict: &mut CFMutableDictionary<CFString, CFType>) {
        let owned_key = CFString::new("ThisNetworkSetWasCreatedByExo");
        let owned_value = CFBoolean::true_value();

        dict.set(owned_key, owned_value.into_CFType());
    }
}
