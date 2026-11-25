use computer::prelude::*;

fn main() {
    let mut mem = memory::Memory::default();

    // println!("{}", mem.ram);
    mem.ram.store_reg(0xFFF4, &mem.ax);
    mem.ram.load_reg(0xFFF4, &mut mem.bx);
    println!("{}", mem.bx)
}
