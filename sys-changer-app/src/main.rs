#![allow(unused_imports, non_upper_case_globals, dead_code)]

mod interfaces;
mod simpler_auth;
mod tweaking_config;

use crate::interfaces::get_interfaces;
use crate::simpler_auth::SimpleAuthorization;
use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use system_configuration::network_configuration::SCNetworkSet;
use system_configuration::preferences::SCPreferences;

pub(crate) mod ext {
    use core_foundation::array::CFArray;
    use core_foundation::base::{FromVoid, ItemRef};
    use extend::ext;

    #[ext(pub, name=CFArrayExt)]
    impl<T> CFArray<T> {
        fn into_collect<B: FromIterator<T>>(self) -> B
        where
            T: FromVoid,
            B: FromIterator<T>,
            for<'a> ItemRef<'a, T>: Clone,
        {
            self.into_iter()
                .into_iter()
                .map(|i| i.clone())
                .collect::<B>()
        }
    }
}

#[cfg(target_os = "macos")]
pub fn main() {
    // constants
    let proc_name = CFString::new("sys-changer-app");
    let my_networkset_name = CFString::new("sys-changer-app-networkset");

    // grab network info
    for i in get_interfaces() {
        println!("found interface {i:?}");
    }

    // grab authorization & create preference set with it
    let authorization = SimpleAuthorization::default().unwrap();
    let prefs =
        unsafe { SCPreferences::default_with_authorization(&proc_name, authorization.get_ref()) };

    // clean up previous sets/services that existed
    helper::delete_old_if_exits(&prefs);

    // grab current set and duplicate it
    let current = SCNetworkSet::get_current(&prefs).unwrap();
    let new = helper::shallow_clone_network_set(&prefs, &current, my_networkset_name);
    println!(
        "Created copy of current: {}]--[{}",
        new.name().unwrap(),
        new.id().unwrap()
    );

    // commit and apply new changes
    helper::save_prefs(&prefs);
}

#[cfg(not(target_os = "macos"))]
pub fn main() {
    panic!("Non-macOS systems are not supported");
}

#[cfg(target_os = "macos")]
pub(crate) mod helper {
    use core_foundation::base::{CFType, TCFType};
    use core_foundation::boolean::CFBoolean;
    use core_foundation::dictionary::{CFDictionary, CFMutableDictionary};
    use core_foundation::error::CFError;
    use core_foundation::string::CFString;
    use std::io::BufRead;
    use system_configuration::network_configuration::{SCNetworkService, SCNetworkSet};
    use system_configuration::preferences::SCPreferences;
    use system_configuration_sys::preferences::{
        SCPreferencesApplyChanges, SCPreferencesCommitChanges,
    };
    use system_configuration_sys::preferences_path::{
        SCPreferencesPathCreateUniqueChild, SCPreferencesPathGetValue, SCPreferencesPathSetValue,
    };
    use system_configuration_sys::schema_definitions::{
        kSCPrefNetworkServices, kSCPrefSets, kSCPropUserDefinedName,
    };
    use system_configuration_sys::system_configuration::SCCopyLastError;

    /// Creates a shallow copy of the provided network set, using the `SCPreferencesPath*` APIs.
    ///
    /// The resulting network set will have the exact same __everything__, except for a different
    /// user-defined name compared to the old network set.
    pub fn shallow_clone_network_set(
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
        let new_set_id: CFString = (*new_set_path
            .to_string()
            .as_str()
            .split("/")
            .collect::<Vec<_>>()
            .last()
            .unwrap())
        .into();
        let new_set = SCNetworkSet::find_set(prefs, new_set_id).unwrap();
        new_set
    }

    /// Returns the dictionary associated with the specified path, or nothing if the path does not exist.
    pub fn get_path_dictionary(
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
    pub fn marked_as_owned(dict: &mut CFMutableDictionary<CFString, CFType>) {
        // constants
        let owned_key = CFString::new("ThisNetworkSetWasCreatedByExo");
        let owned_value = CFBoolean::true_value();

        dict.set(owned_key, owned_value.into_CFType());
    }

    /// Delete old managed items if they still exits
    pub fn delete_old_if_exits(prefs: &SCPreferences) {
        // constants
        let sets_key = unsafe { CFString::wrap_under_get_rule(kSCPrefSets) };
        let services_key = unsafe { CFString::wrap_under_get_rule(kSCPrefNetworkServices) };
        let owned_key = CFString::new("ThisNetworkSetWasCreatedByExo");

        // closures
        let set_path =
            |set_id: &CFString| -> CFString { (&*format!("/{sets_key}/{set_id}")).into() };
        let set_values = |set: &SCNetworkSet| {
            let id = set.id().unwrap();
            get_path_dictionary(prefs, &set_path(&id)).unwrap()
        };
        let service_path = |service_id: &CFString| -> CFString {
            (&*format!("/{services_key}/{service_id}")).into()
        };
        let service_values = |service: &SCNetworkService| {
            let id = service.id().unwrap();
            get_path_dictionary(prefs, &service_path(&id)).unwrap()
        };

        // delete any owned sets
        let sets = SCNetworkSet::get_sets(prefs);
        for set in sets.into_iter() {
            let set = set.clone();
            let values = set_values(&set);

            // encountered ownership marker, remove network set
            if values.contains_key(&owned_key) {
                assert!(set.remove());
            }
        }

        // delete any owned services NOT in the current preference set
        let current_services = SCNetworkSet::get_current(prefs)
            .unwrap()
            .services()
            .into_iter()
            .map(|s| s.clone())
            .collect::<Vec<_>>();
        let services = SCNetworkService::get_services(prefs);
        for service in services.into_iter() {
            let service = service.clone();
            if current_services.contains(&service) {
                continue;
            }
            let values = service_values(&service);

            // encountered ownership marker, remove network service
            if values.contains_key(&owned_key) {
                assert!(service.remove());
            }
        }
    }

    pub fn save_prefs(prefs: &SCPreferences) {
        unsafe {
            panic_err(SCPreferencesCommitChanges(prefs.as_concrete_TypeRef()) != 0);
            panic_err(SCPreferencesApplyChanges(prefs.as_concrete_TypeRef()) != 0);
        }
    }

    pub fn panic_err(success: bool) {
        if success {
            return;
        }
        let e = unsafe { CFError::wrap_under_create_rule(SCCopyLastError()) };
        panic!("error: {}", e);
    }
}
