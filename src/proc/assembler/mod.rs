mod tokenize;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum Error {
    InvalidWord(String),
    NumericParseError(String),
    HexParseError(String),
    NumericOverflow(String),
    UnknownRegister(String),
    UnknownInstruction(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWord(word) => write!(f, "Unable to parse word '{word}'"),
            Self::NumericParseError(number) => write!(f, "Unable to parse '{number}' as a number"),
            Self::HexParseError(number) => write!(f, "Unable to parse '{number}' as a hexadecimal number"),
            Self::NumericOverflow(number) => write!(f, "'{number}' is outside the 16-bit range"),
            Self::UnknownRegister(reg) => write!(f, "The register '{reg}' does not exist"),
            Self::UnknownInstruction(op) => write!(f, "The instruction '{op}' does not exist"),
        }
    }
}
impl std::error::Error for Error {}