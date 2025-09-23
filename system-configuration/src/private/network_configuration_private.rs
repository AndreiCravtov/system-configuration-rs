use core_foundation::array::CFArray;
use core_foundation::base::TCFType;
use core_foundation::propertylist::CFPropertyList;
use sys::preferences::SCPreferencesRef;
use sys::private::network_configuration_private::SCBridgeInterfaceCopyAll;
use crate::helpers::create_empty_array;
use crate::network_configuration::{SCNetworkInterface, SCNetworkInterfaceType};
use crate::preferences::SCPreferences;

/// A thin wrapper around [`SCNetworkInterface`] for those which happen to be **bridge interfaces**.
#[repr(transparent)]
pub struct SCBridgeInterface(SCNetworkInterface);

impl SCBridgeInterface {
    /// Retrieve all current network interfaces
    ///
    /// See [`SCNetworkInterfaceCopyAll`] for more details.
    ///
    /// [`SCNetworkInterfaceCopyAll`]: https://developer.apple.com/documentation/systemconfiguration/1517090-scnetworkinterfacecopyall?language=objc
    pub fn get_interfaces(prefs: &SCPreferences) -> CFArray<Self> {
        // CFPropertyList
        
        unsafe {
            let array_ptr = SCBridgeInterfaceCopyAll(prefs.as_concrete_TypeRef());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<Self>::wrap_under_get_rule(array_ptr)
        }
    }

    /// Dangerously wrap [`SCNetworkInterface`] to obtain a wrapper for **bridge interfaces**.
    ///
    /// Returns the **bridge interface** wrapper, but **doesn't check** that the underlying
    /// interface is _actually_ a _bridge_ interface.
    pub unsafe fn from_network_interface_unchecked(interface: SCNetworkInterface) -> SCBridgeInterface {
        SCBridgeInterface(interface)
    }

    /// Try to wrap [`SCNetworkInterface`] to obtain a wrapper for **bridge interfaces**.
    ///
    /// Returns the **bridge interface** wrapper, or `None` if the interface type is wrong.
    pub fn try_from_network_interface(interface: SCNetworkInterface) -> Option<Self> {
        if let SCNetworkInterfaceType::Bridge = interface.interface_type()? {
            Some(SCBridgeInterface(interface))
        } else {
            None
        }
    }

    /// Unwrap the underlying [`SCNetworkInterface`] instance of this **bridge interface**.
    pub fn into_network_interface(self) -> SCNetworkInterface {
        self.0
    }
}