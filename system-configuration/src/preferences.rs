// Copyright 2017 Amagicom AB.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bindings to [`SCPreferences`].
//!
//! See the examples directory for examples how to use this module.
//!
//! [`SCPreferences`]: https://developer.apple.com/documentation/systemconfiguration/scpreferences-ft8

use crate::sys::preferences::{SCPreferencesCreate, SCPreferencesGetTypeID, SCPreferencesRef};
use core_foundation::array::CFArray;
use core_foundation::base::{Boolean, CFAllocator, TCFType};
use core_foundation::propertylist::CFPropertyList;
use core_foundation::string::CFString;
use std::ptr;
use sys::preferences::{AuthorizationRef, SCPreferencesApplyChanges, SCPreferencesCommitChanges, SCPreferencesCopyKeyList, SCPreferencesCreateWithAuthorization, SCPreferencesGetValue, SCPreferencesLock, SCPreferencesSynchronize, SCPreferencesUnlock};
#[cfg(feature = "private")]
use sys::preferences_private::kSCPreferencesUseEntitlementAuthorization;

declare_TCFType! {
    /// The handle to an open preferences session for accessing system configuration preferences.
    SCPreferences, SCPreferencesRef
}

impl_TCFType!(SCPreferences, SCPreferencesRef, SCPreferencesGetTypeID);

impl SCPreferences {
    /// Initiates access to the default system preferences using the default allocator.
    pub fn default(calling_process_name: &CFString) -> Self {
        Self::new(None, calling_process_name, None)
    }

    /// Initiates access to the given (`prefs_id`) group of configuration preferences using the
    /// default allocator. To access the default system preferences, use the [`default`]
    /// constructor.
    ///
    /// [`default`]: #method.default
    pub fn group(calling_process_name: &CFString, prefs_id: &CFString) -> Self {
        Self::new(None, calling_process_name, Some(prefs_id))
    }

    /// Initiates access to the per-system set of configuration preferences with a given
    /// allocator and preference group to access. See the underlying [SCPreferencesCreate] function
    /// documentation for details. Use the helper constructors [`default`] and [`group`] for easier
    /// creation of an instance using the default allocator.
    ///
    /// [SCPreferencesCreate]: https://developer.apple.com/documentation/systemconfiguration/1516807-scpreferencescreate?language=objc
    /// [`default`]: #method.default
    /// [`group`]: #method.group
    pub fn new(
        allocator: Option<&CFAllocator>,
        calling_process_name: &CFString,
        prefs_id: Option<&CFString>,
    ) -> Self {
        let allocator_ref = match allocator {
            Some(allocator) => allocator.as_concrete_TypeRef(),
            None => ptr::null(),
        };
        let prefs_id_ref = match prefs_id {
            Some(prefs_id) => prefs_id.as_concrete_TypeRef(),
            None => ptr::null(),
        };

        unsafe {
            SCPreferences::wrap_under_create_rule(SCPreferencesCreate(
                allocator_ref,
                calling_process_name.as_concrete_TypeRef(),
                prefs_id_ref,
            ))
        }
    }

    /// Initiates access to the default system preferences using the default allocator with the
    /// given authorization.
    pub unsafe fn default_with_authorization(
        calling_process_name: &CFString,
        authorization_ref: AuthorizationRef,
    ) -> Self {
        Self::new_with_authorization(None, calling_process_name, None, authorization_ref)
    }

    /// Initiates access to the given (`prefs_id`) group of configuration preferences using the
    /// default allocator and the given authorization. To access the default system preferences
    /// with the given authorization, use the [`default_with_authorization`] constructor.
    ///
    /// [`default_with_authorization`]: #method.default_with_authorization
    pub unsafe fn group_with_authorization(
        calling_process_name: &CFString,
        prefs_id: &CFString,
        authorization_ref: AuthorizationRef,
    ) -> Self {
        Self::new_with_authorization(
            None,
            calling_process_name,
            Some(prefs_id),
            authorization_ref,
        )
    }

    /// Initiates access to the per-system set of configuration preferences with a given allocator
    /// and preference group to access, as well as authorization. See the underlying
    /// [SCPreferencesCreateWithAuthorization] function documentation for details. Use the helper
    /// constructors [`default_with_authorization`] and [`group_with_authorization`] for easier
    /// creation of an instance using the default allocator.
    ///
    /// [SCPreferencesCreateWithAuthorization]: https://developer.apple.com/documentation/systemconfiguration/1516807-scpreferencescreate?language=objc
    /// [`default_with_authorization`]: #method.default_with_authorization
    /// [`group_with_authorization`]: #method.group_with_authorization
    pub unsafe fn new_with_authorization(
        allocator: Option<&CFAllocator>,
        calling_process_name: &CFString,
        prefs_id: Option<&CFString>,
        authorization_ref: AuthorizationRef,
    ) -> Self {
        let allocator_ref = match allocator {
            Some(allocator) => allocator.as_concrete_TypeRef(),
            None => ptr::null(),
        };
        let prefs_id_ref = match prefs_id {
            Some(prefs_id) => prefs_id.as_concrete_TypeRef(),
            None => ptr::null(),
        };

        unsafe {
            SCPreferences::wrap_under_create_rule(SCPreferencesCreateWithAuthorization(
                allocator_ref,
                calling_process_name.as_concrete_TypeRef(),
                prefs_id_ref,
                authorization_ref,
            ))
        }
    }

