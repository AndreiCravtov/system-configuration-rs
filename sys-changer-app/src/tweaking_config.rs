use crate::ext::CFArrayExt;
use crate::helper;
use crate::interfaces::Interface;
use core_foundation::array::CFArray;
use system_configuration::network_configuration::{
    SCNetworkInterface, SCNetworkInterfaceType, SCNetworkProtocolType, SCNetworkService,
    SCNetworkSet,
};
use system_configuration::preferences::SCPreferences;

pub fn modify_existing_services(prefs: &SCPreferences, set: &mut SCNetworkSet) {
    let ordered_services = get_priority_ordered_services(set);

    // iterate over the existing ordered services, removing or replacing them as necessary
    let ordered_services = ordered_services
        .into_iter()
        .filter_map(|s| match ServiceModifications::gather(&s) {
            None => Some(s),
            Some(ServiceModifications::Delete) => None,
            Some(ServiceModifications::Modify {
                enable,
                protocol: proto_mods,
            }) => {
                // create a duplicate service on which to apply our changes => we don't taint the
                // original services with our modifications
                let mut new_service = helper::shallow_clone_network_service(prefs, &s);
                apply_modifications(&mut new_service, enable, proto_mods);
                Some(new_service)
            }
        })
        .collect::<Vec<_>>();

    // apply the modified services to the network set -> remove all previous services and add back
    // the new ones as an easy way of achieving this goal; finally just replace the service order
    // with the new one
    let old_services = set.services();
    for s in &old_services {
        assert!(set.remove_service(&s));
    }
    for s in &ordered_services {
        assert!(set.add_service(s));
    }
    let service_order = CFArray::from_CFTypes(
        &ordered_services
            .iter()
            .filter_map(|s| s.id())
            .collect::<Box<[_]>>(),
    );
    set.set_service_order(service_order);
}

pub fn add_missing_services(prefs: &SCPreferences, set: &mut SCNetworkSet) {
    // filter for interfaces that don't already have services in the set, which aren't the Bridge
    // interface & also which support IPv6
    let eligible_ifaces = SCNetworkInterface::get_interfaces()
        .into_iter()
        .filter(|i| !set.contains_network_interface(&i))
        .filter(|i| {
            i.interface_type()
                .map_or(false, |t| !matches!(t, SCNetworkInterfaceType::Bridge))
        })
        .filter_map(|i| {
            (&i).supported_protocol_type_strings()
                .into_iter()
                .filter_map(|s| SCNetworkProtocolType::from_cfstring(&s))
                .find(|p| matches!(p, SCNetworkProtocolType::IPv6))
                .map(|_| i.clone())
        })
        .collect::<Vec<_>>();
    println!(
        "creating this {:#?}",
        eligible_ifaces
            .iter()
            .filter_map(Interface::from_scnetwork_interface)
            .collect::<Vec<_>>()
    );

    // create new services for each interface, add IPv6 to each & then establish a default configuration in general
    let new_services = eligible_ifaces
        .iter()
        .map(|i| {
            let mut service = helper::create_service(prefs, i);
            assert!(service.add_network_protocol(SCNetworkProtocolType::IPv6.to_cfstring()));
            service.establish_default_configuration();
            service
        })
        .collect::<Vec<_>>();

    // get the current service order & extend it with the new services
    let mut service_order = set.service_order().into_collect::<Vec<_>>();
    service_order.extend(new_services.iter().map(|i| i.id().unwrap()));
    let service_order = CFArray::from_CFTypes(&service_order);

    // add all new services and set new service order
    for s in &new_services {
        assert!(set.add_service(s));
    }
    assert!(set.set_service_order(service_order))
}

/// Modifications needed to be done to a service.
#[derive(Debug)]
enum ServiceModifications {
    /// This service needs to be deleted.
    Delete,
    /// This service needs to be modified.
    Modify {
        /// Whether the service needs to be marked as enabled.
        enable: bool,
        /// Protocol modifications to perform.
        protocol: Option<ProtocolModifications>,
    },
}

