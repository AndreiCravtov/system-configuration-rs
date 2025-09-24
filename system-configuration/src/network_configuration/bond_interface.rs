#![allow(non_snake_case)]

use std::os;
use core_foundation::{
    array::CFArray,
    base::{CFRetain, CFType, CFTypeID, CFTypeRef, TCFType, TCFTypeRef, ToVoid},
    dictionary::CFDictionary,
    string::CFString,
};
use sys::network_configuration::{
    SCNetworkInterfaceGetTypeID, SCBondInterfaceCopyAll, SCBondInterfaceRef, SCBondInterfaceCopyAvailableMemberInterfaces,
    SCBondInterfaceCreate, SCBondInterfaceGetMemberInterfaces, SCBondInterfaceGetOptions, SCBondInterfaceRemove,
    SCBondInterfaceSetMemberInterfaces, SCBondInterfaceSetOptions
};
use super::{SCNetworkInterface, SCNetworkInterfaceSubClass, SCNetworkInterfaceType};
use crate::preferences::SCPreferences;

use crate::helpers::create_empty_array;


core_foundation::declare_TCFType! {
    /// Represents a bond interface, which is a subclass of [`SCNetworkInterface`](SCNetworkInterface).
    ///
    /// See [`SCBondInterfaceRef`] and its [methods] for details.
    ///
    /// [`SCBondInterfaceRef`]: https://developer.apple.com/documentation/systemconfiguration/scbondinterface?language=objc
    /// [methods]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc
    SCBondInterface, SCBondInterfaceRef
}
core_foundation::impl_CFTypeDescription!(SCBondInterface);

