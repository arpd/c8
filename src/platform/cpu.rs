use platform::memory::{Stack, STACK_FRAMES};

pub struct Cpu {
    pub gp_registers: [u8; 16],
    pub i: u16,
    stack_pointer: u8,
    program_counter: u16,   
    stack: Stack,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            gp_registers: [0; 16],
            i: 0 as u16,
            stack_pointer: 0 as u8,
            program_counter: 0 as u16,
            stack: [0; STACK_FRAMES],
        }
    }
}
