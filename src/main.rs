extern crate pnet;

mod network_message;
mod packet;
mod packets {
    pub mod chat_abstract_server_message;
    pub mod chat_server_message;
    pub mod kamas_update_message;
}

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ip::{IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;

use std::net::IpAddr;

use crate::packets::chat_server_message::ChatServerMessage;
use crate::packets::kamas_update_message::KamasUpdateMessage;
use crate::network_message::NetworkMessage;

static MAC_INTERFACE: &'static str = "74:d0:2b:93:16:ec";
static SERVER_IP: &'static str = "172.65.252.253";

fn retrieve_network_interface() -> NetworkInterface {
    let interface_names_match = |interface: &NetworkInterface| interface.mac.unwrap().to_string() == MAC_INTERFACE;
    let interfaces = datalink::interfaces();

    interfaces.into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", MAC_INTERFACE))
}

fn handle_packet(buffer: &mut Vec<u8>, packet: &[u8]) {
    let packet = EthernetPacket::new(packet).unwrap();

    if packet.get_ethertype() == EtherTypes::Ipv4 {
        let header = Ipv4Packet::new(packet.payload());

        if let Some(header) = header {
            if header.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
                let tcp = TcpPacket::new(header.payload());

                if let Some(tcp) = tcp {
                    if IpAddr::V4(header.get_source()).to_string() == SERVER_IP && tcp.payload().len() > 0 {
                        println!(
                            "[{}]: TCP Packet received - length: {}",
                            MAC_INTERFACE,
                            tcp.payload().len()
                        );

                        for byte in tcp.payload() {
                            buffer.push(*byte);
                        }

                        let mut keep_going = true;

                        while keep_going {
                            if buffer.len() > 2 {
                                let mut packet = packet::Packet::new();

                                packet.init(buffer);

                                if buffer.len() >= packet.length as usize {
                                    packet.read(buffer);

                                    // TODO packets to implement
                                    // 7116
                                    // 4739
                                    // 6875
                                    // 1678

                                    match Some(packet.id) {
                                        Some(373) => {
                                            ChatServerMessage::new(&mut packet);

                                            packet
                                        },
                                        Some(3977) => {
                                            let test = KamasUpdateMessage::new(&mut packet);

                                            test.display_data();

                                            packet
                                        },
                                        Some(_) => {
                                            packet.print_info();

                                            packet
                                        },
                                        None => {
                                            println!("The impossible has happened.");

                                            packet
                                        }
                                    };
                                }
                            } else {
                                keep_going = false;
                            }
                        }


                        println!("Buffer length = {}", buffer.len());
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
    let buffer: &mut Vec<u8> = &mut Vec::new();

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occured when creating the datalink channel: {}", e)
    };

    loop {
        match rx.next() {
            Ok(packet) => handle_packet(buffer, packet),
            Err(e) => {
                panic!("An error occurred while reading: {}", e)
            }
        }
    }
}
