pub mod cpu;
pub mod stack;

use cpu::{CPUError, CPU};
use stack::{Stack, StackError};

struct Display {
    memory: [u8; 2048],
}

impl Display {
    fn clear_momory(&mut self) {
        self.memory = [0; 2048];
    }
}

struct Memory {
    memory: [u8; 4096],
}

impl Memory {
    fn clear(&mut self) {
        self.memory = [0; 4096];
    }

    fn read(&self, index: u16) -> u16 {
        ((self.memory[index as usize] as u16) << 8) | (self.memory[(index + 1) as usize] as u16)
    }
}

struct Chip8 {
    cpu: CPU,
    memory: Memory,
    display: Display,
}

impl Chip8 {
    pub fn execute_cycle(&mut self) {
        let opcode = self.memory.read(self.cpu.pc);
        self.cpu.process_opcode(opcode, &mut self.display);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_clear() {
        let mut mem = Memory { memory: [1; 4096] };
        mem.clear();
        let t1 = mem.memory;
        let t2 = [0; 4096];
        assert_eq!(t1.len(), t2.len());
        assert!(t1.iter().zip(t2.iter()).all(|(a, b)| a == b));
    }

    #[test]
    fn memory_read() {
        let mut mem = Memory { memory: [1; 4096] };
        mem.memory[1] = 2 as u8;
        let pos0 = mem.read(0);
        assert_eq!(pos0, 0x0102);
    }
}
