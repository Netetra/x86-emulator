use std::{ io::Read, fs::File };
use super::EmulatorX86;

impl EmulatorX86 {
    pub fn new(mem_size: usize, esp: u32, eip: u32) -> EmulatorX86 {
        return EmulatorX86 {
            registers: [0;8],
            esp: esp,
            eflags: 0,
            eip: eip,
            memory: vec![0; mem_size],
        }
    }


    pub fn load_binary(&mut self, file_path: &str) {
        let mut file = File::open(&file_path).unwrap();
        file.read(&mut self.memory[self.eip as usize..]).unwrap();
    }


    pub fn print_dump(&self) {
        println!("Registers = {:?}", &self.registers);
        println!("ESP       = {}", &self.esp);
        println!("EFLAGS    = {}", &self.eflags);
        println!("EIP       = {}", &self.eip);
        println!("");
    }


    pub fn print_memory(&self) {
        println!("{:?}", self.memory);
    }


    pub fn run(&mut self){
        loop {
            let code = self.read_memory::<u8>(self.eip as usize);
            match code {
                0x89 => self.mov_rm32_r32(),
                0x8B => self.mov_r32_rm32(),
                0xB8..=0xC0 => self.mov_r32_imm32(),
                0xC7 => self.mov_rm32_imm32(),
                0xE9 => self.near_jmp(),
                0xEB => self.short_jmp(),
                _ => break
            };
            if self.eip == 0 { break; }
        }
    }
}