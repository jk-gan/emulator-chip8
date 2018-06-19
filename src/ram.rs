struct Ram {
    memory: [u8; 4096],
}

impl Ram {
    fn new() -> Ram {
        Ram { memory: [0; 4096] }
    }

    fn write_byte(&self, address: u16, value: u8) {}

    fn read_byte(&self, address: u16) {}
}
