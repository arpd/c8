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
