use crate::network_message::NetworkMessage;
use crate::packet::Packet;
use crate::packets::chat_abstract_server_message::ChatAbstractServerMessage;

pub struct ChatServerMessage {
    pub chat_server_message: ChatAbstractServerMessage,
    pub sender_id: i64,
    pub sender_name: String,
    pub prefix: String,
    pub sender_account_id: i32
}

impl ChatServerMessage {
    pub fn new(packet: &mut Packet) -> Self {
        ChatServerMessage {
            chat_server_message: ChatAbstractServerMessage::new(packet),
            sender_id: packet.read_double(),
            sender_name: packet.read_utf(),
            prefix: packet.read_utf(),
            sender_account_id: packet.read_int()
        }
    }
}

impl NetworkMessage for ChatServerMessage {
    fn display_data(&self) {
        println!("Sender Id: {}", self.sender_id.to_string());
        println!("Sender Name: {}", self.sender_name);
        println!("Prefix: {}", self.prefix);
        println!("Sender account Id: {}", self.sender_account_id.to_string());

        self.chat_server_message.display_data();
    }
}