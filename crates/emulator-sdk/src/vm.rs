//! This mod holds all the necessary structs and functions to emulate a RISC-V CPU.
use crate::instructions::InstructionDecoder;
use core::{interfaces::MemoryInterface, Memory, MemoryChuckSize, Registers};
use elf_parser::Elf;
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug, Clone)]
pub enum VMErrors {
    InvalidInstruction,
    InvalidMemoryAccess,
    EnvironmentError,
    InvalidOpcode,
}

#[derive(Debug, Clone)]
pub struct Vm {
    pub registers: Registers,
    pub memory: Memory,
    pub pc: u32,
    pub running: bool,
    pub exit_code: u32,
}

impl Vm {
    /// Create a new Vm.
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            pc: 0,
            running: false,
            exit_code: 0,
        }
    }

    /// Create a new Vm from a binary ELF file.
    /// # Errors
    /// This function may return an error if the ELF is not valid.
    pub fn from_bin_elf(path: String) -> Result<Self, anyhow::Error> {
        let mut file = BufReader::new(File::open(path).unwrap());
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();

        let program_elf_decoded = Elf::decode(&buf)?;

        Ok(Self {
            registers: Registers::new(),
            memory: Memory::new_with_load_program(
                &program_elf_decoded.instructions,
                program_elf_decoded.pc_base,
            ),
            pc: program_elf_decoded.pc_start,
            running: false,
            exit_code: 0,
        })
    }

    /// Step the Vm.
    /// This function will execute the instruction at the current program counter.
    /// If the instruction is a branch, the program counter will be updated accordingly.
    /// If the instruction is a jump, the program counter will be updated accordingly.
    /// If the instruction is a syscall, the program will be halted.
    /// If the instruction is a halt, the program will be halted.
    pub fn step(&mut self) -> Result<bool, VMErrors> {
        // Fetch the instruction from memory
        let instruction = self
            .memory
            .read_mem(self.pc, MemoryChuckSize::WORD_SIZE)
            .ok_or(VMErrors::InvalidMemoryAccess)?;

        // Decode the instruction
        let decoded_instruction = InstructionDecoder::decode(&instruction)?;

        // Execute the instruction
        match decoded_instruction.decoded_instruction {
            crate::instructions::DecodedInstruction::RType(rtype) => {}
            crate::instructions::DecodedInstruction::IType(itype) => {}
            crate::instructions::DecodedInstruction::SType(stype) => {}
            crate::instructions::DecodedInstruction::BType(btype) => {}
            crate::instructions::DecodedInstruction::UType(utype) => {}
            crate::instructions::DecodedInstruction::JType(jtype) => {}
        }

        Ok(true)
    }

    /// Run the Vm.
    /// This function will run the Vm until it halts.
    /// The Vm will halt if the program counter is out of bounds or if the instruction is a halt.
    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            match self.step() {
                Ok(true) => continue,
                Ok(false) => break,
                Err(e) => {
                    match e {
                        VMErrors::EnvironmentError => {} // would just be halting the program, sysytem calls are not allowed on the VM
                        _ => {
                            eprintln!("Error at pc: {:x}", self.pc);
                        }
                    }
                    self.running = false;
                }
            }
        }
    }
}