// default implementation copied verbatim from `core_foundation::impl_TCFType!(...)` expansion.
//
// only difference is the lack of `ConcreteCFType` implementation, to prevent `CFType::downcast`
// from being implemented, as that would be unsound behavior.
//
// also implements `SCNetworkInterfaceSubClass` to allow up/downcasting to/from `SCNetworkInterface`
const _: () = {
    impl TCFType for SCBondInterface {
        type Ref = SCBondInterfaceRef;

        #[inline]
        fn as_concrete_TypeRef(&self) -> SCBondInterfaceRef {
            self.0
        }

        #[inline]
        unsafe fn wrap_under_create_rule(reference: SCBondInterfaceRef) -> Self {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");
            SCBondInterface(reference)
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
        unsafe fn wrap_under_get_rule(reference: SCBondInterfaceRef) -> Self {
            assert!(!reference.is_null(), "Attempted to create a NULL object.");
            let reference = CFRetain(reference) as SCBondInterfaceRef;
            TCFType::wrap_under_create_rule(reference)
        }
    }
    impl Clone for SCBondInterface {
        #[inline]
        fn clone(&self) -> SCBondInterface {
            unsafe {
                SCBondInterface::wrap_under_get_rule(self.0)
            }
        }
    }
    impl PartialEq for SCBondInterface {
        #[inline]
        fn eq(&self, other: &SCBondInterface) -> bool {
            self.as_CFType().eq(&other.as_CFType())
        }
    }
    impl Eq for SCBondInterface {}
    unsafe impl<'a> ToVoid<SCBondInterface> for &'a SCBondInterface {
        fn to_void(&self) -> *const os::raw::c_void {
            use TCFTypeRef;
            self.as_concrete_TypeRef().as_void_ptr()
        }
    }
    unsafe impl ToVoid<SCBondInterface> for SCBondInterface {
        fn to_void(&self) -> *const os::raw::c_void {
            use TCFTypeRef;
            self.as_concrete_TypeRef().as_void_ptr()
        }
    }
    unsafe impl ToVoid<SCBondInterface> for SCBondInterfaceRef {
        fn to_void(&self) -> *const os::raw::c_void {
            use TCFTypeRef;
            self.as_void_ptr()
        }
    }
    unsafe impl SCNetworkInterfaceSubClass for SCBondInterface {
        const INTERFACE_TYPE: SCNetworkInterfaceType = SCNetworkInterfaceType::Bond;
    }
};

impl SCBondInterface {
    /// Retrieve all network capable devices on the system that can be added to an Ethernet bond
    /// interface.
    ///
    /// See [`SCBondInterfaceCopyAvailableMemberInterfaces`] for more details.
    ///
    /// [`SCBondInterfaceCopyAvailableMemberInterfaces`]: https://developer.apple.com/documentation/systemconfiguration/scbondinterfacecopyavailablememberinterfaces(_:)?language=objc
    pub fn get_available_member_interfaces(prefs: &SCPreferences) -> CFArray<SCNetworkInterface> {
        unsafe {
            let array_ptr = SCBondInterfaceCopyAvailableMemberInterfaces(prefs.as_concrete_TypeRef());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<SCNetworkInterface>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Retrieve all Ethernet bond interfaces on the system.
    ///
    /// See [`SCBondInterfaceCopyAll`] for more details.
    ///
    /// [`SCBondInterfaceCopyAll`]: https://developer.apple.com/documentation/systemconfiguration/scbondinterfacecopyall(_:)?language=objc
    pub fn get_interfaces(prefs: &SCPreferences) -> CFArray<Self> {
        unsafe {
            let array_ptr = SCBondInterfaceCopyAll(prefs.as_concrete_TypeRef());
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<Self>::wrap_under_create_rule(array_ptr)
        }
    }

    /// Creates a new Ethernet bond interface. Or `None` if an error occurred.
    ///
    /// See [`SCBondInterfaceCreate`] for more details.
    ///
    /// [`SCBondInterfaceCreate`]: https://developer.apple.com/documentation/systemconfiguration/scbondinterfacecreate(_:)?language=objc
    pub fn create(prefs: &SCPreferences) -> Option<Self> {
        unsafe {
            let bond_ref = SCBondInterfaceCreate(prefs.as_concrete_TypeRef());
            if !bond_ref.is_null() {
                Some(Self::wrap_under_create_rule(bond_ref))
            } else {
                None
            }
        }
    }

    /// Returns the member interfaces for the specified Ethernet bond interface.
    ///
    /// See [`SCBondInterfaceGetMemberInterfaces`] for more details.
    ///
    /// [`SCBondInterfaceGetMemberInterfaces`]: https://developer.apple.com/documentation/systemconfiguration/scbondinterfacegetmemberinterfaces(_:)?language=objc
    pub fn member_interfaces(&self) -> CFArray<SCNetworkInterface> {
        unsafe {
            let array_ptr = SCBondInterfaceGetMemberInterfaces(self.0);
            if array_ptr.is_null() {
                return create_empty_array();
            }
            CFArray::<SCNetworkInterface>::wrap_under_get_rule(array_ptr)
        }
    }

    /// Returns the configuration settings associated with the specified Ethernet bond interface.
    /// Or `None` if no changes to the default configuration have been saved.
    ///
    /// See [`SCBondInterfaceGetOptions`] for more details.
    ///
    /// [`SCBondInterfaceGetOptions`]: https://developer.apple.com/documentation/systemconfiguration/scbondinterfacegetoptions(_:)?language=objc
    pub fn options(&self) -> Option<CFDictionary<CFString, CFType>> {
        unsafe {
            let dictionary_ref = SCBondInterfaceGetOptions(self.as_concrete_TypeRef());
            if !dictionary_ref.is_null() {
                Some(CFDictionary::wrap_under_get_rule(dictionary_ref))
            } else {
                None
            }
        }
    }

    /// Removes the Ethernet bond interface from the configuration.
    ///
    /// Returns: `true` if the interface was removed; `false` if an error was encountered.
    pub fn remove(self) -> bool {
        (unsafe { SCBondInterfaceRemove(self.0) }) != 0
    }

    /// Sets the member interfaces for the specified Ethernet bond interface.
    ///
    /// Returns: `true` if the configuration was stored; `false` if an error occurred.
    pub fn set_member_interfaces(&mut self, members: &CFArray<SCNetworkInterface>) -> bool {
        (unsafe { SCBondInterfaceSetMemberInterfaces(self.0, members.as_concrete_TypeRef()) }) != 0
    }

    /// Sets the configuration settings for the specified Ethernet bond interface.
    ///
    /// Returns: `true` if the configuration was stored; `false` if an error occurred.
    pub fn set_options(&mut self, new_options: &CFDictionary<CFString, CFType>) -> bool {
        (unsafe { SCBondInterfaceSetOptions(self.0, new_options.as_concrete_TypeRef()) }) != 0
    }
}
