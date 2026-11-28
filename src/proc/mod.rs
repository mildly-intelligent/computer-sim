/// Code for the assembler
pub mod assembler;

use crate::cpu::{Cpu, Value, Value16};

#[derive(Clone)]
pub struct Args(pub Option<Value>, pub Option<Value>);
impl Args {
    pub fn try_new(arg1: Option<Value>, arg2: Option<Value>) -> Result<Self, ()> {
        if arg1.is_none() && arg2.is_some() {
            Err(())
        } else {
            Ok(Self(arg1, arg2))
        }
    }

    pub fn num(&self) -> usize {
        [&self.0, &self.1].iter()
            .cloned()
            .flatten()
            .count()
    }

    pub fn get_types(&self) -> u8 {
        let a: u8 = match &self.0 {
            None => 0b00,
            Some(value) => match value {
                Value::Reg(_) => 0b01,
                Value::Adr(_) => 0b10,
                Value::Imm(_) => 0b11,
            }
        };
        let b: u8 = match &self.1 {
            None => 0b00,
            Some(value) => match value {
                Value::Reg(_) => 0b01,
                Value::Adr(_) => 0b10,
                Value::Imm(_) => 0b11,
            }
        };

        (a << 2) + b
    }
}
impl<'a> Into<(Option<(u8, u8)>, Option<(u8, u8)>)> for Args {
    fn into(self) -> (Option<(u8, u8)>, Option<(u8, u8)>) {
        (
            self.0
                .and_then(|a| Some(Value16::from(&a)))
                .and_then(|a| Some((a.low, a.high))),
            self.1
                .and_then(|a| Some(Value16::from(&a)))
                .and_then(|a| Some((a.low, a.high))),
        )
    }
}

#[derive(Clone)]
pub enum Instruction {
    DEC =  0x00,
    HLT =  0x01,
    INC =  0x02,
    NOP =  0x03,
    POP =  0x04,
    RCR =  0x05,
    RCL =  0x06,
    ROR =  0x07,
    ROL =  0x08,
    ADC =  0x09,
    ADD =  0x0A,
    AND =  0x0B,
    DIV =  0x0C,
    MUL =  0x0D,
    NOT =  0x0E,
    OR =   0x0F,
    PUSH = 0x10,
    SUB =  0x11,
    XOR =  0x12,
    MOV =  0x13,
}

impl Instruction {
    pub fn get_opcode(self) -> u8 {
        self as isize as u8
    }

    pub fn get_func(self) -> fn(&mut Cpu, Args) {
        match self {
            Self::ADC => Cpu::ADC,
            Self::ADD => Cpu::ADD,
            Self::AND => Cpu::AND,
            Self::DEC => Cpu::DEC,
            Self::DIV => Cpu::DIV,
            Self::HLT => Cpu::HLT,
            Self::INC => Cpu::INC,
            Self::MOV => Cpu::MOV,
            Self::MUL => Cpu::MUL,
            Self::NOP => Cpu::NOP,
            Self::NOT => Cpu::NOT,
            Self::OR => Cpu::OR,
            Self::POP => Cpu::POP,
            Self::PUSH => Cpu::PUSH,
            Self::RCR => Cpu::RCR,
            Self::RCL => Cpu::RCL,
            Self::ROR => Cpu::ROR,
            Self::ROL => Cpu::ROL,
            Self::SUB => Cpu::SUB,
            Self::XOR => Cpu::XOR,
        }
    }
}
impl TryFrom<String> for Instruction {
    type Error = assembler::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "DEC" => Ok(Self::DEC),
            "HLT" => Ok(Self::HLT),
            "INC" => Ok(Self::INC),
            "NOP" => Ok(Self::NOP),
            "POP" => Ok(Self::POP),
            "RCR" => Ok(Self::RCR),
            "RCL" => Ok(Self::RCL),
            "ROR" => Ok(Self::ROR),
            "ROL" => Ok(Self::ROL),
            "ADC" => Ok(Self::ADC),
            "ADD" => Ok(Self::ADD),
            "AND" => Ok(Self::AND),
            "DIV" => Ok(Self::DIV),
            "MUL" => Ok(Self::MUL),
            "NOT" => Ok(Self::NOT),
            "OR" => Ok(Self::OR),
            "PUSH" => Ok(Self::PUSH),
            "SUB" => Ok(Self::SUB),
            "XOR" => Ok(Self::XOR),
            "MOV" => Ok(Self::MOV),
            _ => Err(assembler::Error::UnknownInstruction(value.to_uppercase()))
        }
    }
}

pub struct InstructionCall {
    pub instruction: Instruction,
    pub args: Args,
}
impl InstructionCall {
    pub fn try_new(instruction: Instruction, arg1: Option<Value>, arg2: Option<Value>) -> Result<Self, ()> {
        Ok(Self {
            instruction,
            args: Args::try_new(arg1, arg2)?,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let (arg1, arg2) = &self.args.clone().into();
        match self.args.num() {
            0 => vec![self.instruction.clone().get_opcode(), self.args.get_types()],
            1 => vec![self.instruction.clone().get_opcode(), self.args.get_types(), arg1.unwrap().0, arg1.unwrap().1],
            2 => vec![self.instruction.clone().get_opcode(), self.args.get_types(), arg1.unwrap().0, arg1.unwrap().1, arg2.unwrap().0, arg2.unwrap().1],
            _ => panic!("unreachable"),
        }
    }

    pub fn run(self, cpu: &mut Cpu) {
        self.instruction.get_func()(cpu, self.args)
    }
}