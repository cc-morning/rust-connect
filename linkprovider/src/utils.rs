use pnet::datalink::NetworkInterface;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub fn select_default_interface(interfaces: &[NetworkInterface]) -> Option<NetworkInterface> {
    let default_interface = interfaces.iter().find(|interface| {
        if interface.mac.is_none() {
            return false;
        }

        if interface.ips.is_empty() || !interface.is_up() || interface.is_loopback() {
            return false;
        }

        let potential_ipv4 = interface.ips.iter().find(|ip| ip.is_ipv4());
        if potential_ipv4.is_none() {
            return false;
        }

        true
    });

    default_interface.cloned()
}

pub fn is_port_free(addr: Ipv4Addr, port: u16) -> bool {
    let ipv4 = SocketAddrV4::new(addr, port);
    TcpListener::bind(ipv4).is_ok()
}

pub fn free_port_in_range(addr: Ipv4Addr, min: u16, max: u16) -> Option<u16> {
    (min..max).find(|port| is_port_free(addr, *port))
}
