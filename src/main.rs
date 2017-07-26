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

    let i = instruction::Instruction::new(0xe09e);
    println!("{}", i);
    let i = instruction::Instruction::new(0xe091);
    println!("{}", i);
    let i = instruction::Instruction::new(0xe59e);
    println!("{}", i);
    let i = instruction::Instruction::new(0xef91);
    println!("{}", i);
}
