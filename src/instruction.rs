use std::fmt;
use decoder::*;
use operator::{Operator};

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
            Operand::Register(0xf) => write!(f, "I"),
            Operand::Register(reg) => write!(f, "V{}", reg),
            Operand::Address(addr) => write!(f, "@0x{:x}", addr),
            Operand::Literal(lit)  => write!(f, "$0x{:x}", lit)
        }
    }
}

#[derive(Clone)]
pub struct Instruction {
    src: u16,
    operands: Vec<Operand>,
    //operator: fn(operands: &Vec<Operand>, ctxt: &mut Chip8Context),
    operator: Operator,
}

// get a nibble (i.e. 4-bit value) from a `u16`, indexed from right to left
// (0..3), `pos = 0` will return `src & 0xf`
#[inline]
fn get_nibble(src: u16, pos: u8) -> u8 {
    assert!(pos <= 3);
    ((src & (0xf << pos*4)) >> pos*4) as u8
}

impl Instruction {
    pub fn new(src: u16) -> Instruction {
        // First of all, let's extract the operands that this instruction
        // requires.
        let mut operands: Vec<Operand> = Vec::<Operand>::new();
        let operator = match get_nibble(src, 3) {
            0 => {
                if src != 0x00ee && src != 0x00e0 {
                    let mut addr: u16 = get_nibble(src, 0) as u16;
                    addr |= (get_nibble(src, 1) as u16) << 1;
                    addr |= (get_nibble(src, 2) as u16) << 2;
                    operands.push(Operand::Address(addr));
                }
                decode_special(src)  
            },
            1 | 2 | 0xa | 0xb => {
                // _NNN
                let mut addr: u16 = get_nibble(src, 0) as u16;
                addr |= (get_nibble(src, 1) as u16) << 1;
                addr |= (get_nibble(src, 2) as u16) << 2;
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
                let register_x = get_nibble(src, 2);
                let register_y = get_nibble(src, 1);
                let literal = get_nibble(src, 0) as u16;
                operands.push(Operand::Register(register_x));
                operands.push(Operand::Register(register_y));
                operands.push(Operand::Literal(literal));
                decode_regx_regy_addr4(src)
            }
            0xe | 0xf => {
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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;
        match self.operands.len() {
            0 => write!(f, "{}", self.operator.mnemonic),
            1 => write!(f, "{} {}", self.operator.mnemonic,
                        self.operands[0]), 
            2 => write!(f, "{} {},{}", self.operator.mnemonic,
                        self.operands[0], self.operands[1]), 
            3 => write!(f, "{} {},{}", self.operator.mnemonic,
                        self.operands[0], self.operands[1]), 
            _ => unimplemented!(),
        }
    }
}
