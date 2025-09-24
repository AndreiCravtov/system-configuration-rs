use core_foundation::{
    array::CFArray,
    base::{Boolean, TCFType, ToVoid},
    string::CFString,
};
use sys::network_configuration::{
    SCNetworkServiceAddProtocolType, SCNetworkServiceCopy, SCNetworkServiceCopyAll, SCNetworkServiceCopyProtocol,
    SCNetworkServiceCopyProtocols, SCNetworkServiceCreate, SCNetworkServiceEstablishDefaultConfiguration,
    SCNetworkServiceGetEnabled, SCNetworkServiceGetInterface, SCNetworkServiceGetServiceID, SCNetworkServiceGetTypeID,
    SCNetworkServiceRef, SCNetworkServiceRemove, SCNetworkServiceSetEnabled
};

use super::{SCNetworkInterface, SCNetworkProtocol};
use crate::preferences::SCPreferences;

use crate::helpers::create_empty_array;

core_foundation::declare_TCFType!(
    /// Represents a network service.
    ///
    /// See [`SCNetworkInterfaceRef`] and its [methods] for details.
    ///
    /// [`SCNetworkInterfaceRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkserviceref?language=objc
    /// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
    SCNetworkService,
    SCNetworkServiceRef
);

core_foundation::impl_TCFType!(
    SCNetworkService,
    SCNetworkServiceRef,
    SCNetworkServiceGetTypeID
);
core_foundation::impl_CFTypeDescription!(SCNetworkService);

impl SCNetworkService {
    /// Returns an array of all network services
    pub fn get_services(prefs: &SCPreferences) -> CFArray<Self> {
        unsafe {
            let array_ptr = SCNetworkServiceCopyAll(prefs.to_void());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<Self>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Returns the service with the specified identifier. Or `None` if the service ID does not
    /// exist in the preferences or if an error occurred.
    ///
    /// See [`SCNetworkServiceCopy`] for details.
    ///
    /// [`SCNetworkServiceCopy`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkservicecopy(_:_:)?language=objc
    pub fn find_service<S: Into<CFString>>(prefs: &SCPreferences, service_id: S) -> Option<Self> {
        let cf_service_id = service_id.into();
        unsafe {
            let service_ref = SCNetworkServiceCopy(
                prefs.as_concrete_TypeRef(),
                cf_service_id.as_concrete_TypeRef(),
            );
            if !service_ref.is_null() {
                Some(Self::wrap_under_create_rule(service_ref))
            } else {
                None
            }
        }
    }

    /// Creates a new network service for the specified interface in the configuration. Or `None`
    /// if an error occurred.
    ///
    /// See [`SCNetworkServiceCreate`] for details.
    ///
    /// [`SCNetworkServiceCreate`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkservicecreate(_:_:)?language=objc
    pub fn create(prefs: &SCPreferences, interface: &SCNetworkInterface) -> Option<Self> {
        unsafe {
            let service_ref = SCNetworkServiceCreate(
                prefs.as_concrete_TypeRef(),
                interface.as_concrete_TypeRef(),
            );
            if !service_ref.is_null() {
                Some(Self::wrap_under_create_rule(service_ref))
            } else {
                None
            }
        }
    }

    /// Returns a [`bool`] value indicating whether the specified service is enabled.
    pub fn enabled(&self) -> bool {
        unsafe { SCNetworkServiceGetEnabled(self.0) != 0 }
    }

    /// Returns the service identifier.
    pub fn id(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkServiceGetServiceID(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns the network interface backing this network service, if it has one.
    pub fn network_interface(&self) -> Option<SCNetworkInterface> {
        unsafe {
            let ptr = SCNetworkServiceGetInterface(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(SCNetworkInterface::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns all network protocols associated with the specified service.
    ///
    /// See [`SCNetworkServiceCopyProtocols`] for details.
    ///
    /// [`SCNetworkServiceCopyProtocols`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkservicecopyprotocols(_:)?language=objc
    pub fn network_protocols(&self) -> CFArray<SCNetworkProtocol> {
        unsafe {
            let array_ptr = SCNetworkServiceCopyProtocols(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<SCNetworkProtocol>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Returns the network protocol of the specified type for the specified service. Or `None` if
    /// this protocol has not been added or if an error occurred.
    ///
    /// See [`SCNetworkServiceCopyProtocol`] for details.
    ///
    /// [`SCNetworkServiceCopyProtocol`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkservicecopyprotocol(_:_:)?language=objc
    pub fn find_network_protocol<S: Into<CFString>>(
        &self,
        protocol_type: S,
    ) -> Option<SCNetworkProtocol> {
        let protocol_type_ref = protocol_type.into().as_concrete_TypeRef();
        unsafe {
            let ptr = SCNetworkServiceCopyProtocol(self.0, protocol_type_ref);
            if ptr.is_null() {
                None
            } else {
                Some(SCNetworkProtocol::wrap_under_create_rule(ptr))
            }
        }
    }

    /// Establishes the default configuration for the specified network service. The default
    /// configuration includes the addition of network protocols for the service (with default
    /// configuration options).
    ///
    /// Returns: `true` if the configuration was updated; `false` if an error occurred.
    pub fn establish_default_configuration(&mut self) -> bool {
        (unsafe { SCNetworkServiceEstablishDefaultConfiguration(self.0) }) != 0
    }

    /// Adds the network protocol of the specified type to the specified service. The protocol
    /// configuration is set to default values that are appropriate for the interface associated
    /// with the service.
    ///
    /// Returns: `true` if the protocol was added to the service; `false` if the protocol was
    ///          already present or an error occurred.
    pub fn add_network_protocol<S: Into<CFString>>(&mut self, protocol_type: S) -> bool {
        let protocol_type_ref = protocol_type.into().as_concrete_TypeRef();
        (unsafe { SCNetworkServiceAddProtocolType(self.0, protocol_type_ref) }) != 0
    }

    /// Removes the specified network service from the configuration.
    ///
    /// Returns: `true` if the service was removed; `false` if an error occurred.
    pub fn remove(self) -> bool {
        (unsafe { SCNetworkServiceRemove(self.0) }) != 0
    }

    /// Enables or disables the specified service.
    ///
    /// Returns: `true` if the enabled status was saved; `false` if an error occurred.
    pub fn set_enabled(&mut self, enabled: bool) -> bool {
        (unsafe { SCNetworkServiceSetEnabled(self.0, enabled as Boolean) }) != 0
    }
}
