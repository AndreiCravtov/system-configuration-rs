//! Bindings for [`SCNetworkConfiguration`].
//!
//! [`SCNetworkConfiguration`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
use core_foundation::{
    array::CFArray,
    base::{TCFType, ToVoid},
    string::CFString,
};
use sys::network_configuration::{
    SCNetworkInterfaceGetHardwareAddressString, SCNetworkInterfaceGetSupportedInterfaceTypes,
    SCNetworkInterfaceGetSupportedProtocolTypes, SCNetworkProtocolGetProtocolType,
    SCNetworkProtocolGetTypeID, SCNetworkProtocolRef, SCNetworkServiceRemove, SCNetworkSetCopy,
    SCNetworkSetCopyAll, SCNetworkSetCopyServices, SCNetworkSetGetName, SCNetworkSetGetSetID,
    SCNetworkSetRemove,
};
use system_configuration_sys::network_configuration::{
    SCNetworkInterfaceCopyAll, SCNetworkInterfaceGetBSDName, SCNetworkInterfaceGetInterfaceType,
    SCNetworkInterfaceGetLocalizedDisplayName, SCNetworkInterfaceGetTypeID, SCNetworkInterfaceRef,
    SCNetworkServiceCopyAll, SCNetworkServiceGetEnabled, SCNetworkServiceGetInterface,
    SCNetworkServiceGetServiceID, SCNetworkServiceGetTypeID, SCNetworkServiceRef,
    SCNetworkSetCopyCurrent, SCNetworkSetGetServiceOrder, SCNetworkSetGetTypeID, SCNetworkSetRef,
};

use crate::preferences::SCPreferences;

core_foundation::declare_TCFType!(
    /// Represents a network interface.
    ///
    /// See [`SCNetworkInterfaceRef`] and its [methods] for details.
    ///
    /// [`SCNetworkInterfaceRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfaceref?language=objc
    /// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
    SCNetworkInterface,
    SCNetworkInterfaceRef
);
core_foundation::impl_TCFType!(
    SCNetworkInterface,
    SCNetworkInterfaceRef,
    SCNetworkInterfaceGetTypeID
);

// TODO: implement all the other methods a SCNetworkInterface has
impl SCNetworkInterface {
    /// Retrieve all current network interfaces
    ///
    /// See [`SCNetworkInterfaceCopyAll`] for more details.
    ///
    /// [`SCNetworkInterfaceCopyAll`]: https://developer.apple.com/documentation/systemconfiguration/1517090-scnetworkinterfacecopyall?language=objc
    pub fn get_interfaces() -> CFArray<Self> {
        get_interfaces()
    }

