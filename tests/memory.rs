use computer::prelude::*;
use memory::*;

#[test]
fn register_create_1() {
    let reg: Register = u16::into(0x93E5);
    assert_eq!(reg.low, 0xE5);
    assert_eq!(reg.high, 0x93);
    assert_eq!(reg.raw(), 0xE593);
}
#[test]
fn register_create_2() {
    let reg: Register = u16::into(0x762E);
    assert_eq!(reg.low, 0x2E);
    assert_eq!(reg.high, 0x76);
    assert_eq!(reg.raw(), 0x2E76);
}
#[test]
fn register_create_3() {
    let reg: Register = u16::into(0x94D2);
    assert_eq!(reg.low, 0xD2);
    assert_eq!(reg.high, 0x94);
    assert_eq!(reg.raw(), 0xD294);
}

#[test]
fn ram_store_1() {
    let mut ram = Ram::default();
    ram.store(0x4325, 0xAB);
    assert_eq!(ram.get(0x4325), 0xAB);
}
#[test]
fn ram_store_2() {
    let mut ram = Ram::default();
    ram.store(0xA4C3, 0x11);
    assert_eq!(ram.get(0xA4C3), 0x11);
}
#[test]
fn ram_store_3() {
    let mut ram = Ram::default();
    ram.store(0x8236, 0x37);
    assert_eq!(ram.get(0x8236), 0x37);
}

#[test]
fn ram_store_reg_1() {
    let mut ram = Ram::default();

    let reg: Box<Register> = Box::new(Register::from(0x3F6A));
    ram.store_reg(0x38A2, reg.clone());
    // Make sure it doesn't expand past limits
    assert_eq!(ram.get(0x38A2-1), 0x00);
    assert_eq!(ram.get(0x38A2), reg.low);
    assert_eq!(ram.get(0x38A2+1), reg.high);
    // Make sure it doesn't expand past limits
    assert_eq!(ram.get(0x38A2+2), 0x00);
}
#[test]
fn ram_store_reg_2() {
    let mut ram = Ram::default();

    let reg: Box<Register> = Box::new(Register::from(0x1112));
    ram.store_reg(0x7E39, reg.clone());
    // Make sure it doesn't expand past limits
    assert_eq!(ram.get(0x7E39-1), 0x00);
    assert_eq!(ram.get(0x7E39), reg.low);
    assert_eq!(ram.get(0x7E39+1), reg.high);
    // Make sure it doesn't expand past limits
    assert_eq!(ram.get(0x7E39+2), 0x00);
}
#[test]
fn ram_store_reg_3() {
    let mut ram = Ram::default();

    let reg: Box<Register> = Box::new(Register::from(0x53C7));
    ram.store_reg(0x0146, reg.clone());
    // Make sure it doesn't expand past limits
    assert_eq!(ram.get(0x0146-1), 0x00);
    assert_eq!(ram.get(0x0146), reg.low);
    assert_eq!(ram.get(0x0146+1), reg.high);
    // Make sure it doesn't expand past limits
    assert_eq!(ram.get(0x0146+2), 0x00);
}

#[test]
fn ram_load_1() {
    let mut ram = Ram::default();
    
    let reg1: Box<Register> = Box::new(Register::from(0x3F6A));
    ram.store_reg(0x38A2, reg1);

    let mut reg2 = Register::new();
    ram.load_reg(0x38A2, &mut reg2);

    assert_eq!(u16::from(reg2), 0x3F6A);
}
#[test]
fn ram_load_2() {
    let mut ram = Ram::default();
    
    let reg1: Box<Register> = Box::new(Register::from(0x1112));
    ram.store_reg(0x7E39, reg1);

    let mut reg2 = Register::new();
    ram.load_reg(0x7E39, &mut reg2);

    assert_eq!(u16::from(reg2), 0x1112);
}