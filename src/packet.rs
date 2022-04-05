extern crate pnet;

use std::ascii::escape_default;
use std::str;

// use byteorder::{ByteOrder, BigEndian};

pub struct Packet {
    pub id: u16,
    pub length: u32,
    pub payload: Vec<u8>
}

impl Packet {
    pub const fn new() -> Self {
        Packet { id: 0, length: 0, payload: Vec::new() }
    }

    pub fn init(&mut self, buffer: &mut Vec<u8>) {
        let mut header = buffer.remove(0) as u16;

        header = (header << 8) + buffer.remove(0) as u16;

        println!("{}", header);
        println!("{:#016b}", header);

        self.id = header >> 2;

        println!("{:#016b}", header >> 2);

        let length_type = header & 3;

        println!("{:#016b}", header & 3);

        let mut i = 0;
        let mut j = 0;

        while i < length_type {
            self.length = (self.length << 8) + buffer.remove(0) as u32;
            i += 1;
        }

        while j < self.length {
            self.payload.push(buffer.remove(0));
            j += 1;
        }
    }

    pub fn print_info(self) {
        println!("Packet Id - {}", self.id.to_string());
        println!("Packet Length - {}", self.length);
        println!("Payload size - {}", self.payload.len());

        let mut string = String::new();

        for byte in self.payload {
            string.push(byte as char);
        }
        println!("Payload:\n{}", string);
    }
}
