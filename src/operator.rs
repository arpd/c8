use instruction::{Operand};
use platform::{Chip8Context};

pub struct Operator {
    pub mnemonic: &'static str,
    pub implementation: fn(operands: &Vec<Operand>, ctxt: &mut Chip8Context),
}

// NOTE: We have to implement the `Clone` trait ourselves here, rather than
// being able to `#[drive(Clone)]`, due to a bug in the type system:
// https://github.com/rust-lang/rust/issues/28229
// When this is fixed we can just remove this and use derive again.
impl Clone for Operator {
    fn clone(&self) -> Operator {
        Operator {
            mnemonic: self.mnemonic,
            implementation: self.implementation,
        }
    }
}

pub fn call_rca1802(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn clear_display(ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn ret(ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn key_neq(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn key_eq(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn get_delay_timer(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn get_key(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn set_delay_timer(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn set_sound_timer(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn add_i(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn set_i_sprite_addr(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn bcd(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn dump_regs(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn load_regs(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}
