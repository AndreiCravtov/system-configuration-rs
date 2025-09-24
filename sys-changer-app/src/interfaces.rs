use mac_address::MacAddress;
use std::str::FromStr;
use system_configuration::network_configuration::{SCNetworkInterface, SCNetworkInterfaceMTU, SCNetworkInterfaceType, SCNetworkProtocolType};

#[derive(Debug)]
pub struct Interface {
    pub iface_type: SCNetworkInterfaceType,
    pub bsd_name: String,
    pub mac_addr: MacAddress,
    pub supported_iface_types: Vec<SCNetworkInterfaceType>,
    pub supported_proto_types: Vec<SCNetworkProtocolType>,
    pub underlying_iface: Option<Box<Interface>>,
    pub mtu_opts: Option<SCNetworkInterfaceMTU>
}

impl Interface {
    pub fn from_scnetwork_interface(scnetwork_interface: &SCNetworkInterface) -> Option<Self> {
        let iface_type = scnetwork_interface.interface_type()?;
        let bsd_name = scnetwork_interface.bsd_name()?.to_string();
        let hardware_address_string = scnetwork_interface.hardware_address_string()?.to_string();
        let mac_addr = MacAddress::from_str(&hardware_address_string).ok()?;
        let supported_iface_types = scnetwork_interface
            .supported_interface_type_strings()
            .into_iter()
            .filter_map(|i| SCNetworkInterfaceType::from_cfstring(&i))
            .collect();
        let supported_proto_types = scnetwork_interface
            .supported_protocol_type_strings()
            .into_iter()
            .filter_map(|i| SCNetworkProtocolType::from_cfstring(&i))
            .collect();
        let underlying_iface = scnetwork_interface
            .underlying_interface()
            .and_then(|i| Self::from_scnetwork_interface(&i))
            .map(Box::new);
        let mtu_opts = scnetwork_interface.mtu();

        Some(Interface {
            iface_type,
            bsd_name,
            mac_addr,
            supported_iface_types,
            supported_proto_types,
            underlying_iface,
            mtu_opts
        })
    }
}

pub fn get_interfaces() -> Vec<Interface> {
    SCNetworkInterface::get_interfaces()
        .into_iter()
        .filter_map(|i| Interface::from_scnetwork_interface(&i))
        .collect::<Vec<_>>()
}

/// This is undocumented Apple API
pub fn remove_virtual_network_interfaces_bridge() {}
