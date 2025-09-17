use core_foundation::base::{CFType, FromVoid, TCFType};
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;
use system_configuration::preferences::SCPreferences;
use system_configuration_sys::preferences_path::SCPreferencesPathGetValue;
use system_configuration_sys::schema_definitions::kSCPrefSets;
// This example will read the persistent store and print (to stdout) all the names of any network sets.
// This is done with the `preferences_path` API specifically, it is what is being tested for.

fn main() {
    // constants
    let sets_key = unsafe { CFString::wrap_under_get_rule(kSCPrefSets) };

    // grab IDs
    let prefs = SCPreferences::default(&"my-network-set-test".into());

    // create path that points to stores dictionary
    let sets_path: CFString = (&*format!("/{sets_key}")).into();
    println!("{sets_path}");

    // Grab the dictionary corresponding to that path, and cast all keys to CFString
    // TODO: is this behavior even correct??????
    //       what should be the reference count of things???
    let sets_dict = get_path_dictionary(&prefs, &sets_path).unwrap();
    let (keys, _) = sets_dict.get_keys_and_values();
    let keys = keys
        .into_iter()
        .map(|k| unsafe { (&*CFType::from_void(k)).downcast::<CFString>().unwrap() })
        .collect::<Vec<_>>();
    for k in keys {
        println!("key -> {}; {}", k, k.retain_count());
    }
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
