use crate::ext::CFArrayExt;
use system_configuration::network_configuration::{SCNetworkService, SCNetworkSet};
use system_configuration::preferences::SCPreferences;

// pub fn into_priorities()

pub fn remove_bridge_services(prefs: &SCPreferences, set: &SCNetworkSet) {
    let mut ifaces = set.services().into_collect::<Vec<_>>();
    let iface_priorities = set.service_order().into_collect::<Vec<_>>();

    // for all priorities, look up corresponding interfaces
    let mut new_ifaces = Vec::<SCNetworkService>::with_capacity(iface_priorities.len());
    for iface_id in iface_priorities {
        // remove the interface reference if found
        let Some(i) = ifaces
            .iter()
            .position(|s| s.id().map_or(false, |id| id == iface_id))
        else {
            continue;
        };
        let iface = ifaces.remove(i);

        // if the interface is a bridge interface,
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
