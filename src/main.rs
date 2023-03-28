use std::{
    io::{Read},
    fs::File, fmt::Debug,
};

#[derive(Debug)] 
struct EmulatorX86 {
    registers: [u32;8],
    _esp: u32,
    _eflags: u32,
    eip: u32,
    memory: Vec<u8>,
}

impl EmulatorX86 {
    fn new(mem_size: usize, esp: u32, eip: u32) -> EmulatorX86 {
        return EmulatorX86 {
            registers: [0;8],
            _esp: esp,
            _eflags: 0,
            eip: eip,
            memory: vec![0; mem_size],
        }
    }
    fn load_binary(&mut self, file_path: &str) {
        let mut file = File::open(&file_path).unwrap();
        file.read(&mut self.memory).unwrap();
    }
    fn print_dump(&self) {
        println!("\n{:?}\n", &self);
    }
    fn get_code<T: Copy>(&self, index:usize) -> T {
        let bytes = std::mem::size_of::<T>();
        let base_point = self.eip as usize + index;
        let code_ptr = &self.memory[base_point..base_point+bytes] as *const [u8] as *const T;
        let code = unsafe { *code_ptr };
        return code;
    }
    fn short_jump(&mut self) {
        println!("run short op");
        let diff = self.get_code::<i8>(1);
        if diff >= 0 {
            self.eip += (diff + 2) as u32;
        } else {
            self.eip -= (diff + 2).abs() as u32;
        }
        
    }
    fn mov(&mut self) {
        println!("run mov op");
        let reg = self.get_code::<u8>(0) - 0xB8;
        let value = self.get_code::<u32>(1);
        self.registers[reg as usize] = value;
        self.eip += 5u32;
    }
    fn execute(&mut self) {
        let code = self.get_code::<u8>(0);
        match code {
            0xB8..=0xC0 => self.mov(),
            0xEB => self.short_jump(),
            _ => panic!("No op")
        }
    }
}

fn main(){
    let mem_size: usize = 512;
    let file_path = "./hoge.bin";
    let mut emulator = EmulatorX86::new(mem_size, 0x0000, 0);
    emulator.load_binary(file_path);
    println!("{}",3u32-2u32);
    loop {
        emulator.execute();
        emulator.print_dump();
        if emulator.eip == 0 { break; }
    }
    
}
