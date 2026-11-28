#[derive(Debug, PartialEq, Clone)]
pub(super) enum Token<'a> {
    Comment,
    Op(&'a Token<'a>, &'a Token<'a>),
    Instruction(crate::proc::Instruction),
    Operands(Vec<Token<'a>>),
    Operand(&'a Token<'a>),
    Immediate(u16),
    Address(u16),
    Register(u8),
    Word(String),
    Unqualified(UnqualifiedToken),
}
impl<'a> TryFrom<UnqualifiedToken> for Token<'a> {
    type Error = super::Error;

    fn try_from(value: UnqualifiedToken) -> Result<Self, Self::Error> {
        match value {
            UnqualifiedToken::Instruction(op) => {
                crate::proc::Instruction::try_from(op)
                    .and_then(|a| Ok(Token::Instruction(a)))
            },
            UnqualifiedToken::Immediate(imm) => {
                let value: u16 = imm.parse::<u16>()
                    .or_else(|a| Err(match a.kind() {
                        std::num::IntErrorKind::PosOverflow => super::Error::NumericOverflow(imm),
                        _ => super::Error::NumericParseError(imm),
                    }))?;
                Ok(Token::Immediate(value))
            },
            UnqualifiedToken::ImmediateHex(imm) => {
                if !imm.chars().all(|a| char::is_ascii_hexdigit(&a)) {
                    Err(super::Error::HexParseError("0x".to_owned() + &imm.clone()))?
                }
                let value = u16::from_str_radix(imm.as_str(), 16)
                    .or_else(|a| Err(match a.kind() {
                        std::num::IntErrorKind::PosOverflow => super::Error::NumericOverflow("0x".to_owned() + &imm),
                        _ => super::Error::HexParseError("0x".to_owned() + &imm),
                    }))?;
                Ok(Token::Immediate(value))
            },
            UnqualifiedToken::Address(adr) => {
                if !adr.chars().all(|a| char::is_ascii_hexdigit(&a)) {
                    Err(super::Error::HexParseError("$0x".to_owned() + &adr.clone()))?
                }
                let value = u16::from_str_radix(adr.as_str(), 16)
                    .or_else(|a| Err(match a.kind() {
                        std::num::IntErrorKind::PosOverflow => super::Error::NumericOverflow("$0x".to_owned() + &adr),
                        _ => super::Error::HexParseError("$0x".to_owned() + &adr),
                    }))?;
                Ok(Token::Address(value))
            },
            UnqualifiedToken::Register(reg) => {
                match reg.as_str() {
                    "ax" => Ok(Token::Register(0)),
                    "bx" => Ok(Token::Register(1)),
                    "cx" => Ok(Token::Register(2)),
                    "dx" => Ok(Token::Register(3)),
                    _ => Err(super::Error::UnknownRegister(reg)),
                }
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(super) enum UnqualifiedToken {
    Instruction(String),
    Immediate(String),
    ImmediateHex(String),
    Address(String),
    Register(String),
}
impl TryFrom<String> for UnqualifiedToken {
    type Error = super::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with('%') {
            Ok(Self::Register(value.strip_prefix('%').unwrap_or(&value).to_string()))
        } else if value.starts_with("$0x") {
            Ok(Self::Address(value.strip_prefix("$0x").unwrap_or(&value).to_string()))
        } else if value.starts_with("0x") {
            Ok(Self::ImmediateHex(value.strip_prefix("0x").unwrap_or(&value).to_string()))
        } else if value.chars().all(char::is_numeric) {
            Ok(Self::Immediate(value))
        } else if value.chars().all(char::is_alphabetic) {
            Ok(Self::Instruction(value))
        } else {
            Err(Self::Error::InvalidWord(value))
        }
    }
}

/// First pass of the tokenization process.
/// 
/// Separates out [comments](Token::Comment) and splits lines into individual [words](Token::Word) (separates by space)
pub(super) fn tokenize_pass1<'a>(asm: String) -> Result<Vec<Token<'a>>, super::Error> {
    let lines = asm.split('\n').collect::<Vec<_>>();
    let mut tokens = Vec::new();

    for line in lines {
        let line = line.split(';').collect::<Vec<_>>();
        let has_comment = line.len() > 1;

        let mut line = line[0]
            .trim()
            .split(' ')
            .filter(|a| *a != "")
            .map(|a| Token::Word(a.to_string()))
            .collect::<Vec<_>>();

        tokens.append(&mut line);
        if has_comment { tokens.push(Token::Comment); }
    }

    Ok(tokens)
}

/// Second pass of the tokenization process
/// 
/// Converts [Token::Word] into unqualified tokens. See [UnqualifiedToken] for a description on what that means
pub(super) fn tokenize_pass2<'a>(tokens: Vec<Token<'a>>) -> Result<Vec<Token<'a>>, super::Error> {
    let mut tokens_out = Vec::new();

    for token in tokens {
        tokens_out.push(match token {
            Token::Word(token) => Token::Unqualified(UnqualifiedToken::try_from(token)?),
            other => other,
        })
    }

    Ok(tokens_out)
}

/// Third pass of the tokenization process
/// 
/// Converts the [unqualified](UnqualifiedToken) tokens into qualified tokens. 
pub(super) fn tokenize_pass3<'a>(tokens: Vec<Token<'a>>) -> Result<Vec<Token<'a>>, super::Error> {
    let mut tokens_out = Vec::new();

    for token in tokens {
        tokens_out.push(match token {
            Token::Unqualified(token) => Token::try_from(token)?,
            other => other,
        })
    }

    Ok(tokens_out)
}

/// Fourth pass of the tokenization process
/// 
/// Turns the flattened token list into a tree.
/// ## Examples
/// ```
///     use Token::*;
///     let start = vec![Instruction(0x13), Register(0), Immediate(6)];
///     assert_eq!(tokenize_pass4(start).unwrap(), vec![Op(&Instruction(0x13)), &Operands(vec![&Operand(&Register(0)), &Operand(&Immediate(6))])]);
/// ```
pub(super) fn tokenize_pass4<'a>(tokens: Vec<Token<'a>>) -> Result<Vec<Token<'a>>, super::Error> {
    let mut tokens_out = Vec::new();
    
    let i = 0;
    while i < tokens.len() {
        let token = &tokens[i];

        match token {
            Token::Instruction(op) => op.
        }
    }

    Ok(tokens_out)
}