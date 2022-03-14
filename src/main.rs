extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::{IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

use byteorder::{ByteOrder, BigEndian};

use std::net::IpAddr;

static MAC_INTERFACE: &'static str = "74:d0:2b:93:16:ec";
static SERVER_IP: &'static str = "172.65.252.253";

fn retrieve_network_interface() -> NetworkInterface {
    let interface_names_match = |iface: &NetworkInterface| iface.mac.unwrap().to_string() == MAC_INTERFACE;
    let interfaces = datalink::interfaces();

    interfaces.into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", MAC_INTERFACE))
}

fn handle_packet(packet: &[u8]) {
    let packet = EthernetPacket::new(packet).unwrap();

    if packet.get_ethertype() == EtherTypes::Ipv4 {
        let header = Ipv4Packet::new(packet.payload());

        if let Some(header) = header {
            if header.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                let tcp = TcpPacket::new(header.payload());

                if let Some(tcp) = tcp {
                    if IpAddr::V4(header.get_source()).to_string() == SERVER_IP {
                        println!(
                            "[{}]: TCP Packet received - length: {}",
                            MAC_INTERFACE,
                            tcp.payload().len()
                        );

                        // for byte in header.payload() {
                        //     println!("{:#010b} - {}", byte, byte);
                        // }
                        if tcp.payload().len() == 0 {
                            return
                        }

                        let hiheader = BigEndian::read_u16(&tcp.payload());
                        let packet_id = hiheader >> 2;
                        let len_type = hiheader & 3;

                        println!("{:#018b} - {}", hiheader, hiheader);
                        println!("{:#018b} - {}", packet_id, packet_id);
                        println!("{:#018b} - {}", len_type, len_type);
                    }
                } else {
                    println!("[{}]: Malformed TCP Packet", MAC_INTERFACE);
                }
            }
        } else {
            println!("[{}]: Malformed IPv4 Packet", MAC_INTERFACE);
        }
    }
}

fn main() {
    println!("Starting data extractor.");

    let interface = retrieve_network_interface();

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occured when creating the datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => handle_packet(packet),
            Err(e) => {
                panic!("An error occurred while reading: {}", e)
            }
        }
    }
}
