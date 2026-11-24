mod ram;

fn main() {
    let mut mem = ram::Memory {
        ram:   ram::Ram::default(),
        ax:    0b00000000_00000000.into(),
        bx:    0b00000000_00000000.into(),
        cx:    0b00000000_00000000.into(),
        dx:    0b00000000_00000000.into(),
        pc:    0b00000000_00000000.into(),
        flags: 0b00000000_00000000.into(),
    };

    println!("Hello, world!");
}
