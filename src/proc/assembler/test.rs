
use super::tokenize::{self, Token::*, UnqualifiedToken as uq};

const ASM: &'static str = "
    ; This is a test comment
    MOV %ax 6
    ADD 0x0050
    MOV $0x5009 %bx ; test
    HLT";

#[test]
fn tokenize_pass1() {
    assert_eq!(tokenize::tokenize_pass1(ASM.to_string()).unwrap(), vec![
        Comment,
        Word(String::from("MOV")), Word(String::from("%ax")), Word(String::from("6")),
        Word(String::from("ADD")), Word(String::from("0x0050")),
        Word(String::from("MOV")), Word(String::from("$0x5009")), Word(String::from("%bx")), Comment,
        Word(String::from("HLT"))
    ]);
}

#[test]
fn tokenize_pass2() {
    assert_eq!(tokenize::tokenize_pass1(ASM.to_string()).unwrap(), vec![
        Comment,
        Unqualified(uq::Instruction(String::from("MOV"))), Unqualified(uq::Register(String::from("%ax"))), Unqualified(uq::Immediate(String::from("6"))),
        Unqualified(uq::Instruction(String::from("ADD"))), Unqualified(uq::ImmediateHex(String::from("0x0050"))),
        Unqualified(uq::Instruction(String::from("MOV"))), Unqualified(uq::Address(String::from("$0x5009"))), Unqualified(uq::Register(String::from("%bx"))), Comment,
        Unqualified(uq::Instruction(String::from("HLT")))
    ]);
}

// #[test]
// fn tokenize() {
//     assert_eq!(tokenize::tokenize(ASM.to_string()), vec![
//         Comment,
//         Op(&Instruction(0x13), vec![&Operand(&Register(0)), &Operand(&Immediate(0x0006))]),
//         Op(&Instruction(0x0A), vec![&Operand(&Immediate(0x0050))]),
//         Op(&Instruction(0x13), vec![&Address(0x5009), &Operand(&Register(1))]), Comment,
//         Op(&Instruction(0x01), vec![])
//     ]);
// }