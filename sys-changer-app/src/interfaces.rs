use core_foundation::array::CFArray;
use core_foundation::string::CFString;
use system_configuration::network_configuration::SCNetworkInterfaceType;

pub struct Interface {
    pub iface_type: SCNetworkInterfaceType,
    pub bsd_name: CFString,
    pub mac_address: CFString,
    pub supported_interface_types: CFArray<SCNetworkInterfaceType>,
    // pub supported_protocol_types: CFArray<>, // TODO: make
}

pub fn get_interface_info() {}
