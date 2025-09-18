use system_configuration::network_configuration::{SCNetworkService, SCNetworkSet};
use system_configuration::preferences::SCPreferences;

pub fn remove_bridge_services(prefs: &SCPreferences, set: &SCNetworkSet) {
    let ifaces = set
        .services()
        .into_iter()
        .map(|i| i.clone())
        .collect::<Vec<_>>();
    let iface_priorities = set
        .service_order()
        .into_iter()
        .map(|i| i.clone())
        .collect::<Vec<_>>();
    let mut new_ifaces = Vec::<SCNetworkService>::with_capacity(iface_priorities.len());
}
