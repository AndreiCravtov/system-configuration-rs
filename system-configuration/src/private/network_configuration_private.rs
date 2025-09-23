use std::os;
use core_foundation::array::CFArray;
use core_foundation::base::{CFRetain, CFTypeID, CFTypeRef, TCFType, TCFTypeRef, ToVoid};
use core_foundation::propertylist::CFPropertyList;
use sys::network_configuration::SCNetworkInterfaceGetTypeID;
use sys::preferences::SCPreferencesRef;
use sys::private::network_configuration_private::{SCBridgeInterfaceCopyAll, SCBridgeInterfaceRef};
use crate::helpers::create_empty_array;
use crate::network_configuration::{SCNetworkInterface, SCNetworkInterfaceType};
use crate::preferences::SCPreferences;

core_foundation::declare_TCFType! {
    SCBridgeInterface, SCBridgeInterfaceRef
}
core_foundation::impl_CFTypeDescription!(SCBridgeInterface);

// default implementation copied verbatim from `core_foundation::impl_TCFType!(...)` expansion.
//
// only difference is the lack of `ConcreteCFType` implementation, to prevent `CFType::downcast`
// from being implemented, as that would be unsound behavior.
const _: () = {
    impl TCFType for SCBridgeInterface {
        type Ref = SCBridgeInterfaceRef;

        #[inline]
        fn as_concrete_TypeRef(&self) -> SCBridgeInterfaceRef {
            self.0
        }

        #[inline]
        unsafe fn wrap_under_create_rule(reference: SCBridgeInterfaceRef) -> Self {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");


            SCBridgeInterface(reference)
        }

        #[inline]
        fn type_id() -> CFTypeID {
            unsafe {
                SCNetworkInterfaceGetTypeID()
            }
        }

        #[inline]
        fn as_CFTypeRef(&self) -> CFTypeRef {
            self.as_concrete_TypeRef() as CFTypeRef
        }

        #[inline]
        unsafe fn wrap_under_get_rule(reference: SCBridgeInterfaceRef) -> Self {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");
            let reference = CFRetain(reference) as SCBridgeInterfaceRef;
            TCFType::wrap_under_create_rule(reference)
        }
    }
    impl Clone for SCBridgeInterface {
        #[inline]
        fn clone(&self) -> SCBridgeInterface {
            unsafe {
                SCBridgeInterface::wrap_under_get_rule(self.0)
            }
        }
    }
    impl PartialEq for SCBridgeInterface {
        #[inline]
        fn eq(&self, other: &SCBridgeInterface) -> bool {
            self.as_CFType().eq(&other.as_CFType())
        }
    }
    impl Eq for SCBridgeInterface {}
    unsafe impl<'a> ToVoid<SCBridgeInterface> for &'a SCBridgeInterface {
        fn to_void(&self) -> *const os::raw::c_void {
            self.as_concrete_TypeRef().as_void_ptr()
        }
    }
    unsafe impl ToVoid<SCBridgeInterface> for SCBridgeInterface {
        fn to_void(&self) -> *const os::raw::c_void {
            self.as_concrete_TypeRef().as_void_ptr()
        }
    }
    unsafe impl ToVoid<SCBridgeInterface> for SCBridgeInterfaceRef {
        fn to_void(&self) -> *const os::raw::c_void {
            self.as_void_ptr()
        }
    }
};



// /// A thin wrapper around [`SCNetworkInterface`] for those which happen to be **bridge interfaces**.
// #[repr(transparent)]
// pub struct SCBridgeInterface(SCNetworkInterface);
//
// impl SCBridgeInterface {
//     /// Retrieve all current network interfaces
//     ///
//     /// See [`SCNetworkInterfaceCopyAll`] for more details.
//     ///
//     /// [`SCNetworkInterfaceCopyAll`]: https://developer.apple.com/documentation/systemconfiguration/1517090-scnetworkinterfacecopyall?language=objc
//     pub fn get_interfaces(prefs: &SCPreferences) -> CFArray<Self> {
//         // CFPropertyList
//
//         unsafe {
//             let array_ptr = SCBridgeInterfaceCopyAll(prefs.as_concrete_TypeRef());
//             if array_ptr.is_null() {
//                 return create_empty_array();
//             }
//             CFArray::<Self>::wrap_under_get_rule(array_ptr)
//         }
//     }
//
//     /// Dangerously wrap [`SCNetworkInterface`] to obtain a wrapper for **bridge interfaces**.
//     ///
//     /// Returns the **bridge interface** wrapper, but **doesn't check** that the underlying
//     /// interface is _actually_ a _bridge_ interface.
//     pub unsafe fn from_network_interface_unchecked(interface: SCNetworkInterface) -> SCBridgeInterface {
//         SCBridgeInterface(interface)
//     }
//
//     /// Try to wrap [`SCNetworkInterface`] to obtain a wrapper for **bridge interfaces**.
//     ///
//     /// Returns the **bridge interface** wrapper, or `None` if the interface type is wrong.
//     pub fn try_from_network_interface(interface: SCNetworkInterface) -> Option<Self> {
//         if let SCNetworkInterfaceType::Bridge = interface.interface_type()? {
//             Some(SCBridgeInterface(interface))
//         } else {
//             None
//         }
//     }
//
//     /// Unwrap the underlying [`SCNetworkInterface`] instance of this **bridge interface**.
//     pub fn into_network_interface(self) -> SCNetworkInterface {
//         self.0
//     }
// }