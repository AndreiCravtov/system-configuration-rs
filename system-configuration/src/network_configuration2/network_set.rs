use std::mem;
use core_foundation::{
    array::CFArray,
    base::{Boolean, TCFType, ToVoid, TCFTypeRef, CFType},
    string::CFString,
    dictionary::CFDictionary,
};
use sys::network_configuration::{SCNetworkInterfaceCopyAll, SCNetworkInterfaceGetBSDName, SCNetworkInterfaceGetHardwareAddressString, SCNetworkInterfaceGetInterface, SCNetworkInterfaceGetInterfaceType, SCNetworkInterfaceGetLocalizedDisplayName, SCNetworkInterfaceGetSupportedInterfaceTypes, SCNetworkInterfaceGetSupportedProtocolTypes, SCNetworkInterfaceGetTypeID, SCNetworkInterfaceRef, SCNetworkProtocolGetConfiguration, SCNetworkProtocolGetEnabled, SCNetworkProtocolGetProtocolType, SCNetworkProtocolGetTypeID, SCNetworkProtocolRef, SCNetworkProtocolSetConfiguration, SCNetworkProtocolSetEnabled, SCNetworkServiceAddProtocolType, SCNetworkServiceCopy, SCNetworkServiceCopyAll, SCNetworkServiceCopyProtocol, SCNetworkServiceCopyProtocols, SCNetworkServiceCreate, SCNetworkServiceEstablishDefaultConfiguration, SCNetworkServiceGetEnabled, SCNetworkServiceGetInterface, SCNetworkServiceGetServiceID, SCNetworkServiceGetTypeID, SCNetworkServiceRef, SCNetworkServiceRemove, SCNetworkServiceSetEnabled, SCNetworkSetAddService, SCNetworkSetContainsInterface, SCNetworkSetCopy, SCNetworkSetCopyAll, SCNetworkSetCopyCurrent, SCNetworkSetCopyServices, SCNetworkSetGetName, SCNetworkSetGetServiceOrder, SCNetworkSetGetSetID, SCNetworkSetGetTypeID, SCNetworkSetRef, SCNetworkSetRemove, SCNetworkSetRemoveService, SCNetworkSetSetCurrent, SCNetworkSetSetServiceOrder};

use super::{SCNetworkInterface, SCNetworkProtocol, SCNetworkService};

use crate::preferences::SCPreferences;
use crate::helpers::create_empty_array;

core_foundation::declare_TCFType!(
    /// Represents a complete network configuration for a particular host.
    ///
    /// See [`SCNetworkSet`] for details.
    ///
    /// [`SCNetworkSet`]: https://developer.apple.com/documentation/systemconfiguration/scnetworksetref?language=objc
    SCNetworkSet,
    SCNetworkSetRef
);
core_foundation::impl_TCFType!(SCNetworkSet, SCNetworkSetRef, SCNetworkSetGetTypeID);
core_foundation::impl_CFTypeDescription!(SCNetworkSet);

impl SCNetworkSet {
    /// Returns all available sets for the specified preferences session.
    pub fn get_sets(prefs: &SCPreferences) -> CFArray<Self> {
        unsafe {
            let array_ptr = SCNetworkSetCopyAll(prefs.to_void());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<Self>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Returns the current set. Or `None` if no current set has been defined.
    pub fn get_current(prefs: &SCPreferences) -> Option<Self> {
        unsafe {
            let set_ref = SCNetworkSetCopyCurrent(prefs.as_concrete_TypeRef());
            if !set_ref.is_null() {
                Some(SCNetworkSet::wrap_under_create_rule(set_ref))
            } else {
                None
            }
        }
    }

    /// Returns the set with the specified identifier. Or `None` if the identifier does not exist
    /// in the preferences or if an error occurred
    ///
    /// See [`SCNetworkSetCopy`] for details.
    ///
    /// [`SCNetworkSetCopy`]: https://developer.apple.com/documentation/systemconfiguration/scnetworksetcopy(_:_:)?language=objc
    pub fn find_set<S: Into<CFString>>(prefs: &SCPreferences, set_id: S) -> Option<Self> {
        let cf_set_id = set_id.into();
        unsafe {
            let set_ref =
                SCNetworkSetCopy(prefs.as_concrete_TypeRef(), cf_set_id.as_concrete_TypeRef());
            if !set_ref.is_null() {
                Some(Self::wrap_under_create_rule(set_ref))
            } else {
                None
            }
        }
    }

    /// Constructs a new set of network services from the preferences.
    pub fn new(prefs: &SCPreferences) -> Self {
        let ptr = unsafe { SCNetworkSetCopyCurrent(prefs.to_void()) };
        unsafe { SCNetworkSet::wrap_under_create_rule(ptr) }
    }

    /// Returns all network services associated with the specified set.
    pub fn services(&self) -> CFArray<SCNetworkService> {
        unsafe {
            let array_ptr = SCNetworkSetCopyServices(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<SCNetworkService>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Returns an list of network service identifiers, ordered by their priority.
    pub fn service_order(&self) -> CFArray<CFString> {
        unsafe {
            let array_ptr = SCNetworkSetGetServiceOrder(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<CFString>::wrap_under_get_rule(array_ptr)
        }
    }

    /// Returns the identifier for the specified set.
    pub fn id(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkSetGetSetID(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns the user-specified name associated with the specified set. Or `None` if it hasn't
    /// been defined.
    pub fn name(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkSetGetName(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns a [`bool`] value indicating whether the specified interface is represented by at
    /// least one network service in the specified set.
    pub fn contains_network_interface(&self, interface: &SCNetworkInterface) -> bool {
        let iface_ref = interface.as_concrete_TypeRef();
        (unsafe { SCNetworkSetContainsInterface(self.0, iface_ref) }) != 0
    }

    /// Adds the specified network service to the specified set.
    ///
    /// Returns: `true` if the service was added to the set; `false` if the service was already
    ///          present or an error occurred.
    pub fn add_service(&mut self, service: &SCNetworkService) -> bool {
        let service_ref = service.as_concrete_TypeRef();
        (unsafe { SCNetworkSetAddService(self.0, service_ref) }) != 0
    }

    /// Removes the specified set from the configuration.
    ///
    /// Returns: `true` if the set was removed; `false` if an error occurred.
    pub fn remove(self) -> bool {
        (unsafe { SCNetworkSetRemove(self.0) }) != 0
    }

    /// Removes the specified network service from the specified set.
    ///
    /// Returns: `true` if the service was removed from the set; `false` if the service was not
    ///          already present or an error occurred.
    pub fn remove_service(&mut self, service: &SCNetworkService) -> bool {
        let service_ref = service.as_concrete_TypeRef();
        (unsafe { SCNetworkSetRemoveService(self.0, service_ref) }) != 0
    }

    /// Specifies the set that should be the current set.
    ///
    /// Returns: `true` if the current set was updated; `false` if an error occurred.
    pub fn set_current(&mut self) -> bool {
        (unsafe { SCNetworkSetSetCurrent(self.0) }) != 0
    }

    /// Stores the user-specified ordering of network services for the specified set.
    ///
    /// Returns: `true` if the new service order was saved; `false` if an error occurred.
    pub fn set_service_order(&mut self, new_order: CFArray<CFString>) -> bool {
        let cf_order_ref = new_order.as_concrete_TypeRef();
        (unsafe { SCNetworkSetSetServiceOrder(self.0, cf_order_ref) }) != 0
    }
}