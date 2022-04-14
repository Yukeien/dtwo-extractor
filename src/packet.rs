extern crate pnet;

pub struct Packet {
    pub id: u16,
    pub length_type: u8,
    pub length: u32,
    pub payload: Vec<u8>
}

impl Packet {
    pub const fn new() -> Self {
        Packet { id: 0, length_type: 0, length: 0, payload: Vec::new() }
    }

    pub fn init(&mut self, buffer: &mut Vec<u8>) {
        let header = (buffer[0] as u16) << 8 | (buffer[1] as u16);

        self.id = header >> 2 as i32;
        self.length_type = (header & 3) as u8;

        let mut i = 0;

        while i < self.length_type {
            self.length = (self.length << 8) + buffer[(i + 2) as usize] as u32;
            i += 1;
        }
    }

    pub fn read(&mut self, buffer: &mut Vec<u8>) {
        let clear = self.length_type + 2;
        let mut i = 0;
        let mut j = 0;

        while i < clear {
            buffer.remove(0);
            i += 1;
        }

        while j < self.length {
            self.payload.push(buffer.remove(0));
            j += 1;
        }
    }

    pub fn read_double(&mut self) -> i64 {
        let mut value: i64 = self.payload.remove(0) as i64;
        let mut i = 0;

        while i < 7 && self.payload.len() > 0 {
            value = value << 8 | self.payload.remove(0) as i64;
            i += 1
        }

        value
    }

    pub fn read_int(&mut self) -> i32 {
        let mut value: i32 = self.payload.remove(0) as i32;
        let mut i = 0;

        while i < 3 && self.payload.len() > 0 {
            value = value << 8 | self.payload.remove(0) as i32;
            i += 1
        }

        value
    }

    pub fn read_byte(&mut self) -> i8 {
        self.payload.remove(0) as i8
    }

    pub fn read_utf(&mut self) -> String {
        let length: u16 = (self.payload.remove(0) as u16) << 8 | self.payload.remove(0) as u16;
        let mut str: String = String::with_capacity(length as usize);
        let mut i = 0;

        while i < length && self.payload.len() > 0 {
            str.push(self.payload.remove(0) as char);
            i += 1;
        }

        str
    }

    pub fn print_info(& mut self) {
        println!("Packet Id - {}", self.id.to_string());

        if self.id == 373 {
            // ChatAbstractClientMessage
            // println!("Content Func: {}", self.read_utf());

            // ChatAbstractServerMessage
            println!("Channel: {}", self.read_byte().to_string());
            println!("Content: {}", self.read_utf());
            println!("Timestamp: {}", self.read_int().to_string());
            println!("Fingerprint: {}", self.read_utf());

            // ChatServerMessage
            println!("Sender Id: {}", self.read_double().to_string());
            println!("Sender Name: {}", self.read_utf());
            println!("Prefix: {}", self.read_utf());
            println!("Sender account Id: {}", self.read_int().to_string());
        }
    }
}
