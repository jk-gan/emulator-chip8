/*

Memory Map:
+---------------+= 0xFFF (4095) End of Chip-8 RAM
|               |
|               |
|               |
|               |
|               |
| 0x200 to 0xFFF|
|     Chip-8    |
| Program / Data|
|     Space     |
|               |
|               |
|               |
+- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
|               |
|               |
|               |
+---------------+= 0x200 (512) Start of most Chip-8 programs
| 0x000 to 0x1FF|
| Reserved for  |
|  interpreter  |
+---------------+= 0x000 (0) Start of Chip-8 RAM

*/

const PROGRAM_START_ADDRESS: u16 = 0x200;

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
