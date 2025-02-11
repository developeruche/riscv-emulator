use interfaces::MemoryInterface;

pub mod interfaces;

/// This is the size of a word in bytes for this vm
pub const WORD_SIZE: usize = 4;
/// This is the maximum memory size for this vm
pub const MAXIMUM_MEMORY_SIZE: u32 = u32::MAX;
/// This is the size of the half word of the VM
const HALF_WORD: usize = 2;
/// This is the size of a byte in the VM
const BYTE: usize = 1;

/// This defines the different chuck of memory that can be read or written to
pub enum MemoryChuckSize {
    BYTE,
    HALF_WORD,
    WORD_SIZE,
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub memory: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct Registers {
    data: [u32; 32],
}

impl MemoryInterface for Memory {
    fn read_mem(&self, addr: u32, size: MemoryChuckSize) -> u32 {
        unimplemented!()
    }

    fn write_word(&mut self, addr: u32, size: MemoryChuckSize, value: u32) {
        unimplemented!()
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers { data: [0; 32] }
    }

    pub fn read_reg(&self, reg: u32) -> u32 {
        self.data[reg as usize]
    }

    pub fn write_reg(&mut self, reg: u32, value: u32) {
        if reg == 0 {
            return;
        }

        self.data[reg as usize] = value;
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: vec![0; (MAXIMUM_MEMORY_SIZE / 4) as usize],
        }
    }

    pub fn load_program(&mut self, program: &Vec<u32>, base_addr: u32) {
        let mut addr = base_addr as usize;

        for byte in program {
            self.memory[addr] = *byte;
            addr += 1;
        }
    }

    pub fn new_with_load_program(program: &Vec<u32>, base_addr: u32) -> Self {
        let mut memory = Memory::new();
        memory.load_program(program, base_addr);

        memory
    }
}
