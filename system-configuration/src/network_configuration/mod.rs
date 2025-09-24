//! Bindings for [`SCNetworkConfiguration`].
//!
//! [`SCNetworkConfiguration`]: https://developer.apple.com/documentation/systemconfiguration/scnetworkconfiguration?language=objc

mod network_interface;
mod network_protocol;
mod network_service;
mod network_set;
mod bond_interface;

#[cfg(feature = "private")]
mod private {
    mod bridge_interface;

    use super::*;
    pub use bridge_interface::*;
}

pub use network_interface::*;
pub use network_protocol::*;
pub use network_service::*;
pub use network_set::*;
#[cfg(feature = "private")]
pub use private::*;

#[cfg(test)]
mod test {
    use core_foundation::string::CFString;
    use crate::preferences::SCPreferences;

    use crate::helpers::create_empty_array;
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
