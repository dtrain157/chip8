pub mod cpu;
pub mod display;
pub mod keyboard;
pub mod memory;
pub mod stack;

use cpu::CPU;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;

#[allow(dead_code)]
struct Chip8 {
    cpu: CPU,
    memory: Memory,
    display: Display,
    keyboard: Keyboard,
}

impl Chip8 {
    #[allow(dead_code)]
    pub fn power_up() -> Self {
        Chip8 {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }

    #[allow(dead_code)]
    pub fn execute_cycle(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let opcode = self.memory.read_word(self.cpu.pc as usize)?;
        self.cpu.process_opcode(opcode, &mut self.display, &mut self.memory, &self.keyboard)?;
        Ok(())
    }
}
