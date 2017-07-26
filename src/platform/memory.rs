// pub const FRAME_SIZE:   usize = 16;
pub const STACK_FRAMES: usize = 16;
pub type Frame = u16;
pub type Stack = [Frame; STACK_FRAMES];

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
