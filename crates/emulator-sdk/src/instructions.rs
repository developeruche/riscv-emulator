use crate::vm::VMErrors;

#[derive(Debug, Clone, PartialEq)]
pub struct RType {
    pub funct7: u32,
    pub rs2: usize,
    pub rs1: usize,
    pub funct3: u32,
    pub rd: usize,
}

impl RType {
    pub fn new(insn: u32) -> RType {
        RType {
            funct7: (insn >> 25) & 0x7f,
            rs2: ((insn >> 20) & 0x1f) as usize,
            rs1: ((insn >> 15) & 0x1f) as usize,
            funct3: (insn >> 12) & 0x7,
            rd: ((insn >> 7) & 0x1f) as usize,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IType {
    pub imm: i32,
    pub rs1: usize,
    pub funct3: u32,
    pub rd: usize,
}

impl IType {
    pub fn new(insn: u32) -> IType {
        let uimm: i32 = ((insn >> 20) & 0x7ff) as i32;

        let imm: i32 = if (insn & 0x8000_0000) != 0 {
            uimm - (1 << 11)
        } else {
            uimm
        };

        IType {
            imm,
            rs1: ((insn >> 15) & 0x1f) as usize,
            funct3: (insn >> 12) & 0x7,
            rd: ((insn >> 7) & 0x1f) as usize,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SType {
    pub imm: i32,
    pub rs2: usize,
    pub rs1: usize,
    pub funct3: u32,
}

impl SType {
    pub fn new(insn: u32) -> SType {
        let uimm: i32 = (((insn >> 20) & 0x7e0) | ((insn >> 7) & 0x1f)) as i32;

        let imm: i32 = if (insn & 0x8000_0000) != 0 {
            uimm - (1 << 11)
        } else {
            uimm
        };

        SType {
            imm,
            rs2: ((insn >> 20) & 0x1f) as usize,
            rs1: ((insn >> 15) & 0x1f) as usize,
            funct3: (insn >> 12) & 0x7,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BType {
    pub imm: i32,
    pub rs2: usize,
    pub rs1: usize,
    pub funct3: u32,
}

impl BType {
    pub fn new(insn: u32) -> BType {
        let uimm: i32 =
            (((insn >> 20) & 0x7e0) | ((insn >> 7) & 0x1e) | ((insn & 0x80) << 4)) as i32;

        let imm: i32 = if (insn & 0x8000_0000) != 0 {
            uimm - (1 << 12)
        } else {
            uimm
        };

        BType {
            imm,
            rs2: ((insn >> 20) & 0x1f) as usize,
            rs1: ((insn >> 15) & 0x1f) as usize,
            funct3: (insn >> 12) & 0x7,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UType {
    pub imm: i32,
    pub rd: usize,
}

impl UType {
    pub fn new(insn: u32) -> UType {
        UType {
            imm: (insn & 0xffff_f000) as i32,
            rd: ((insn >> 7) & 0x1f) as usize,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct JType {
    pub imm: i32,
    pub rd: usize,
}

impl JType {
    pub fn new(insn: u32) -> JType {
        let uimm: i32 =
            ((insn & 0xff000) | ((insn & 0x100000) >> 9) | ((insn >> 20) & 0x7fe)) as i32;

        let imm: i32 = if (insn & 0x8000_0000) != 0 {
            uimm - (1 << 20)
        } else {
            uimm
        };

        JType {
            imm,
            rd: ((insn >> 7) & 0x1f) as usize,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DecodedInstruction {
    RType(RType),
    IType(IType),
    SType(SType),
    BType(BType),
    UType(UType),
    JType(JType),
}

pub const REGISTER_CLASS: u32 = 0b0110011;
pub const IMMEDIATE_CLASS: u32 = 0b0010011;
pub const IMMEDIATE_LOAD_CLASS: u32 = 0b0000011;
pub const STORE_CLASS: u32 = 0b0100011;
pub const BRANCH_CLASS: u32 = 0b1100011;
pub const JAL_CLASS: u32 = 0b1101111;
pub const JALR_CLASS: u32 = 0b1100111;
pub const UPPER_IMMEDIATE_CLASS: u32 = 0b0110111;
pub const ENVIRONMENT_CLASS: u32 = 0b1110011;

#[derive(Debug, Clone)]
pub struct InstructionDecoder {
    pub decoded_instruction: DecodedInstruction,
}

impl InstructionDecoder {
    pub fn decode(instruction: &u32) -> Result<Self, VMErrors> {
        let opcode = instruction & 0x7f;

        match opcode {
            REGISTER_CLASS => {
                let decoded_instruction = DecodedInstruction::RType(RType::new(*instruction));
                return Ok(Self {
                    decoded_instruction,
                });
            }
            IMMEDIATE_CLASS | IMMEDIATE_LOAD_CLASS => {
                let decoded_instruction = DecodedInstruction::IType(IType::new(*instruction));
                return Ok(Self {
                    decoded_instruction,
                });
            }
            STORE_CLASS => {
                let decoded_instruction = DecodedInstruction::SType(SType::new(*instruction));
                return Ok(Self {
                    decoded_instruction,
                });
            }
            BRANCH_CLASS => {
                let decoded_instruction = DecodedInstruction::BType(BType::new(*instruction));
                return Ok(Self {
                    decoded_instruction,
                });
            }
            JAL_CLASS => {
                let decoded_instruction = DecodedInstruction::JType(JType::new(*instruction));
                return Ok(Self {
                    decoded_instruction,
                });
            }
            JALR_CLASS => {
                let decoded_instruction = DecodedInstruction::IType(IType::new(*instruction));
                return Ok(Self {
                    decoded_instruction,
                });
            }
            UPPER_IMMEDIATE_CLASS => {
                let decoded_instruction = DecodedInstruction::UType(UType::new(*instruction));
                return Ok(Self {
                    decoded_instruction,
                });
            }
            ENVIRONMENT_CLASS => Err(VMErrors::EnvironmentError),
            _ => Err(VMErrors::InvalidOpcode),
        }
    }
}
