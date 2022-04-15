use crate::network_message::NetworkMessage;
use crate::packet::Packet;

pub struct KamasUpdateMessage {
    pub kamas_total: i64
}

impl KamasUpdateMessage {
    pub fn new(packet: &mut Packet) -> Self {
        KamasUpdateMessage {
            kamas_total: packet.read_var_ulong()
        }
    }
}

impl NetworkMessage for KamasUpdateMessage {
    fn display_data(&self) {
        println!("Channel: {}", self.kamas_total.to_string());
    }
}