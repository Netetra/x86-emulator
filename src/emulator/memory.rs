use super::{EmulatorX86, ModRM, Disp};

impl EmulatorX86 {
    pub(super) fn read_memory<T: Copy>(&self, address:usize) -> T {
        let bytes = std::mem::size_of::<T>();
        let raw_pointer = &self.memory[address..address+bytes] as *const [u8] as *const T;
        return unsafe { *raw_pointer };
    }


    pub(super) fn write_memory<T>(&mut self, address: usize, value: T) {
        let bytes = std::mem::size_of::<T>();
        let value_ptr = &value as *const T as *const u8;
        let value_bytes = unsafe { core::slice::from_raw_parts(value_ptr, bytes) };
        for i in 0..bytes {
            self.memory[address+i] = value_bytes[i];
        }
    }
}