
mod emulator;
use crate::emulator::EmulatorX86;

fn main(){
    let mem_size: usize = 31800;
    let file_path = "./hoge.bin";
    let mut emulator = EmulatorX86::new(mem_size, 0x7C00, 0x7C00);
    emulator.load_binary(file_path);
    emulator.run();
    emulator.print_dump();
    emulator.print_memory();
}
