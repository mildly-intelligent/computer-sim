use core::ops;

pub const SIZE: usize = 256_usize.pow(2)-1;

/// Flags taken from https://en.wikibooks.org/wiki/X86_Assembly/X86_Architecture#Memory
pub enum Flags {
    /// Carry flag
    CF,
    // 1
    /// Parity flag
    PF,
    // 0
    /// Adjust flag
    AF,
    // 0
    /// Zero flag
    ZF,
    /// Sign flag
    SF,
    /// Trap flag
    TF,
    /// Interrupt flag
    IF,
    /// Direction flag
    DF,
    /// Overflow flag
    OF,
    /// I/O Privilege level (low)
    IOPLL,
    /// I/O Privilege level (high)
    IOPLH,
    /// Nested Task flag
    NT,
    // 0
}
impl From<Flags> for u16 {
    fn from(value: Flags) -> Self {
        match value {
            Flags::CF =>    0b00000000_00000001, 
            Flags::PF =>    0b00000000_00000100,
            Flags::AF =>    0b00000000_00010000,
            Flags::ZF =>    0b00000000_01000000,
            Flags::SF =>    0b00000000_10000000,
            Flags::TF =>    0b00000001_00000000,
            Flags::IF =>    0b00000010_00000000,
            Flags::DF =>    0b00000100_00000000,
            Flags::OF =>    0b00001000_00000000,
            Flags::IOPLL => 0b00010000_00000000,
            Flags::IOPLH => 0b00100000_00000000,
            Flags::NT =>    0b01000000_00000000,
        }
    }
}
impl ops::BitOr<Flags> for Register {
    type Output = Register;

    fn bitor(self, rhs: Flags) -> Self::Output {
        (u16::from(self) | u16::from(rhs)).into()
    }
}

pub struct Register {
    pub high: u8,
    pub low: u8,
}
impl Register {
    pub fn raw(self) -> u16 {
        self.low as u16 * 0x100 + self.high as u16
    }
}
impl From<Register> for u16 {
    fn from(value: Register) -> u16 {
        value.high as u16 * 0x100 + value.low as u16
    }
}
impl Into<Register> for u16 {
    fn into(self) -> Register {
        Register {
            high: ((self & 0b11111111_00000000) > 8) as u8,
            low: (self & 0b00000000_11111111) as u8
        }
    }
}

pub struct Ram([u8; SIZE]);
impl Ram {
    pub fn get(self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    pub fn store(&mut self, addr: u16, value: u8) {
        self.0[addr as usize] = value;
    }
}
impl Default for Ram {
    fn default() -> Self {
        Self([0; SIZE])
    }
}

pub struct Memory {
    pub ram: Ram,
    pub ax: Register,
    pub bx: Register,
    pub cx: Register,
    pub dx: Register,
    pub pc: Register,
    pub flags: Register,
}

impl Memory {

}