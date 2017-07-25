use instruction::{Operand};
use platform::{Chip8Context};
use operator::*;

// TODO: Remove when operations & decoders implemented
fn test(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {}

// The following `decode_XXXXX` functions are named such that they decode an
// instruction with the operands matching `XXXXX`, for example:
// `decode_addr12` decodes instructions that provide a 12 bit address,
// `decode_regx_regy_lit` decodes instructions that provide the X and Y
//                        registers, and a 4 bit literal
#[inline]
pub fn decode_addr12(src: u16) -> Operator {
    Operator {
        mnemonic: "TEST",
        implementation: test
    }
}
#[inline]
pub fn decode_regx_addr8(src: u16) -> Operator {
    Operator {
        mnemonic: "TEST",
        implementation: test
    }
}

#[inline]
pub fn decode_regx_regy(src: u16) -> Operator {
    Operator {
        mnemonic: "TEST",
        implementation: test
    }
}

#[inline] pub fn decode_regx_regy_addr4(src: u16) -> Operator {
    Operator {
        mnemonic: "TEST",
        implementation: test
    }
}

#[inline]
pub fn decode_regx(src: u16) -> Operator {
    Operator {
        mnemonic: "TEST",
        implementation: test
    }
}

#[inline]
pub fn decode_special(src: u16) -> Operator {
    match src {
        0x00e0 => Operator {
            mnemonic: "CLEAR_DISPLAY",
            implementation: |x, y| clear_display(y)
        },
        0x00ee => Operator {
            mnemonic: "RET",
            implementation: |x, y| ret(y)
        },
        _      => Operator {
            mnemonic: "RCA1802_CALL",
            implementation: |x, y| call_rca1802(x, y)
        },
    }
}

