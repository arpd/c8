use platform::memory::{Stack};

pub struct Cpu {
    pub gp_registers: [u8; 16],
    pub i: u16,
    pub program_counter: u16,   
    pub stack: Stack,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            gp_registers: [0; 16],
            i: 0 as u16,
            program_counter: 0 as u16,
            stack: Stack::new(),
        }
    }
}
