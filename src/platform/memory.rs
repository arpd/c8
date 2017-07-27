use std::mem::{size_of};

pub const STACK_FRAMES: usize = 16;
pub type Addr = u16;

pub struct Stack {
    raw: [Addr; STACK_FRAMES],
    ptr: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            raw: [0; STACK_FRAMES],
            ptr: 0,
        }
    }
    
    pub fn pop(&mut self) -> Addr {
        assert!(self.ptr >= 1);
        let rv = self.raw[self.ptr as usize];
        assert_eq!(rv % size_of::<Addr>() as Addr, 0);
        self.ptr -= 1;
        rv
    }
    
    pub fn push(&mut self, addr: Addr) {
        assert!(self.ptr < (STACK_FRAMES - 1));
        assert_eq!(addr % size_of::<Addr>() as Addr, 0);
        self.ptr += 1;
        self.raw[self.ptr as usize] = addr;
    }
}

// pub const CELL_SIZE:    usize = 8;
pub const RAM_CELLS:    usize = 0x1000;
pub type Cell = u8;

pub struct Ram {
    pub cell: [Cell; RAM_CELLS],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            cell: [0; RAM_CELLS],
        }
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        self.cell[addr as usize]
    }
    
    pub fn write(&mut self, addr: u16, val: u8) -> u8 {
        self.cell[addr as usize] = val;
        self.read(addr)
    }
}
