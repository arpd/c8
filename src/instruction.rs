use std::fmt;
use decoder::*;
use operator::{Operator};

// TODO: We need to support a few more operands here, e.g. composable addresses (I+NNN)
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Operand {
    Register(u8),
    Address(u16),
    Literal(u16),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::Register(0xf) => write!(f, "rI"),
            Operand::Register(reg) => write!(f, "rV{:02x}", reg),
            Operand::Address(addr) => write!(f, "@0x{:04x}", addr),
            Operand::Literal(lit)  => write!(f, "$0x{:04x}", lit)
        }
    }
}

#[derive(Clone)]
pub struct Instruction {
    pub src: u16,
    pub operands: Vec<Operand>,
    //operator: fn(operands: &Vec<Operand>, ctxt: &mut Chip8Context),
    pub operator: Operator,
}

// get a nibble (i.e. 4-bit value) from a `u16`, indexed from right to left
// (0..3), `pos = 0` will return `src & 0xf`
#[inline]
pub fn get_nibble(src: u16, pos: u8) -> u8 {
    assert!(pos <= 3);
    ((src & (0xf << pos*4)) >> pos*4) as u8
}

impl Instruction {
    pub fn new(src: u16) -> Instruction {
        // TODO: Operand extraction should probably be moved in to the
        // decoder functions, and have those return a full `Instruction`
        // instead; We still need to match here to get the correct
        // decoder anyway, so work done will be identical.
        let mut operands: Vec<Operand> = Vec::<Operand>::new();
        let operator = match get_nibble(src, 3) {
            0 => {
                if src != 0x00ee && src != 0x00e0 {
                    let mut addr: u16 = get_nibble(src, 0) as u16;
                    addr |= (get_nibble(src, 1) as u16) << 1*4;
                    addr |= (get_nibble(src, 2) as u16) << 2*4;
                    operands.push(Operand::Address(addr));
                }
                decode_special(src)  
            },
            1 | 2 | 0xa | 0xb => {
                // _NNN
                let mut addr: u16 = get_nibble(src, 0) as u16;
                addr |= (get_nibble(src, 1) as u16) << 1*4;
                addr |= (get_nibble(src, 2) as u16) << 2*4;
                operands.push(Operand::Address(addr));
                decode_addr12(src)
            }
            3 | 4 | 6 | 7 | 0xc => {
                // _XNN
                let register = get_nibble(src, 2);
                let mut addr: u16 = get_nibble(src, 0) as u16;
                addr |= (get_nibble(src, 1) as u16) << 1;
                operands.push(Operand::Register(register));
                operands.push(Operand::Address(addr));
                decode_regx_addr8(src)
            }
            5 | 8 | 9 => {
                // _XY_
                let register_x = get_nibble(src, 2);
                let register_y = get_nibble(src, 1);
                operands.push(Operand::Register(register_x));
                operands.push(Operand::Register(register_y));
                decode_regx_regy(src)
            }
            0xd => {
                // _XYN
                let register_x = get_nibble(src, 2);
                let register_y = get_nibble(src, 1);
                let literal = get_nibble(src, 0) as u16;
                operands.push(Operand::Register(register_x));
                operands.push(Operand::Register(register_y));
                operands.push(Operand::Literal(literal));
                decode_regx_regy_addr4(src)
            }
            0xe | 0xf => {
                // _X__
                let register_x = get_nibble(src, 2);
                operands.push(Operand::Register(register_x));
                decode_regx(src)
            }
            _ => unimplemented!(),
        };

        Instruction {
            src,
            operands: operands.clone(),
            operator: operator,
        }
    }
}

// FIXME: This is arbitrarily the size of "CLEAR_DISPLAY" + 1 for now
const MAX_MNEMONIC_LEN: usize = 14;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.operands.len() {
            0 => write!(f, "{mnemonic:width$}",
                        width = MAX_MNEMONIC_LEN,
                        mnemonic = self.operator.mnemonic),
            1 => write!(f, "{mnemonic:width$} {oper_x}",
                        width = MAX_MNEMONIC_LEN,
                        mnemonic = self.operator.mnemonic,
                        oper_x = self.operands[0]),
            2 => write!(f, "{mnemonic:width$} {oper_x},{oper_y}",
                        width = MAX_MNEMONIC_LEN,
                        mnemonic = self.operator.mnemonic,
                        oper_x = self.operands[0],
                        oper_y = self.operands[1]),
            3 => write!(f, "{mnemonic:width$} {oper_x},{oper_y},{oper_z}",
                        width = MAX_MNEMONIC_LEN,
                        mnemonic = self.operator.mnemonic,
                        oper_x = self.operands[0],
                        oper_y = self.operands[1],
                        oper_z = self.operands[2]),

            _ => unimplemented!(),
        }
    }
}
