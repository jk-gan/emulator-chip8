use ram::Ram;
use register::Register;

struct Chip8 {
    register: Register,
    ram: Ram,
}
