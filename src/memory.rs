/// Number of bytes in ram.
/// Large enough to be accessed with a 16-bit address
pub const SIZE: usize = 2usize.pow(16)-1;

// #region RAM
/// Class to store the Random Access Memory
pub struct Ram([u8; SIZE]);
impl Ram {
    /// Gets the byte at `addr`
    pub fn get(&self, addr: u16) -> u8 {
        self.0[addr as usize]
    }

    /// Gets the 16-bit value at `addr`
    pub fn getw(&self, addr: u16) -> u16 {
        ((self.0[addr as usize] as u16) << 8) + self.0[addr as usize + 1] as u16
    }

    /// Reads two-byte value at `addr` into the register `reg`
    pub fn load_reg(&self, addr:u16, reg: &mut Register) {
        reg.low = self.get(addr);
        reg.high = self.get(addr+1);
    }

    /// Stores a byte `value` at some `addr`
    pub fn store(&mut self, addr: u16, value: u8) {
        self.0[addr as usize] = value;
    }

    /// Stores a two-byte register `value` at `addr`
    pub fn store_reg(&mut self, addr: u16, value: Box<Register>) {
        self.0[addr as usize] = value.low;
        self.0[addr as usize + 1] = value.high;
    }

    /// Gets an ASCII string representation of the RAM
    pub unsafe fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}
impl Default for Ram {
    fn default() -> Self {
        Self([0; SIZE])
    }
}
impl std::fmt::Display for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.map(|a| format!("{a:02X}")).join(""))
    }
}
// #endregion

// #region Registers
/// A two-byte value
#[derive(Clone, Copy)]
pub struct Register {
    pub index: u8,
    pub high: u8,
    pub low: u8,
}
impl Register {
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the bytes in little-endian form
    pub fn raw(self) -> u16 {
        self.low as u16 * 0x100 + self.high as u16
    }
}
impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:02X}{:02X}", self.low, self.high)
    }
}
impl Default for Register {
    fn default() -> Self {
        Self { index: 0xFF, high: 0x00, low: 0x00 }
    }
}
impl From<Register> for u16 {
    fn from(value: Register) -> u16 {
        value.high as u16 * 0x100 + value.low as u16
    }
}
impl From<u16> for Register {
    fn from(value: u16) -> Register {
        Register {
            index: 0xFF,
            high: ((value & 0xFF00) >> 8) as u8,
            low: (value & 0x00FF) as u8
        }
    }
}

/// Flags for the flag register taken from https://en.wikibooks.org/wiki/X86_Assembly/X86_Architecture#Memory
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
impl core::ops::BitOr<Flags> for Register {
    type Output = Register;

    fn bitor(self, rhs: Flags) -> Self::Output {
        (u16::from(self) | u16::from(rhs)).into()
    }
}
// #endregion