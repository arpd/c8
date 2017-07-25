mod instruction;
mod platform;
mod operator;
mod decoder;

fn main() {
    println!("Hello, world!");
    let i = instruction::Instruction::new(0);
    println!("{}", i);
    let i = instruction::Instruction::new(0x00e0);
    println!("{}", i);
    let i = instruction::Instruction::new(0x00ee);
    println!("{}", i);
    let i = instruction::Instruction::new(0xee00);
    println!("{}", i);
}
