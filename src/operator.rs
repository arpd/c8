use instruction::{Operand};
use platform::{Chip8Context};
use platform::memory::{Cell};
use std::mem::{size_of};

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
    if let Operand::Register(index) = operands[0] {
        let start_addr: u16 = ctxt.cpu.i;
        for i in 0..index {
            let offset = i as u16 * size_of::<Cell>() as u16;
            let reg_val: u8 = ctxt.cpu.gp_registers[i as usize];
            ctxt.ram.write(start_addr + offset, reg_val);
        }
    } else {
        unimplemented!()
    }
}

pub fn load_regs(operands: &Vec<Operand>, ctxt: &mut Chip8Context) {
    if let Operand::Register(index) = operands[0] {
        let start_addr: u16 = ctxt.cpu.i;
        for i in 0..index {
            let offset = i as u16 * size_of::<Cell>() as u16;
            let cell_val = ctxt.ram.read(start_addr + offset);
            ctxt.cpu.gp_registers[i as usize] = cell_val;
        }
    } else {
        unimplemented!()
    }
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

#[cfg(test)]
mod tests {
    use platform::{Chip8Context};
    use instruction::{Instruction};

    fn print_mem(ctxt: &Chip8Context, start_addr: u16, num_cells: u16) {
        println!("");
        println!("{} cells starting at @{:x}:", num_cells, start_addr);
        for i in 0..num_cells {
            println!("@{:04x}: 0x{:04x}", start_addr + i, ctxt.ram.read(start_addr + i));
        }
        println!("---");
    }
    
    fn dump_regs_setup() -> Chip8Context {
        let mut ctxt = Chip8Context::new();
        let addr_begin = 0xded;
        // Fill some interesting values in the registers, and make sure these cells
        // are only set to 0 at the moment; If they're not there's some funny
        // business going on!
        for i in 0..0xf {
            ctxt.cpu.gp_registers[i as usize] = i as u8;
            assert_eq!(ctxt.ram.read(addr_begin + i as u16), 0);
        }
        ctxt.cpu.i = addr_begin as u16;

        ctxt
    }

    #[test]
    fn dump_regs() {
        let mut ctxt = dump_regs_setup();
        let addr_begin = ctxt.cpu.i;

        // Now construct this instruction, and execute it on our context
        // The dump registers instruction has the form `0xF_55`, where _ represents the
        // final register to dump
        let dump_regs_insn = Instruction::new(0xff55);
        let operands = dump_regs_insn.operands.clone();
        let impl_fn = dump_regs_insn.operator.implementation;

        //print_mem(&ctxt, addr_begin as u16, 0x10);
        impl_fn(&operands, &mut ctxt);
        //print_mem(&ctxt, addr_begin as u16, 0x10);

        // Finally, let's verify the results
        for i in 0..0xf {
            let v = ctxt.ram.read(addr_begin + i);
            assert_eq!(v, i as u8)
        }
    }

    fn load_regs_setup() -> Chip8Context {
        let mut ctxt = Chip8Context::new();
        let addr_begin = 0xded;
        // Fill some interesting values in to ram, and make sure these registers
        // are only set to 0 at the moment; If they're not there's some funny
        // business going on!
        for i in 0..0xf {
            assert_eq!(ctxt.ram.write(addr_begin + i as u16, i), i);
            assert_eq!(ctxt.cpu.gp_registers[i as usize], 0);
        }
        ctxt.cpu.i = addr_begin;

        ctxt
    }

    #[test]
    fn load_regs() {
        let mut ctxt = load_regs_setup();
        let addr_begin = ctxt.cpu.i;

        // Now construct this instruction, and execute it on our context
        // The load registers instruction has the form `0xF_65`, where _ represents the
        // final register to dump
        let load_regs_insn = Instruction::new(0xff65);
        let operands = load_regs_insn.operands.clone();
        let impl_fn = load_regs_insn.operator.implementation;

        impl_fn(&operands, &mut ctxt);

        // Finally, let's verify the results
        for i in 0..0xf {
            let v = ctxt.cpu.gp_registers[i as usize];
            println!("Reg {:02x} = {:x}", i, v);
            assert_eq!(i,v);
        }
    }
    
}
