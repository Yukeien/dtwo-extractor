use crate::network_message::NetworkMessage;
use crate::packet::Packet;

pub struct ChatAbstractServerMessage {
    pub channel: i8,
    pub content: String,
    pub timestamp: i32,
    pub fingerprint: String
}

impl ChatAbstractServerMessage {
    pub fn new(packet: &mut Packet) -> Self {
        ChatAbstractServerMessage {
            channel: packet.read_byte(),
            content: packet.read_utf(),
            timestamp: packet.read_int(),
            fingerprint: packet.read_utf()
        }
    }
}

impl NetworkMessage for ChatAbstractServerMessage {
    fn display_data(&self) {
        println!("Channel: {}", self.channel.to_string());
        println!("Content: {}", self.content);
        println!("Timestamp: {}", self.timestamp.to_string());
        println!("Fingerprint: {}", self.fingerprint);
    }
}