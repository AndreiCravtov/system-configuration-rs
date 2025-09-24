#![allow(non_snake_case)]

use std::{mem, ptr};
use core_foundation::{
    array::CFArray,
    base::{TCFType, TCFTypeRef},
    string::CFString,
};
use sys::network_configuration::{SCNetworkInterfaceCopyAll, SCNetworkInterfaceCopyMTU, SCNetworkInterfaceGetBSDName, SCNetworkInterfaceGetHardwareAddressString, SCNetworkInterfaceGetInterface, SCNetworkInterfaceGetInterfaceType, SCNetworkInterfaceGetLocalizedDisplayName, SCNetworkInterfaceGetSupportedInterfaceTypes, SCNetworkInterfaceGetSupportedProtocolTypes, SCNetworkInterfaceGetTypeID, SCNetworkInterfaceRef};

use crate::helpers::create_empty_array;

/// Trait for all subclasses of [`SCNetworkInterface`].
///
/// [`SCNetworkInterface`]: struct.SCNetworkInterface.html
pub unsafe trait SCNetworkInterfaceSubClass: TCFType {
    /// Determines what the type subclass of [`SCNetworkInterface`] this is.
    const INTERFACE_TYPE: SCNetworkInterfaceType;

    /// Create an instance of the superclass type [`SCNetworkInterface`] for this instance.
    ///
    /// [`SCNetworkInterface`]: struct.SCNetworkInterface.html
    #[inline]
    fn to_SCNetworkInterface(&self) -> SCNetworkInterface {
        unsafe { SCNetworkInterface::wrap_under_get_rule(self.as_concrete_TypeRef().as_void_ptr()) }
    }

