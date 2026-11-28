
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
    let pass1 = tokenize::tokenize_pass1(ASM.to_string()).unwrap();

    assert_eq!(tokenize::tokenize_pass2(pass1).unwrap(), vec![
        Comment,
        Unqualified(uq::Instruction(String::from("MOV"))), Unqualified(uq::Register(String::from("ax"))), Unqualified(uq::Immediate(String::from("6"))),
        Unqualified(uq::Instruction(String::from("ADD"))), Unqualified(uq::ImmediateHex(String::from("0050"))),
        Unqualified(uq::Instruction(String::from("MOV"))), Unqualified(uq::Address(String::from("5009"))), Unqualified(uq::Register(String::from("bx"))), Comment,
        Unqualified(uq::Instruction(String::from("HLT")))
    ]);
}

#[test]
fn tokenize_pass3() {
    let pass1 = tokenize::tokenize_pass1(ASM.to_string()).unwrap();
    let pass2 = tokenize::tokenize_pass2(pass1).unwrap();

    assert_eq!(tokenize::tokenize_pass3(pass2).unwrap(), vec![
        Comment,
        Instruction(0x13), Register(0), Immediate(6),
        Instruction(0x0A), Immediate(0x0050),
        Instruction(0x13), Address(0x5009), Register(1), Comment,
        Instruction(0x01)
    ]);
}

// #[test]
// fn tokenize() {
//     assert_eq!(tokenize::tokenize(ASM.to_string()), vec![
//         Comment,
//         Op(&Instruction(0x13), vec![&Operand(&Register(0)), &Operand(&Immediate(6))]),
//         Op(&Instruction(0x0A), vec![&Operand(&Immediate(0x0050))]),
//         Op(&Instruction(0x13), vec![&Address(0x5009), &Operand(&Register(1))]), Comment,
//         Op(&Instruction(0x01), vec![])
//     ]);
// }