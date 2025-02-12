//! This mod holds all the necessary structs and functions to emulate a RISC-V CPU.
use crate::instructions::InstructionDecoder;
use core::{interfaces::MemoryInterface, sign_extend_u32, Memory, MemoryChuckSize, Registers};
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

        println!("Decoded Inst: {:?}", decoded_instruction);

        // Execute the instruction
        match decoded_instruction.decoded_instruction {
            crate::instructions::DecodedInstruction::RType(rtype) => {
                match rtype.funct3 {
                    0b000 => {
                        // Funct3 for add, sub, mul
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for add
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_add(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0100000 => {
                                // Funct7 for sub
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_sub(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for mul
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_mul(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b001 => {
                        // Funct3 for sll, mulh
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for sll
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_shl(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for mulh
                                let rs1 =
                                    sign_extend_u32(self.registers.read_reg(rtype.rs1 as u32));
                                let rs2 =
                                    sign_extend_u32(self.registers.read_reg(rtype.rs2 as u32));
                                let rd = (rs1.wrapping_mul(rs2) >> 32) as u32;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b010 => {
                        // Funct3 for slt, mulhsu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for slt
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as i32;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as i32;
                                let rd = if rs1 < rs2 { 1 } else { 0 };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for mulhsu
                                let rs1 =
                                    sign_extend_u32(self.registers.read_reg(rtype.rs1 as u32));
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as i64;
                                let rd = (rs1.wrapping_mul(rs2) >> 32) as u32;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b011 => {
                        // Funct3 for sltu, mulhu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for sltu
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = if rs1 < rs2 { 1 } else { 0 };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for mulhu
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as u64;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as u64;
                                let rd = (rs1.wrapping_mul(rs2) >> 32) as u32;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b100 => {
                        // Funct3 for xor, div
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for xor
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1 ^ rs2;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for div
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as i32;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as i32;
                                let rd = if rs2 != 0 {
                                    rs1.wrapping_div(rs2) as u32
                                } else {
                                    u32::MAX
                                };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b101 => {
                        // Funct3 for srl, sra, divu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for srl
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1.wrapping_shr(rs2);
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for divu
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = if rs2 != 0 { rs1 / rs2 } else { u32::MAX };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0100000 => {
                                // Funct7 for sra
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as i32;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as u32;
                                let rd = rs1.wrapping_shr(rs2) as u32;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b110 => {
                        // Funct3 for or, rem
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for or
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1 | rs2;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for rem
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32) as i32;
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32) as i32;
                                let rd = if rs2 != 0 {
                                    rs1.wrapping_rem(rs2) as u32
                                } else {
                                    rs1 as u32
                                };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b111 => {
                        // Funct3 for and, remu
                        match rtype.funct7 {
                            0b0000000 => {
                                // Funct7 for and
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = rs1 & rs2;
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            0b0000001 => {
                                // Funct7 for remu
                                let rs1 = self.registers.read_reg(rtype.rs1 as u32);
                                let rs2 = self.registers.read_reg(rtype.rs2 as u32);
                                let rd = if rs2 != 0 { rs1 % rs2 } else { rs1 };
                                self.registers.write_reg(rtype.rd as u32, rd);
                                self.pc += 4;
                                Ok(true)
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }

                    _ => return Err(VMErrors::InvalidOpcode),
                }
            }
            crate::instructions::DecodedInstruction::IType(itype) => {
                match decoded_instruction.opcode {
                    0b0010011 => {
                        // Funct3 for addi, slti, sltiu, xori, ori, andi
                        match itype.funct3 {
                            0b000 => {
                                // Funct3 for addi
                                todo!()
                            }
                            0b010 => {
                                // Funct3 for slti
                                todo!()
                            }
                            0b011 => {
                                // Funct3 for sltiu
                                todo!()
                            }
                            0b100 => {
                                // Funct3 for xori
                                todo!()
                            }
                            0b101 => {
                                // Funct3 for srli, srai
                                todo!()
                            }
                            0b110 => {
                                // Funct3 for ori
                                todo!()
                            }
                            0b111 => {
                                // Funct3 for andi
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b0000011 => {
                        // Funct3 for lb, lh, lw, lbu, lhu
                        match itype.funct3 {
                            0b000 => {
                                // Funct3 for lb
                                todo!()
                            }
                            0b001 => {
                                // Funct3 for lh
                                todo!()
                            }
                            0b010 => {
                                // Funct3 for lw
                                todo!()
                            }
                            0b100 => {
                                // Funct3 for lbu
                                todo!()
                            }
                            0b101 => {
                                // Funct3 for lhu
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    0b1100111 => {
                        // Funct3 for jalr
                        match itype.funct3 {
                            0b000 => {
                                // Funct3 for jalr
                                todo!()
                            }
                            _ => return Err(VMErrors::InvalidOpcode),
                        }
                    }
                    // not handling enviroment calls because it is halted during encoding
                    _ => return Err(VMErrors::InvalidOpcode),
                }
            }
            crate::instructions::DecodedInstruction::SType(stype) => {
                match stype.funct3 {
                    0b000 => {
                        // Funct3 for sb
                        todo!()
                    }
                    0b001 => {
                        // Funct3 for sh
                        todo!()
                    }
                    0b010 => {
                        // Funct3 for sw
                        todo!()
                    }
                    _ => return Err(VMErrors::InvalidOpcode),
                }
            }
            crate::instructions::DecodedInstruction::BType(btype) => {
                match btype.funct3 {
                    0b000 => {
                        // Funct3 for beq
                        todo!()
                    }
                    0b001 => {
                        // Funct3 for bne
                        todo!()
                    }
                    0b100 => {
                        // Funct3 for blt
                        todo!()
                    }
                    0b101 => {
                        // Funct3 for bge
                        todo!()
                    }
                    0b110 => {
                        // Funct3 for bltu
                        todo!()
                    }
                    0b111 => {
                        // Funct3 for bgeu
                        todo!()
                    }
                    _ => return Err(VMErrors::InvalidOpcode),
                }
            }
            crate::instructions::DecodedInstruction::UType(utype) => {
                match decoded_instruction.opcode {
                    0b0110111 => {
                        // Funct3 for lui
                        todo!()
                    }
                    0b0010111 => {
                        // Funct3 for auipc
                        todo!()
                    }
                    _ => return Err(VMErrors::InvalidOpcode),
                }
            }
            crate::instructions::DecodedInstruction::JType(jtype) => {
                match decoded_instruction.opcode {
                    0b1101111 => {
                        // Funct3 for jal
                        todo!()
                    }
                    _ => return Err(VMErrors::InvalidOpcode),
                }
            }
        }
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
                            eprintln!("Error at pc: {:x} - error: {:?}", self.pc, e);
                        }
                    }
                    self.running = false;
                }
            }
        }
    }
}
