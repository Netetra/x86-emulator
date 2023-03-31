use std::u8;

use super::EmulatorX86;

impl EmulatorX86 {
    pub(super) fn short_jmp(&mut self) {
        let diff = self.read_memory::<i8>(self.eip as usize + 1);
        if diff >= 0 {
            self.eip += (diff + 2) as u32;
        }
        else {
            self.eip -= (diff + 2).abs() as u32;
        }
        
    }


    pub(super) fn near_jmp(&mut self) {
        let diff = self.read_memory::<i32>(self.eip as usize + 1);
        if diff >= 0 {
            self.eip += (diff + 5) as u32;
        }
        else {
            self.eip -= (diff + 5).abs() as u32;
        }
    }


    pub(super) fn mov_r32_imm32(&mut self) {
        let reg = self.read_memory::<u8>(self.eip as usize) - 0xB8;
        let value = self.read_memory::<u32>(self.eip as usize + 1);
        self.registers[reg as usize] = value;
        self.eip += 5;
    }


    pub(super) fn mov_rm32_imm32(&mut self) {
        self.eip += 1;
        let modrm = self.get_modrm();
        let sib = self.get_sib(&modrm);
        let disp = self.get_disp(&modrm);
        let value = self.read_memory::<u32>(self.eip as usize);
        self.eip += 4;
        self.set_rm32(&modrm, sib, disp, value);
    }


    pub(super) fn mov_rm32_r32(&mut self){
        self.eip += 1;
        let modrm = self.get_modrm();
        let sib = self.get_sib(&modrm);
        let disp = self.get_disp(&modrm);
        let r32 = self.get_register32(modrm.reg as usize);
        self.set_rm32(&modrm, sib, disp, r32);
    }


    pub(super) fn mov_r32_rm32(&mut self){
        self.eip += 1;
        let modrm = self.get_modrm();
        let sib = self.get_sib(&modrm);
        let disp = self.get_disp(&modrm);
        let rm32 = self.get_rm32(&modrm, sib, disp);
        self.set_register32(modrm.reg as usize, rm32)
    }
}
