mod core;
mod cpu_op;
mod memory;
mod register;
mod modrm;

pub struct EmulatorX86 {
    registers: [u32;8],
    esp: u32,
    eflags: u32,
    eip: u32,
    memory: Vec<u8>,
}

struct ModRM {
    mode: u8,
    reg: u8,
    rm: u8,
}

enum Disp {
    Disp8(u8),
    Disp32(u32),
}
