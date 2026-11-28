/// Code for the assembler
pub mod assembler;

use crate::cpu::{Cpu, Value, Value16};

#[derive(Clone)]
pub enum Instruction {
    ADC(Value),
    ADD(Value),
    AND(Value),
    DEC(),
    DIV(Value),
    HLT(),
    INC(),
    MOV(Value, Value),
    MUL(Value),
    NOP(),
    NOT(Value),
    OR(Value),
    POP(),
    PUSH(Value),
    RCR(),
    RCL(),
    ROR(),
    ROL(),
    SUB(Value),
    XOR(Value),
}
impl Instruction {
    pub fn get_args(&self) -> (Option<&Value>, Option<&Value>) {
        match self {
            Self::DEC()  |
            Self::HLT()  |
            Self::INC()  |
            Self::NOP()  |
            Self::POP()  |
            Self::RCR()  |
            Self::RCL()  |
            Self::ROR()  |
            Self::ROL() => (None, None),
            Self::ADC(a)  |
            Self::ADD(a)  |
            Self::AND(a)  |
            Self::DIV(a)  |
            Self::MUL(a)  |
            Self::NOT(a)  |
            Self::OR(a)   |
            Self::PUSH(a) |
            Self::SUB(a)  |
            Self::XOR(a) => (Some(a), None),
            Self::MOV(a,b) => (Some(a), Some(b)),
        }
    }
    pub fn get_arg_type(&self) -> u8 {
        let (a, b) = self.get_args();

        let a: u8 = match a {
            None => 0b00,
            Some(value) => match value {
                Value::Reg(_) => 0b01,
                Value::Adr(_) => 0b10,
                Value::Imm(_) => 0b11,
            }
        };
        let b: u8 = match b {
            None => 0b00,
            Some(value) => match value {
                Value::Reg(_) => 0b01,
                Value::Adr(_) => 0b10,
                Value::Imm(_) => 0b11,
            }
        };

        (a << 2) + b
    }

    pub fn get_opcode(&self) -> u8 {
        match self {
            Self::DEC() =>    0x00,
            Self::HLT() =>    0x00,
            Self::INC() =>    0x00,
            Self::NOP() =>    0x00,
            Self::POP() =>    0x00,
            Self::RCR() =>    0x00,
            Self::RCL() =>    0x00,
            Self::ROR() =>    0x00,
            Self::ROL() =>    0x00,
            Self::ADC(_) =>   0x00,
            Self::ADD(_) =>   0x00,
            Self::AND(_) =>   0x00,
            Self::DIV(_) =>   0x00,
            Self::MUL(_) =>   0x00,
            Self::NOT(_) =>   0x00,
            Self::OR(_) =>    0x00,
            Self::PUSH(_) =>  0x00,
            Self::SUB(_) =>   0x00,
            Self::XOR(_) =>   0x00,
            Self::MOV(_,_) => 0x00,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::DEC() |
            Self::HLT() |
            Self::INC() |
            Self::NOP() |
            Self::POP() |
            Self::RCR() |
            Self::RCL() |
            Self::ROR() |
            Self::ROL() => vec![self.get_opcode(), self.get_arg_type()],
            Self::ADC(a) |
            Self::ADD(a) |
            Self::AND(a) |
            Self::DIV(a) |
            Self::MUL(a) |
            Self::NOT(a) |
            Self::OR(a) |
            Self::PUSH(a) |
            Self::SUB(a) |
            Self::XOR(a) => {
                let a = Value16::from(a);
                vec![self.get_opcode(), self.get_arg_type(), a.high, a.low]
            }
            Self::MOV(a,b) => {
                let a = Value16::from(a);
                let b = Value16::from(b);
                vec![self.get_opcode(), self.get_arg_type(), a.high, a.low, b.high, b.low]
            }
        }
    }

    pub fn run(self, cpu: &mut Cpu) {
        match self {
            Self::ADC(a) => cpu.ADC(a),
            Self::ADD(a) => cpu.ADD(a),
            Self::AND(a) => cpu.AND(a),
            Self::DEC() => cpu.DEC(),
            Self::DIV(a) => cpu.DIV(a),
            Self::HLT() => cpu.HLT(),
            Self::INC() => cpu.INC(),
            Self::MOV(a,b) => cpu.MOV(a,b),
            Self::MUL(a) => cpu.MUL(a),
            Self::NOP() => cpu.NOP(),
            Self::NOT(a) => cpu.NOT(a),
            Self::OR(a) => cpu.OR(a),
            Self::POP() => cpu.POP(),
            Self::PUSH(a) => cpu.PUSH(a),
            Self::RCR() => cpu.RCR(),
            Self::RCL() => cpu.RCL(),
            Self::ROR() => cpu.ROR(),
            Self::ROL() => cpu.ROL(),
            Self::SUB(a) => cpu.SUB(a),
            Self::XOR(a) => cpu.XOR(a),
        }
    }
}