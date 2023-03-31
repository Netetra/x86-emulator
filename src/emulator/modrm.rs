use super::{EmulatorX86, ModRM, Disp};

impl EmulatorX86 {
    pub(super) fn get_modrm(&mut self) -> ModRM {
        let modrm_byte = self.read_memory(self.eip as usize + 1);
        let modrm = self.parse_modrm(modrm_byte);
        self.eip += 1;
        return modrm;
    }


    pub(super) fn parse_modrm(&self, modrm_byte: u8) -> ModRM {
        return ModRM {
            mode: modrm_byte >> 6,
            reg: (modrm_byte << 2) >> 5,
            rm: (modrm_byte << 5) >> 5,
        };
    }


    pub(super) fn get_sib(&mut self, modrm: &ModRM) -> Option<u8> {    
        if modrm.mode != 3 && modrm.rm == 4 {
            self.eip += 1;
            let sib = self.read_memory(self.eip as usize + 0);
            return Some(sib);
        }
        return None;
    }


    pub(super) fn get_disp(&mut self, modrm: &ModRM) -> Option<Disp> {
        if (modrm.mode == 0 && modrm.rm == 5) || modrm.mode == 2 {
            let value = self.read_memory::<u32>(self.eip as usize + 0);
            self.eip += 4;
            return Some(Disp::Disp32(value))
        }
        else if modrm.mode == 1 {
            let value = self.read_memory::<u8>(self.eip as usize + 0);
            self.eip += 1;
            return Some(Disp::Disp8(value))
        }
        else {
            return None;
        }
    }


    pub(super) fn set_rm32(&mut self, modrm: &ModRM, sib: Option<u8>, disp: Option<Disp>, value: u32) {
        if modrm.mode == 3 {
            self.set_register32(modrm.rm as usize, value)
        }
        else {
            let address = self.calc_memory_address(&modrm, sib, disp);
            self.write_memory::<u32>(address, value)
        }
    }


    pub(super) fn get_rm32(&mut self, modrm: &ModRM, sib: Option<u8>, disp: Option<Disp>) -> u32 {
        if modrm.mode == 3 {
            self.get_register32(modrm.rm as usize)
        }
        else {
            let address = self.calc_memory_address(&modrm, sib, disp);
            self.read_memory::<u32>(address)
        }
    }


    fn calc_memory_address(&self, modrm: &ModRM, sib: Option<u8>, disp: Option<Disp>) -> usize {
        if modrm.rm == 4 {
            panic!("not Implemented ModRM mod = {} rm = 4",modrm.mode);
        }
        if modrm.mode == 0 {
            if modrm.rm == 5 {
                if let Some(Disp::Disp32(disp32)) = disp{
                    return disp32 as usize;
                }
                else {
                    panic!("Option = None or Disp::Disp8");
                }
            }
            else {
                return self.get_register32(modrm.rm as usize) as usize;
            }
        }
        else if modrm.mode == 1 {
            if let Some(Disp::Disp8(disp8)) = disp{
                return self.get_register32(modrm.rm as usize) as usize + disp8 as usize;
            }
            else {
                panic!("Option = None or Disp::Disp8");
            }
        }
        else if modrm.mode == 2 {
            if let Some(Disp::Disp32(disp32)) = disp{
                return self.get_register32(modrm.rm as usize) as usize + disp32 as usize;
            }
            else {
                panic!("Option = None or Disp::Disp8");
            }
        }
        else {
            panic!("ModRM mode = 3");
        }
    }
}