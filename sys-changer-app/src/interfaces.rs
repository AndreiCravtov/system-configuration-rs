use mac_address::MacAddress;
use std::str::FromStr;
use system_configuration::network_configuration::{
    SCNetworkInterface, SCNetworkInterfaceType, SCNetworkProtocolType,
};

#[derive(Debug)]
pub struct Interface {
    pub iface_type: SCNetworkInterfaceType,
    pub bsd_name: String,
    pub mac_addr: MacAddress,
    pub supported_iface_types: Vec<SCNetworkInterfaceType>,
    pub supported_proto_types: Vec<SCNetworkProtocolType>,
}

pub fn get_interfaces() -> Vec<Interface> {
    SCNetworkInterface::get_interfaces()
        .into_iter()
        .filter_map(|iface| {
            println!("running");
            let iface_type = iface.interface_type()?;
            let bsd_name = iface.bsd_name()?.to_string();
            let hardware_address_string = iface.hardware_address_string()?.to_string();
            let mac_addr = MacAddress::from_str(&hardware_address_string).ok()?;
            let supported_iface_types = iface
                .supported_interface_type_strings()
                .into_iter()
                .filter_map(|i| SCNetworkInterfaceType::from_cfstring(&i))
                .collect();
            let supported_proto_types = iface
                .supported_interface_type_strings()
                .into_iter()
                .filter_map(|i| SCNetworkProtocolType::from_cfstring(&i))
                .collect();

            Some(Interface {
                iface_type,
                bsd_name,
                mac_addr,
                supported_iface_types,
                supported_proto_types,
            })
        })
        .collect::<Vec<_>>()
}
