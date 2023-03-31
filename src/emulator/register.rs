use super::EmulatorX86;

impl EmulatorX86 {
    #[allow(dead_code)]
    fn get_register8(&self, index: usize) {
        /*if (index < 4) {
            return self.registers[index] & 0xff;
        } else {
            return (self.registers[index - 4] >> 8) & 0xff;
        }*/
    }


    #[allow(dead_code)]
    fn set_register8(&mut self, index: usize, value: u8) {
        /*if (index < 4) {
            let r = self.registers[index] & 0xffffff00;
            self.registers[index] = r | value as u32;
        } else {
            let r = self.registers[index - 4] & 0xffff00ff;
            self.registers[index - 4] = r | (value as u32 << 8);
        }*/
    }


    pub(super) fn get_register32(&self, index: usize) -> u32 {
        return self.registers[index];
    }


    pub(super) fn set_register32(&mut self, index: usize, value: u32) {
        self.registers[index] = value;
    }
}