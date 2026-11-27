use crate::memory::{Flags, Ram, Register};

/// Contains the types that instructions can take
pub enum Value<'a> {
    /// A register
    Reg(&'a mut Register),
    /// An address in memory
    Adr(u16),
    /// A direct value
    Imm(u16),
}

/// CPU state container
pub struct Cpu {
    /// Cpu's RAM
    pub ram: Ram,

    /// Accumulator register (result of ALU)
    pub ax: Register,
    /// General purpose register X
    pub bx: Register,
    /// General purpose register Y
    pub cx: Register,
    /// General purpose register Z
    pub dx: Register,
    /// Stack pointer register
    pub sp: Register,
    /// Instruction pointer register
    pub ip: Register,
    /// Program state
    pub flag: Register,
}

#[allow(non_snake_case)]
impl Cpu {


    pub fn ADC(&mut self, a: Value) {todo!()}
    pub fn ADD(&mut self, a: Value) {
        let a = match a {
            Value::Reg(reg) => u16::from(*reg),
            Value::Adr(adr) => self.ram.getw(adr),
            Value::Imm(imm) => imm,
        } as u32;
        let r = u16::from(self.ax) as u32;

        self.ax = if a+r > u16::MAX.into() {
            self.flag = self.flag | Flags::OF;
            (((a+r) % u16::MAX as u32) as u16).into()
        } else {
            ((a+r) as u16).into()
        }
    }
    pub fn AND(&mut self, a: Value) {todo!()}
    // pub fn CMP(&mut self, a: Value) {todo!()}
    pub fn DEC(&mut self) {
        self.ax = (u16::from(self.ax) - 1).into();
    }
    pub fn DIV(&mut self, a: Value) {
        let a = match a {
            Value::Reg(reg) => u16::from(*reg),
            Value::Adr(adr) => self.ram.getw(adr),
            Value::Imm(imm) => imm,
        } as u32;
        let r = u16::from(self.ax) as u32;

        self.ax = if a/r > u16::MAX.into() {
            self.flag = self.flag | Flags::OF;
            (((a/r) % u16::MAX as u32) as u16).into()
        } else {
            ((a/r) as u16).into()
        }
    }
    pub fn HLT(&mut self) {todo!()}
    pub fn INC(&mut self) {
        self.ax = (u16::from(self.ax) + 1).into();
    }
    // pub fn INT(&mut self) {todo!()}
    // TODO: JUMPS
    pub fn MOV(&mut self, a: Value, b: Value) {
        match a {
            Value::Reg(regA) => {
                match b {
                    Value::Reg(regB) => *regA = *regB,
                    Value::Adr(_) => panic!("Second argument to MOV cannot be an address in memory"),
                    Value::Imm(imm) => *regA = imm.into(),
                }
            },
            Value::Adr(_) => panic!("First argument to MOV cannot be an address in memory"),
            Value::Imm(_) => panic!("First argument to MOV cannot be an immediate value"),
        }
    }
    pub fn MUL(&mut self, a: Value) {
        let a = match a {
            Value::Reg(reg) => u16::from(*reg),
            Value::Adr(adr) => self.ram.getw(adr),
            Value::Imm(imm) => imm,
        } as u32;
        let r = u16::from(self.ax) as u32;

        self.ax = if a*r > u16::MAX.into() {
            self.flag = self.flag | Flags::OF;
            (((a*r) % u16::MAX as u32) as u16).into()
        } else {
            ((a*r) as u16).into()
        }
    }
    // pub fn NEG(&mut self) {todo!()}
    pub fn NOP() {todo!()}
    pub fn NOT(&mut self, a: Value) {todo!()}
    pub fn OR(&mut self, a: Value) {todo!()}
    pub fn POP(&mut self) {todo!()}
    pub fn PUSH(&mut self, a: Value) {todo!()}
    pub fn RCR(&mut self) {todo!()}
    pub fn RCL(&mut self) {todo!()}
    pub fn ROR(&mut self) {todo!()}
    pub fn ROL(&mut self) {todo!()}
    pub fn SUB(&mut self, a: Value) {
        // this may not work i literally just swapped + to - and it might've broken things
        let a = match a {
            Value::Reg(reg) => u16::from(*reg),
            Value::Adr(adr) => self.ram.getw(adr),
            Value::Imm(imm) => imm,
        } as u32;
        let r = u16::from(self.ax) as u32;

        self.ax = if a-r > u16::MAX.into() {
            self.flag = self.flag | Flags::OF;
            (((a-r) % u16::MAX as u32) as u16).into()
        } else {
            ((a-r) as u16).into()
        }
    }
    pub fn XOR(&mut self, a: Value) {todo!()}
}