    /// Returns the currently defined preference keys.
    ///
    /// See [`SCPreferencesCopyKeyList`] for details.
    ///
    /// [`SCPreferencesCopyKeyList`]: https://developer.apple.com/documentation/systemconfiguration/scpreferencescopykeylist(_:)?language=objc
    pub fn get_keys(&self) -> CFArray<CFString> {
        unsafe {
            let array_ref = SCPreferencesCopyKeyList(self.as_concrete_TypeRef());
            assert!(!array_ref.is_null());
            CFArray::wrap_under_create_rule(array_ref)
        }
    }

    /// Retrieves the value associated with the specified preference key. Or `None` if no value exists.
    ///
    /// Use `CFPropertyList::downcast_into` to cast the result into the correct type.
    ///
    /// See [`SCPreferencesGetValue`] for details.
    ///
    /// [`SCPreferencesGetValue`]: https://developer.apple.com/documentation/systemconfiguration/scpreferencesgetvalue(_:_:)?language=objc
    pub fn get<S: Into<CFString>>(&self, key: S) -> Option<CFPropertyList> {
        let cf_key = key.into();
        unsafe {
            let dict_ref =
                SCPreferencesGetValue(self.as_concrete_TypeRef(), cf_key.as_concrete_TypeRef());
            if !dict_ref.is_null() {
                Some(CFPropertyList::wrap_under_get_rule(dict_ref))
            } else {
                None
            }
        }
    }

    /// Obtains exclusive access to the configuration preferences. The `wait` flag indicates whether
    /// the calling process should block, waiting for another process to complete its update operation
    /// and release its lock.
    ///
    /// Returns: `true` if the lock was obtained; `false` if an error occurred.
    pub fn lock(&mut self, wait: bool) -> bool {
        (unsafe { SCPreferencesLock(self.0, wait as Boolean) }) != 0
    }

    /// Releases exclusive access to the configuration preferences.
    ///
    /// Returns: `true` if the lock was obtained; `false` if an error occurred.
    pub fn unlock(&mut self) -> bool {
        (unsafe { SCPreferencesUnlock(self.0) }) != 0
    }

    /// Commits changes made to the configuration preferences to persistent storage. Implicit calls
    /// to the [`lock`](Self::lock) and [`unlock`](Self::lock) functions are made if exclusive
    /// access has not already been established.
    ///
    /// Returns: `true` if the lock was obtained; `false` if an error occurred.
    ///
    /// Note: this function commits changes to persistent storage; to apply the changes to the
    ///       running system, use the [`apply_changes`](Self::apply_changes) function.
    pub fn commit_changes(&mut self) -> bool {
        (unsafe { SCPreferencesCommitChanges(self.0) }) != 0
    }

    /// Requests that the currently stored configuration preferences be applied to the active
    /// configuration.
    ///
    /// Returns: `true` if the lock was obtained; `false` if an error occurred.
    pub fn apply_changes(&mut self) -> bool {
        (unsafe { SCPreferencesApplyChanges(self.0) }) != 0
    }

    /// Synchronizes accessed preferences with committed changes. Any preference values that were
    /// updated (added, set, or removed), but not committed, are discarded.
    ///
    /// See [`SCPreferencesSynchronize`] for more details.
    ///
    /// [`SCPreferencesSynchronize`]: https://developer.apple.com/documentation/systemconfiguration/scpreferencessynchronize(_:)?language=objc
    pub fn synchronize(&mut self) {
        unsafe { SCPreferencesSynchronize(self.0) };
    }
}

#[cfg(feature = "private")]
const _: () = {
    impl SCPreferences {
        /// Initiates access to the default system preferences using the default allocator, using
        /// an implicit authorization derived from the entitlements of the current process.
        pub unsafe fn default_with_current_authorization(calling_process_name: &CFString) -> Self {
            Self::new_with_current_authorization(None, calling_process_name, None)
        }

        /// Initiates access to the given (`prefs_id`) group of configuration preferences using the
        /// default allocator, as well as an implicit authorization derived from the entitlements of
        /// the current process. To access the default system preferences with the given authorization,
        /// use the [`default_with_current_authorization`] constructor.
        ///
        /// [`default_with_current_authorization`]: #method.default_with_current_authorization
        pub fn group_with_current_authorization(
            calling_process_name: &CFString,
            prefs_id: &CFString
        ) -> Self {
            Self::new_with_current_authorization(
                None,
                calling_process_name,
                Some(prefs_id),
            )
        }

        /// Initiates access to the per-system set of configuration preferences with a given allocator
        /// and preference group to access, as well as an implicit authorization derived from the
        /// entitlements of the current process. See the underlying [SCPreferencesCreateWithAuthorization]
        /// function documentation for details. Use the helper constructors
        /// [`default_with_current_authorization`] and [`group_with_current_authorization`]
        /// for easier creation of an instance using the default allocator.
        ///
        /// [SCPreferencesCreateWithAuthorization]: https://developer.apple.com/documentation/systemconfiguration/1516807-scpreferencescreate?language=objc
        /// [`default_with_current_authorization`]: #method.default_with_current_authorization
        /// [`group_with_current_authorization`]: #method.group_with_current_authorization
        pub fn new_with_current_authorization(
            allocator: Option<&CFAllocator>,
            calling_process_name: &CFString,
            prefs_id: Option<&CFString>,
        ) -> Self {
            unsafe { Self::new_with_authorization(allocator, calling_process_name, prefs_id,
                                                  kSCPreferencesUseEntitlementAuthorization) }
        }
    }
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retain_count() {
        let preferences = SCPreferences::default(&CFString::new("test"));
        assert_eq!(preferences.retain_count(), 1);
    }
}
