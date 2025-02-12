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
        let mut file = BufReader::new(File::open(path)?);
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
            crate::instructions::DecodedInstruction::RType(rtype) => {
                match rtype.funct3 {
                    0b000 => {
                        // Funct3 for add, sub, mul
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for add
                                todo!()
                            }
                            0b0100000 => {
                                // Funct7 for sub
                                todo!()
                            }
                            0b0000001 => {
                                // Funct7 for mul
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b001 => {
                        // Funct3 for sll, mulh
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for sll
                                todo!()
                            }
                            0b0000001 => {
                                // Funct7 for mulh
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b010 => {
                        // Funct3 for slt, mulhsu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for slt
                                todo!()
                            }
                            0b0000001 => {
                                // Funct7 for mulhsu
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b011 => {
                        // Funct3 for sltu, mulhu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for sltu
                                todo!()
                            }
                            0b0000001 => {
                                // Funct7 for mulhu
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b100 => {
                        // Funct3 for xor, div
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for xor
                                todo!()
                            }
                            0b0000001 => {
                                // Funct7 for div
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b101 => {
                        // Funct3 for srl, sra, divu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for srl
                                todo!()
                            }
                            0b0000001 => {
                                // Funct7 for divu
                                todo!()
                            }
                            0b0100000 => {
                                // Funct7 for sra
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b110 => {
                        // Funct3 for or, rem
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for or
                                todo!()
                            }
                            0b0000001 => {
                                // Funct7 for rem
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b111 => {
                        // Funct3 for and, remu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for and
                                todo!()
                            }
                            0b0000001 => {
                                // Funct7 for remu
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }

                    _ => return Err(VMErrors::InvalidOpcode),
                }
            }
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
