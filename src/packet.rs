extern crate pnet;

// use byteorder::{ByteOrder, BigEndian};

pub struct Packet {
    pub id: u16,
    pub length: u16
}

impl Packet {
    pub const fn new() -> Self {
        Packet { id: 0, length: 0 }
    }

    pub fn init(&mut self, header: u16) {
        self.id = header >> 2;
        self.length = header & 3;
    }
}