    /// Equal to [`to_SCNetworkInterface`], but consumes self and avoids changing the reference count.
    ///
    /// [`to_SCNetworkInterface`]: #method.to_SCNetworkInterface
    #[inline]
    fn into_SCNetworkInterface(self) -> SCNetworkInterface
    where
        Self: Sized,
    {
        let reference = self.as_concrete_TypeRef().as_void_ptr();
        mem::forget(self);
        unsafe { SCNetworkInterface::wrap_under_create_rule(reference) }
    }
}

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
core_foundation::impl_CFTypeDescription!(SCNetworkInterface);

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

    /// Try to downcast the [`SCNetworkInterface`] to a subclass. Checking if the instance is the
    /// correct subclass happens at runtime and `None` is returned if it is not the correct type.
    /// Works similar to [`CFPropertyList::downcast`](core_foundation::propertylist::CFPropertyList::downcast)
    /// and [`CFType::downcast`](core_foundation::base::CFType::downcast).
    pub fn downcast_SCNetworkInterface<T: SCNetworkInterfaceSubClass>(&self) -> Option<T> {
        if self.instance_of::<T>() && self.interface_type()? == T::INTERFACE_TYPE {
            unsafe {
                let subclass_ref = T::Ref::from_void_ptr(self.0);
                Some(T::wrap_under_get_rule(subclass_ref))
            }
        } else {
            None
        }
    }

    /// Similar to [`downcast_SCNetworkInterface`], but consumes self and can thus avoid touching
    /// the retain count.
    ///
    /// [`downcast_SCNetworkInterface`]: #method.downcast_SCNetworkInterface
    pub fn downcast_into_SCNetworkInterface<T: SCNetworkInterfaceSubClass>(self) -> Option<T> {
        if self.instance_of::<T>() && self.interface_type()? == T::INTERFACE_TYPE {
            unsafe {
                let subclass_ref = T::Ref::from_void_ptr(self.0);
                mem::forget(self);
                Some(T::wrap_under_create_rule(subclass_ref))
            }
        } else {
            None
        }
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

    /// Returns the underlying interface, for layered network interfaces. Or `None` if the specified
    /// interface is a leaf interface.
    ///
    /// See [`SCNetworkInterfaceGetInterface`] for details.
    ///
    /// [`SCNetworkInterfaceGetInterface`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfacegetinterface(_:)?language=objc
    pub fn underlying_interface(&self) -> Option<Self> {
        unsafe {
            let ptr = SCNetworkInterfaceGetInterface(self.0);
            if ptr.is_null() {
                None
            } else {
                Some(Self::wrap_under_get_rule(ptr))
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

    /// Returns the current MTU setting and the range of allowable values for the specified network
    /// interface. Or `None` if the requested information could not be found.
    ///
    /// See [`SCNetworkInterfaceCopyMTU`] for more details.
    ///
    /// [`SCNetworkInterfaceCopyMTU`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkinterfacecopymtu(_:_:_:_:)?language=objc
    pub fn mtu(&self) -> Option<SCNetworkInterfaceMTU> {
        // perform call w/ out parameters
        let mut mtu_cur: std::ffi::c_int = -1;
        let mut mtu_min: std::ffi::c_int = -1;
        let mut mtu_max: std::ffi::c_int = -1;
        let succeeded = (unsafe { SCNetworkInterfaceCopyMTU(
            self.as_concrete_TypeRef(), &mut mtu_cur, &mut mtu_min, &mut mtu_max) }) != 0;

        let mtu_cur_bytes = if !succeeded { return None; } else {
            assert!(mtu_cur >= 0, "if `SCNetworkInterfaceCopyMTU` succeeded, `mtu_cur` MUST be non-negative");
            mtu_cur as u32
        };

        // if `mtu_min` and `mtu_max` are negative, then those settings could not be determined
        let mtu_min_bytes = if mtu_min >= 0 { Some(mtu_min as u32) } else { None };
        let mtu_max_bytes = if mtu_max >= 0 { Some(mtu_max as u32) } else { None };

        Some(SCNetworkInterfaceMTU { mtu_cur_bytes, mtu_min_bytes, mtu_max_bytes })
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
            CFArray::<CFString>::wrap_under_get_rule(array_ptr)
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
            CFArray::<CFString>::wrap_under_get_rule(array_ptr)
        }
    }

    pub fn set_mtu(&self, mtu: u32) {
        let mtu: Result<std::ffi::c_int, _> = TryFrom::try_from(mtu);
    }
}

/// Represents the current MTU settings of an [`SCNetworkInterface`], including the current MTU and
/// potentially then minimum/maximum allowed MTU values for that interface.
///
/// See [`mtu`](SCNetworkInterface::mtu) for more details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct SCNetworkInterfaceMTU {
    pub mtu_cur_bytes: u32,
    pub mtu_min_bytes: Option<u32>,
    pub mtu_max_bytes: Option<u32>,
}

/// Represents the possible network interface types.
///
/// See [_Network Interface Types_] documentation for details.
///
/// [_Network Interface Types_]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration/network_interface_types?language=objc
#[derive(Debug, PartialEq, Eq)]
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
#[cfg(not(feature = "private"))]
static BRIDGE_INTERFACE_TYPE_ID: &str = "Bridge";

/// IrDA interface referenced as `kSCNetworkInterfaceTypeIrDA` but deprecated since macOS 12.
static IRDA_INTERFACE_TYPE_ID: &str = "IrDA";

impl SCNetworkInterfaceType {
    /// Tries to construct a type by matching it to string constants used to identify a network
    /// interface type. If no constants match it, `None` is returned.
    pub fn from_cfstring(type_id: &CFString) -> Option<Self> {
        #[cfg(feature = "private")]
        use system_configuration_sys::private::network_configuration_private::*;
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
            } else if {
                #[cfg(feature = "private")]
                let matches = id_is_equal_to(kSCNetworkInterfaceTypeBridge);

                #[cfg(not(feature = "private"))]
                let matches = type_id == &BRIDGE_INTERFACE_TYPE_ID;

                matches
            } {
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

    /// Returns the string constants used to identify this network interface type.
    pub fn to_cfstring(&self) -> CFString {
        #[cfg(feature = "private")]
        use system_configuration_sys::private::network_configuration_private::*;
        use system_configuration_sys::network_configuration::*;
        let wrap_const = |const_str| unsafe { CFString::wrap_under_get_rule(const_str) };
        unsafe {
            match self {
                SCNetworkInterfaceType::SixToFour => wrap_const(kSCNetworkInterfaceType6to4),
                SCNetworkInterfaceType::Bluetooth => wrap_const(kSCNetworkInterfaceTypeBluetooth),
                SCNetworkInterfaceType::Bridge => {
                    #[cfg(feature = "private")]
                    let val = wrap_const(kSCNetworkInterfaceTypeBridge);

                    #[cfg(not(feature = "private"))]
                    let val = BRIDGE_INTERFACE_TYPE_ID.into();

                    val
                },
                SCNetworkInterfaceType::Bond => wrap_const(kSCNetworkInterfaceTypeBond),
                SCNetworkInterfaceType::Ethernet => wrap_const(kSCNetworkInterfaceTypeEthernet),
                SCNetworkInterfaceType::FireWire => wrap_const(kSCNetworkInterfaceTypeFireWire),
                SCNetworkInterfaceType::IEEE80211 => wrap_const(kSCNetworkInterfaceTypeIEEE80211),
                SCNetworkInterfaceType::IPSec => wrap_const(kSCNetworkInterfaceTypeIPSec),
                SCNetworkInterfaceType::IrDA => IRDA_INTERFACE_TYPE_ID.into(),
                SCNetworkInterfaceType::L2TP => wrap_const(kSCNetworkInterfaceTypeL2TP),
                SCNetworkInterfaceType::Modem => wrap_const(kSCNetworkInterfaceTypeModem),
                SCNetworkInterfaceType::PPP => wrap_const(kSCNetworkInterfaceTypePPP),
                SCNetworkInterfaceType::PPTP => wrap_const(kSCNetworkInterfaceTypePPTP),
                SCNetworkInterfaceType::Serial => wrap_const(kSCNetworkInterfaceTypeSerial),
                SCNetworkInterfaceType::VLAN => wrap_const(kSCNetworkInterfaceTypeVLAN),
                SCNetworkInterfaceType::WWAN => wrap_const(kSCNetworkInterfaceTypeWWAN),
                SCNetworkInterfaceType::IPv4 => wrap_const(kSCNetworkInterfaceTypeIPv4),
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
