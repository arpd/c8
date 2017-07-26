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

pub fn draw(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn skip_eq_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn skip_neq_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn mov_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn or_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn and_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn xor_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn add_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn sub_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn rshift_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn sub_regy_regx(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn lshift_regx_regy(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn skip_eq_regx_addr8(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn skip_neq_regx_addr8(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn mov_regx_addr8(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn add_regx_addr8(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn rand_regx_addr8(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

pub fn jmp_addr12(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}
pub fn call_addr12(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}
pub fn mov_i_addr12(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}
pub fn jmp_addr12_offset_regx(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    unimplemented!()
}

