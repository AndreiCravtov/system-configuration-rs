use core_foundation::{
    base::{Boolean, CFType, TCFType},
    dictionary::CFDictionary,
    string::CFString,
};
use sys::network_configuration::{
    SCNetworkProtocolGetConfiguration, SCNetworkProtocolGetEnabled,
    SCNetworkProtocolGetProtocolType, SCNetworkProtocolGetTypeID, SCNetworkProtocolRef,
    SCNetworkProtocolSetConfiguration, SCNetworkProtocolSetEnabled,
};

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
core_foundation::impl_CFTypeDescription!(SCNetworkProtocol);

// TODO: implement all the other methods a SCNetworkProtocol has
impl SCNetworkProtocol {
    /// Returns a [`bool`] value indicating whether the specified protocol is enabled.
    #[inline]
    pub fn enabled(&self) -> bool {
        unsafe { SCNetworkProtocolGetEnabled(self.0) != 0 }
    }

    /// Get type of the network protocol, if the type is recognized, returns `None` otherwise.
    ///
    /// See [`SCNetworkProtocolGetProtocolType`] for details.
    ///
    /// [`SCNetworkProtocolGetProtocolType`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocolgetprotocoltype(_:)?language=objc
    #[inline]
    pub fn protocol_type(&self) -> Option<SCNetworkProtocolType> {
        SCNetworkProtocolType::from_cfstring(&self.protocol_type_string()?)
    }

    /// Returns the raw protocol type identifier.
    ///
    /// See [`SCNetworkProtocolGetProtocolType`] for details.
    ///
    /// [`SCNetworkProtocolGetProtocolType`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocolgetprotocoltype(_:)?language=objc
    #[inline]
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

    /// Returns the configuration settings associated with the specified protocol. Or `None` if no
    /// configuration settings are associated with the protocol or an error occurred.
    ///
    /// See [`SCNetworkProtocolGetConfiguration`] for details.
    ///
    /// [`SCNetworkProtocolGetConfiguration`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkprotocolgetconfiguration(_:)?language=objc
    #[inline]
    pub fn configuration(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let dictionary_ref = SCNetworkProtocolGetConfiguration(self.as_concrete_TypeRef());
            if !dictionary_ref.is_null() {
                Some(CFDictionary::wrap_under_get_rule(dictionary_ref))
            } else {
                None
            }
        }
    }

    /// Enables or disables the specified protocol.
    ///
    /// Returns: `true` if the enabled status was saved; `false` if an error occurred.
    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) -> bool {
        (unsafe { SCNetworkProtocolSetEnabled(self.0, enabled as Boolean) }) != 0
    }

    /// Stores the configuration settings for the specified network protocol.
    ///
    /// Returns: `true` if the configuration was stored; `false` if an error occurred.
    #[inline]
    pub fn set_configuration(&mut self, config: &CFDictionary<CFString, CFType>) -> bool {
        (unsafe { SCNetworkProtocolSetConfiguration(self.0, config.as_concrete_TypeRef()) }) != 0
    }
}

/// Represents the possible network protocol types.
///
/// See [_Network Protocol Types_] documentation for details.
///
/// [_Network Protocol Types_]: https://developer.apple.com/documentation/systemconfiguration/network-protocol-types?language=objc
#[derive(Debug, PartialEq, Eq)]
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
    #[inline]
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

    /// Returns the string constants used to identify this network protocol type.
    #[inline]
    pub fn to_cfstring(&self) -> CFString {
        use system_configuration_sys::network_configuration::*;
        let wrap_const = |const_str| unsafe { CFString::wrap_under_get_rule(const_str) };
        unsafe {
            match self {
                SCNetworkProtocolType::DNS => wrap_const(kSCNetworkProtocolTypeDNS),
                SCNetworkProtocolType::IPv4 => wrap_const(kSCNetworkProtocolTypeIPv4),
                SCNetworkProtocolType::IPv6 => wrap_const(kSCNetworkProtocolTypeIPv6),
                SCNetworkProtocolType::Proxies => wrap_const(kSCNetworkProtocolTypeProxies),
                SCNetworkProtocolType::SMB => wrap_const(kSCNetworkProtocolTypeSMB),
            }
        }
    }
}
