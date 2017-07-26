use instruction::{Operand, get_nibble};
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
    unimplemented!();
    Operator {
        mnemonic: "TEST",
        implementation: test
    }
}
#[inline]
pub fn decode_regx_addr8(src: u16) -> Operator {
    unimplemented!();
    Operator {
        mnemonic: "TEST",
        implementation: test
    }
}

#[inline]
pub fn decode_regx_regy(src: u16) -> Operator {
    unimplemented!();
    Operator {
        mnemonic: "TEST",
        implementation: test
    }
}

#[inline] pub fn decode_regx_regy_addr4(src: u16) -> Operator {
    unimplemented!();
    Operator {
        mnemonic: "TEST",
        implementation: test
    }
}

#[inline]
pub fn decode_regx(src: u16) -> Operator {
    match get_nibble(src, 3) {
        0xe => {
            match get_nibble(src, 0) {
                0x1 => Operator {
                    mnemonic: "KEY_NEQ",
                    implementation: |x, y| key_neq(x, y),
                },
                0xe => Operator {
                    mnemonic: "KEY_EQ",
                    implementation: |x, y| key_eq(x, y),
                },
                _ => unimplemented!(),
            }
        },

        0xf => {
            match src & 0xff {
                0x07 => Operator {
                    mnemonic: "GET_DELAY_TIMER",
                    implementation: |x, y| get_delay_timer(x, y),
                },
                0x0a => Operator {
                    mnemonic: "GET_KEY",
                    implementation: |x, y| get_key(x, y),
                },
                0x15 => Operator {
                    mnemonic: "SET_DELAY_TIMER",
                    implementation: |x, y| set_delay_timer(x, y),
                },
                0x18 => Operator {
                    mnemonic: "SET_SOUND_TIMER",
                    implementation: |x, y| set_sound_timer(x, y),
                },
                0x1e => Operator {
                    mnemonic: "ADD_I",
                    implementation: |x, y| add_i(x, y),
                },
                0x29 => Operator {
                    mnemonic: "SET_I_SPRITE_ADDR",
                    implementation: |x, y| set_i_sprite_addr(x, y),
                },
                0x33 => Operator {
                    mnemonic: "BCD",
                    implementation: |x, y| bcd(x, y),
                },
                0x55 => Operator {
                    mnemonic: "DUMP_REGS",
                    implementation: |x, y| dump_regs(x, y),
                },
                0x65 => Operator {
                    mnemonic: "LOAD_REGS",
                    implementation: |x, y| load_regs(x, y),
                },

                _ => unimplemented!(),
            }
        },

        _ => unimplemented!()
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
