use crate::ext::CFArrayExt;
use system_configuration::network_configuration::{
    SCNetworkInterfaceType, SCNetworkProtocolType, SCNetworkService, SCNetworkSet,
};
use system_configuration::preferences::SCPreferences;

/// Modifications needed to be done to a service.
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

        // if it's a service for bridge interfaces, delete it
        if let SCNetworkInterfaceType::Bridge = iface_ty {
            return Some(Self::Delete);
        }

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

        // no protocol modifications needed && service ALREADY enabled  =>  leave this service unmodified
        match (service.enabled(), ProtocolModifications::gather(service)) {
            (true, None) => None,
            (enabled, protocol) => Some(ServiceModifications::Modify {
                enable: !enabled,
                protocol,
            }),
        }
    }
}

/// Modifications needed to be done to the protocols of a service.
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

pub fn remove_bridge_services(prefs: &SCPreferences, set: &SCNetworkSet) {
    let ordered_services = get_priority_ordered_services(set);

    // remove bridge interface
    let ordered_services = ordered_services
        .into_iter()
        .filter_map(|s| match ServiceModifications::gather(&s) {
            None => Some(s),
            Some(ServiceModifications::Delete) => None,
            Some(ServiceModifications::Modify {
                enable,
                protocol: proto_mods,
            }) => {
                todo!()
            }
        })
        .collect::<Vec<_>>();
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
