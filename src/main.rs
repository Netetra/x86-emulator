//use std::env;
use std::io::Read;
use std::fs::File;

#[derive(Debug)] 
struct EmulatorX86 {
    _esp: u32,
    _eflags: u32,
    eip: u32,
    memory: Vec<u8>,
}

impl EmulatorX86 {
    fn new(mem_size: usize, esp: u32, eip: u32) -> EmulatorX86 {
        return EmulatorX86 {
            _esp: esp,
            _eflags: 0,
            eip: eip,
            memory: vec![0; mem_size],
        }
    }
    fn status(&self) {
        println!("\n{:?}\n", &self);
    }
    fn get_code8(&self, index:usize) -> &u8 {
        return &self.memory[self.eip as usize + index];
    }
    fn get_sign_code8(&self, index:usize) -> i8 {
        return self.memory[self.eip as usize + index] as i8;
    }
    fn get_code32(&self, index:usize) -> u32 {
        let mut ans:u32 = 0;
        for i in 0..4 {
            ans = ans << 8;
            ans |= *self.get_code8(index+i) as u32;
        }
        //u32がビッグエンディアンだから
        return ans.to_be();
    }
}

fn main(){
    let mem_size: usize = 512;
    let file_path = "./hoge.bin";
    let mut emulator = EmulatorX86::new(mem_size, 0x0000, 0x0000);

    let mut file = match File::open(&file_path) {
        Ok(f) => f ,
        Err(e) => panic!("{}", e)
    };
    file.read(&mut emulator.memory).unwrap();

    loop {
        //let code = emulator.get_code8(0);
        let code = emulator.get_code8(0);
        println!("code: {}",code);
        emulator.status();
        if emulator.eip == 0 { break; }
    }
}