impl ServiceModifications {
    /// Gather modifications needed to be done to a service.
    pub fn gather(service: &SCNetworkService) -> Option<Self> {
        // grab network interface & interface type -> leave unmodified if missing
        let Some(iface) = service.network_interface() else {
            return None;
        };
        let Some(iface_ty) = iface.interface_type() else {
            return None;
        };

        println!("{iface_ty:?} has been detected");

        // if it's a service for bridge interfaces, delete it
        if let SCNetworkInterfaceType::Bridge = iface_ty {
            return Some(Self::Delete);
        }

        println!("{iface_ty:?} not bridge");

        // check that the interface supports IPv6 protocol -> leave this service unmodified if doesn't
        if iface
            .supported_protocol_type_strings()
            .into_iter()
            .filter_map(|p| SCNetworkProtocolType::from_cfstring(&p))
            .find(|p| matches!(p, SCNetworkProtocolType::IPv6))
            .is_none()
        {
            return None;
        }

        println!("{iface_ty:?} supports ipv6");

        // no protocol modifications needed && service ALREADY enabled  =>  leave this service unmodified
        match (service.enabled(), ProtocolModifications::gather(service)) {
            (true, None) => {
                println!("{iface_ty:?} not modified");
                None
            }
            (enabled, protocol) => {
                println!(
                    "{iface_ty:?} needs to be modified => is_enabled: {enabled}, {protocol:?}"
                );
                Some(ServiceModifications::Modify {
                    enable: !enabled,
                    protocol,
                })
            }
        }
    }
}

/// Modifications needed to be done to the protocols of a service.
#[derive(Debug)]
enum ProtocolModifications {
    AddIPv6,
    ModifyIPv6 {
        /// Whether the protocol needs to be marked as enabled
        enable: bool,
        // TODO: if the configuration keys are WRONG, add information here
    },
}

impl ProtocolModifications {
    /// Gather modifications needed to be done to a service.
    pub fn gather(service: &SCNetworkService) -> Option<Self> {
        // check that the service ALREADY the IPv6 protocol configured => add IPv6 otherwise
        let Some(ipv6_proto) =
            service.find_network_protocol(SCNetworkProtocolType::IPv6.to_cfstring())
        else {
            return Some(Self::AddIPv6);
        };

        // if protocol is disabled => enable it
        // TODO: expand this match-case statement to account for configuration keys!!!
        let ipv6_enabled = ipv6_proto.enabled();
        match (ipv6_enabled,) {
            (false,) => Some(Self::ModifyIPv6 { enable: true }),
            _ => None,
        }
    }
}

pub fn apply_modifications(
    service: &mut SCNetworkService,
    enable_service: bool,
    proto_mods: Option<ProtocolModifications>,
) {
    // enable network if needed
    if enable_service {
        assert!(service.set_enabled(true));
    }

    // apply protocol modifications
    let Some(proto_mods) = proto_mods else {
        return;
    };
    match proto_mods {
        ProtocolModifications::AddIPv6 => {
            assert!(service.add_network_protocol(SCNetworkProtocolType::IPv6.to_cfstring()));
        }
        ProtocolModifications::ModifyIPv6 {
            enable: enable_proto,
        } => {
            let mut ipv6_proto = service
                .find_network_protocol(SCNetworkProtocolType::IPv6.to_cfstring())
                .unwrap();

            // enable protocol if needed
            if enable_proto {
                assert!((&mut ipv6_proto).set_enabled(true));
            }

            // TODO: if the configuration keys are WRONG, modify configuration here
            //       kSCValNetIPv6ConfigMethodAutomatic, which has the value Automatic
            //       kSCValNetIPv6ConfigMethodManual, which has the value Manual
            //       kSCValNetIPv6ConfigMethodRouterAdvertisement, which has the value RouterAdvertisement
            //       kSCValNetIPv6ConfigMethod6to4, which has the value 6to4
        }
    }
}

/// Compute a list of services __in order__ of their priority, if any.
pub fn get_priority_ordered_services(set: &SCNetworkSet) -> Vec<SCNetworkService> {
    let service_priorities = set.service_order().into_collect::<Vec<_>>();
    let mut services = set
        .services()
        .into_iter()
        .map(|s| {
            let service = s.clone();
            let Some(service_id) = s.id() else {
                return (service, None);
            };

            let Some(id) = service_priorities.iter().position(|id| id == &service_id) else {
                return (service, None);
            };

            (service, Some(id))
        })
        .collect::<Vec<_>>();
    services.sort_by(|(s1, p1), (s2, p2)| match (p1, p2) {
        (Some(p1), Some(p2)) => p1.cmp(p2),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });
    services.into_iter().map(|(s, _)| s).collect()
}
