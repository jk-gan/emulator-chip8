struct Register {
    // general purpose 8-bit registers
    vx: [u8; 16],
    // index register
    i: u16,
    // program counter
    pc: u16,
    // stack pointer
    sp: u8,
    // stack
    stack: [u16; 16],
}
