mod cpu;
pub mod memory;
mod display;
mod sound;
mod input;

use self::cpu::{Cpu};
use self::memory::{Ram};

pub struct Chip8Context {
    pub cpu: Cpu,
    pub ram: Ram,
}

impl Chip8Context {
    pub fn new() -> Chip8Context {
        Chip8Context {
            cpu: Cpu::new(),
            ram: Ram::new(),
        }
    }
}