    /// Get type of the network interface, if the type is recognized, returns `None` otherwise.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn interface_type(&self) -> Option<SCNetworkInterfaceType> {
        SCNetworkInterfaceType::from_cfstring(&self.interface_type_string()?)
    }

    /// Returns the raw interface type identifier.
    ///
    /// See [`SCNetworkInterfaceGetInterfaceType`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterfaceType`]: https://developer.apple.com/documentation/systemconfiguration/1517371-scnetworkinterfacegetinterfacety?language=objc
    pub fn interface_type_string(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkInterfaceGetInterfaceType(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns the _BSD_ name for the interface, such as `en0`.
    ///
    /// See [`SCNetworkInterfaceGetBSDName`] for details.
    ///
    /// [`SCNetworkInterfaceGetBSDName`]: https://developer.apple.com/documentation/systemconfiguration/1516854-scnetworkinterfacegetbsdname?language=objc
    pub fn bsd_name(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkInterfaceGetBSDName(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Returns a displayable link layer address for the specified interface, i.e. the hardware
    /// MAC (Media Access Control) address for the interface.
    ///
    /// See [`SCNetworkInterfaceGetHardwareAddressString`] for details.
    ///
    /// [`SCNetworkInterfaceGetHardwareAddressString`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfacegethardwareaddressstring(_:)?language=objc
    pub fn hardware_address_string(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkInterfaceGetHardwareAddressString(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }
    /// Returns the localized display name for the interface.
    ///
    /// See [`SCNetworkInterfaceGetLocalizedDisplayName`] for details.
    ///
    /// [`SCNetworkInterfaceGetLocalizedDisplayName`]: https://developer.apple.com/documentation/systemconfiguration/1517060-scnetworkinterfacegetlocalizeddi?language=objc
    pub fn display_name(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkInterfaceGetLocalizedDisplayName(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }

    /// Get all the raw network interface type identifiers, such as PPP, that can be layered on top
    /// of the specified interface.
    ///
    /// See [`SCNetworkInterfaceGetSupportedInterfaceTypes`] for details.
    ///
    /// [`SCNetworkInterfaceGetSupportedInterfaceTypes`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfacegetsupportedinterfacetypes(_:)?language=objc
    pub fn supported_interface_type_strings(&self) -> CFArray<CFString> {
        unsafe {
            let array_ptr = SCNetworkInterfaceGetSupportedInterfaceTypes(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<CFString>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Get all the raw network protocol type identifiers, such as IPv4 and IPv6, that can be
    /// layered on top of the specified interface.
    ///
    /// See [`SCNetworkInterfaceGetSupportedProtocolTypes`] for details.
    ///
    /// [`SCNetworkInterfaceGetSupportedProtocolTypes`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfacegetsupportedprotocoltypes(_:)?language=objc
    pub fn supported_protocol_type_strings(&self) -> CFArray<CFString> {
        unsafe {
            let array_ptr = SCNetworkInterfaceGetSupportedProtocolTypes(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<CFString>::wrap_under_create_rule(array_ptr)
        }
    }
}

/// Represents the possible network interface types.
///
/// See [_Network Interface Types_] documentation for details.
///
/// [_Network Interface Types_]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration/network_interface_types?language=objc
#[derive(Debug)]
pub enum SCNetworkInterfaceType {
    /// A 6to4 interface.
    SixToFour,
    /// Bluetooth interface.
    Bluetooth,
    /// Bridge interface.
    Bridge,
    /// Ethernet bond interface.
    Bond,
    /// Ethernet interface.
    Ethernet,
    /// FireWire interface.
    FireWire,
    /// IEEE80211 interface.
    IEEE80211,
    /// IPSec interface.
    IPSec,
    /// IrDA interface.
    IrDA,
    /// L2TP interface.
    L2TP,
    /// Modem interface.
    Modem,
    /// PPP interface.
    PPP,
    /// PPTP interface.
    ///
    /// Deprecated, one should use the PPP variant.
    PPTP,
    /// Serial interface.
    Serial,
    /// VLAN interface.
    VLAN,
    /// WWAN interface.
    WWAN,
    /// IPv4 interface.
    IPv4,
}

/// Bridge interface type referred to as `kSCNetworkInterfaceTypeBridge` in private headers.
static BRIDGE_INTERFACE_TYPE_ID: &str = "Bridge";

/// IrDA interface referenced as `kSCNetworkInterfaceTypeIrDA` but deprecated since macOS 12.
static IRDA_INTERFACE_TYPE_ID: &str = "IrDA";

impl SCNetworkInterfaceType {
    /// Tries to construct a type by matching it to string constants used to identify a network
    /// interface type. If no constants match it, `None` is returned.
    pub fn from_cfstring(type_id: &CFString) -> Option<Self> {
        use system_configuration_sys::network_configuration::*;

        let id_is_equal_to = |const_str| -> bool {
            let const_str = unsafe { CFString::wrap_under_get_rule(const_str) };
            &const_str == type_id
        };
        unsafe {
            if id_is_equal_to(kSCNetworkInterfaceType6to4) {
                Some(SCNetworkInterfaceType::SixToFour)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBluetooth) {
                Some(SCNetworkInterfaceType::Bluetooth)
            } else if type_id == &BRIDGE_INTERFACE_TYPE_ID {
                Some(SCNetworkInterfaceType::Bridge)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeBond) {
                Some(SCNetworkInterfaceType::Bond)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeEthernet) {
                Some(SCNetworkInterfaceType::Ethernet)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeFireWire) {
                Some(SCNetworkInterfaceType::FireWire)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIEEE80211) {
                Some(SCNetworkInterfaceType::IEEE80211)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIPSec) {
                Some(SCNetworkInterfaceType::IPSec)
            } else if type_id == &IRDA_INTERFACE_TYPE_ID {
                Some(SCNetworkInterfaceType::IrDA)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeL2TP) {
                Some(SCNetworkInterfaceType::L2TP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeModem) {
                Some(SCNetworkInterfaceType::Modem)
            } else if id_is_equal_to(kSCNetworkInterfaceTypePPP) {
                Some(SCNetworkInterfaceType::PPP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypePPTP) {
                Some(SCNetworkInterfaceType::PPTP)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeSerial) {
                Some(SCNetworkInterfaceType::Serial)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeVLAN) {
                Some(SCNetworkInterfaceType::VLAN)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeWWAN) {
                Some(SCNetworkInterfaceType::WWAN)
            } else if id_is_equal_to(kSCNetworkInterfaceTypeIPv4) {
                Some(SCNetworkInterfaceType::IPv4)
            } else {
                None
            }
        }
    }
}

/// Retrieve all current network interfaces
///
/// See [`SCNetworkInterfaceCopyAll`] for more details.
///
/// [`SCNetworkInterfaceCopyAll`]: https://developer.apple.com/documentation/systemconfiguration/1517090-scnetworkinterfacecopyall?language=objc
pub fn get_interfaces() -> CFArray<SCNetworkInterface> {
    unsafe { CFArray::<SCNetworkInterface>::wrap_under_create_rule(SCNetworkInterfaceCopyAll()) }
}

core_foundation::declare_TCFType!(
    /// Represents a network protocol.
    ///
    /// See [`SCNetworkProtocolRef`] and its [methods] for details.
    ///
    /// [`SCNetworkProtocolRef`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocol?language=objc
    /// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
    SCNetworkProtocol,
    SCNetworkProtocolRef
);
core_foundation::impl_TCFType!(
    SCNetworkProtocol,
    SCNetworkProtocolRef,
    SCNetworkProtocolGetTypeID
);

// TODO: implement all the other methods a SCNetworkProtocol has
impl SCNetworkProtocol {
    /// Get type of the network protocol, if the type is recognized, returns `None` otherwise.
    ///
    /// See [`SCNetworkProtocolGetProtocolType`] for details.
    ///
    /// [`SCNetworkProtocolGetProtocolType`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocolgetprotocoltype(_:)?language=objc
    pub fn protocol_type(&self) -> Option<SCNetworkProtocolType> {
        SCNetworkProtocolType::from_cfstring(&self.protocol_type_string()?)
    }

    /// Returns the raw protocol type identifier.
    ///
    /// See [`SCNetworkProtocolGetProtocolType`] for details.
    ///
    /// [`SCNetworkProtocolGetProtocolType`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocolgetprotocoltype(_:)?language=objc
    pub fn protocol_type_string(&self) -> Option<CFString> {
        unsafe {
            let ptr = SCNetworkProtocolGetProtocolType(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(CFString::wrap_under_get_rule(ptr))
            }
        }
    }
}

/// Represents the possible network protocol types.
///
/// See [_Network Protocol Types_] documentation for details.
///
/// [_Network Protocol Types_]: https://developer.apple.com/documentation/systemconfiguration/network-protocol-types?language=objc
#[derive(Debug)]
pub enum SCNetworkProtocolType {
    /// DNS protocol.
    DNS,
    /// IPv4 protocol.
    IPv4,
    /// IPv6 protocol.
    IPv6,
    /// Protocol proxies.
    Proxies,
    /// SMB protocol.
    SMB,
}

impl SCNetworkProtocolType {
    /// Tries to construct a type by matching it to string constants used to identify a network
    /// protocol type. If no constants match it, `None` is returned.
    pub fn from_cfstring(type_id: &CFString) -> Option<Self> {
        use system_configuration_sys::network_configuration::*;

        let id_is_equal_to = |const_str| -> bool {
            let const_str = unsafe { CFString::wrap_under_get_rule(const_str) };
            &const_str == type_id
        };
        unsafe {
            if id_is_equal_to(kSCNetworkProtocolTypeDNS) {
                Some(SCNetworkProtocolType::DNS)
            } else if id_is_equal_to(kSCNetworkProtocolTypeIPv4) {
                Some(SCNetworkProtocolType::IPv4)
            } else if id_is_equal_to(kSCNetworkProtocolTypeIPv6) {
                Some(SCNetworkProtocolType::IPv6)
            } else if id_is_equal_to(kSCNetworkProtocolTypeProxies) {
                Some(SCNetworkProtocolType::Proxies)
            } else if id_is_equal_to(kSCNetworkProtocolTypeSMB) {
                Some(SCNetworkProtocolType::SMB)
            } else {
                None
            }
        }
    }
}

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

    /// Returns true if the network service is currently enabled
    pub fn enabled(&self) -> bool {
        unsafe { SCNetworkServiceGetEnabled(self.0) == 0 }
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

    /// Removes the specified network service from the configuration.
    ///
    /// Returns: `true` if the service was removed; `false` if an error occurred.
    pub fn remove(self) -> bool {
        (unsafe { SCNetworkServiceRemove(self.0) }) != 0
    }
}

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
    pub fn find_set<S: Into<CFString>>(prefs: &SCPreferences, set_id: S) -> Option<SCNetworkSet> {
        let cf_set_id = set_id.into();
        unsafe {
            let set_ref =
                SCNetworkSetCopy(prefs.as_concrete_TypeRef(), cf_set_id.as_concrete_TypeRef());
            if !set_ref.is_null() {
                Some(SCNetworkSet::wrap_under_create_rule(set_ref))
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

    /// Removes the specified set from the configuration.
    ///
    /// Returns: `true` if the set was removed; `false` if an error occurred.
    pub fn remove(self) -> bool {
        (unsafe { SCNetworkSetRemove(self.0) }) != 0
    }
}

fn create_empty_array<T>() -> CFArray<T> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_all_interfaces() {
        let _ = get_interfaces();
    }

    #[test]
    fn test_get_type() {
        for iface in get_interfaces().into_iter() {
            if iface.interface_type().is_none() {
                panic!(
                    "Interface  {:?} ({:?}) has unrecognized type {:?}",
                    iface.display_name(),
                    iface.bsd_name(),
                    iface.interface_type_string()
                )
            }
        }
    }

    #[test]
    fn test_service_order() {
        let prefs = SCPreferences::default(&CFString::new("test"));
        let services = SCNetworkService::get_services(&prefs);
        let set = SCNetworkSet::new(&prefs);
        let service_order = set.service_order();

        assert!(service_order.iter().all(|service_id| {
            services
                .iter()
                .any(|service| service.id().as_ref() == Some(&*service_id))
        }))
    }

    #[test]
    fn test_empty_array() {
        let empty = create_empty_array::<CFString>();
        let values = empty.get_all_values();
        assert!(values.is_empty())
    }
}
