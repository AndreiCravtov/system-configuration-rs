use std::os;
use core_foundation::array::CFArray;
use core_foundation::base::{CFRetain, CFTypeID, CFTypeRef, TCFType, TCFTypeRef, ToVoid};
use core_foundation::propertylist::CFPropertyList;
use sys::network_configuration::SCNetworkInterfaceGetTypeID;
use sys::preferences::SCPreferencesRef;
use sys::private::network_configuration_private::{SCBridgeInterfaceCopyAll, SCBridgeInterfaceRef};
use crate::helpers::create_empty_array;
use crate::network_configuration::{SCNetworkInterfaceSubClass, SCNetworkInterfaceType};
use crate::preferences::SCPreferences;

core_foundation::declare_TCFType! {
    /// Represents a bridge interface, which is a subclass of
    /// [`SCNetworkInterface`](crate::network_configuration::SCNetworkInterface).
    SCBridgeInterface, SCBridgeInterfaceRef
}
core_foundation::impl_CFTypeDescription!(SCBridgeInterface);

// default implementation copied verbatim from `core_foundation::impl_TCFType!(...)` expansion.
//
// only difference is the lack of `ConcreteCFType` implementation, to prevent `CFType::downcast`
// from being implemented, as that would be unsound behavior.
//
// also implements `SCNetworkInterfaceSubClass` to allow up/downcasting to/from `SCNetworkInterface`
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
    unsafe impl SCNetworkInterfaceSubClass for SCBridgeInterface {
        const INTERFACE_TYPE: SCNetworkInterfaceType = SCNetworkInterfaceType::Bridge;
    }
};



impl SCBridgeInterface {
    /// Retrieve all current bridge interfaces.
    pub fn get_interfaces(prefs: &SCPreferences) -> CFArray<Self> {
        unsafe {
            let array_ptr = SCBridgeInterfaceCopyAll(prefs.as_concrete_TypeRef());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<Self>::wrap_under_get_rule(array_ptr)
        }
    }
}
