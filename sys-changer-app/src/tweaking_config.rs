use crate::ext::CFArrayExt;
use system_configuration::network_configuration::{
    SCNetworkInterfaceType, SCNetworkProtocolType, SCNetworkService, SCNetworkSet,
};
use system_configuration::preferences::SCPreferences;

// pub fn into_priorities()

pub fn remove_bridge_services(prefs: &SCPreferences, set: &SCNetworkSet) {
    let ordered_services = get_priority_ordered_services(set);

    // remove bridge interface
    let ordered_services = ordered_services
        .into_iter()
        .filter_map(|s| {
            // grab interface & interface type -> leave unmodified if missing
            let Some(iface) = s.network_interface() else {
                return Some(s);
            };
            let Some(iface_ty) = iface.interface_type() else {
                return Some(s);
            };

            // if it's a service for bridge interfaces, remove it
            if let SCNetworkInterfaceType::Bridge = iface_ty {
                return None;
            }

            // check that the interface supports IPv6 protocol -> leave modified if not
            if iface
                .supported_protocol_type_strings()
                .into_iter()
                .filter_map(|p| SCNetworkProtocolType::from_cfstring(&p))
                .find(|p| matches!(p, SCNetworkProtocolType::IPv6))
                .is_none()
            {
                return Some(s);
            }

            // check that the service HAS

            // matches!(iface_ty, SCNetworkInterfaceType::Bridge)
            None
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
