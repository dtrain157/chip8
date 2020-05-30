pub mod cpu;
pub mod display;
pub mod keyboard;
pub mod memory;
pub mod stack;

use cpu::CPU;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8 {
    cpu: CPU,
    memory: Memory,
    display: Display,
    keyboard: Keyboard,
}

#[wasm_bindgen]
impl Chip8 {
    pub fn power_up() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        Chip8 {
            cpu: CPU::new(),
            memory: Memory::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
        }
    }

    // TODO: Fix error handling
    // Unwrapping for now, 'til I can work out how to pass custom errors through the wasm boundry
    pub fn execute_cycle(&mut self) {
        let pc = self.cpu.get_pc() as usize;
        let opcode = self.memory.read_word(pc).unwrap();
        self.cpu.process_opcode(opcode, &mut self.display, &mut self.memory, &self.keyboard).unwrap();
    }

    pub fn get_display_width(&self) -> usize {
        display::COLUMNS
    }

    pub fn get_display_height(&self) -> usize {
        display::ROWS
    }

    pub fn get_display_memory(&self) -> *const u8 {
        self.display.memory.as_ptr()
    }

    pub fn get_memory_size(&self) -> usize {
        memory::MEMORY_SIZE
    }

    pub fn get_memory(&self) -> *const u8 {
        self.memory.memory.as_ptr()
    }

    pub fn get_pc(&self) -> u16 {
        self.cpu.get_pc()
    }

    pub fn get_i(&self) -> u16 {
        self.cpu.get_i()
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.cpu.get_delay_timer()
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.cpu.get_sound_timer()
    }

    pub fn clear_control_registers(&mut self) {
        self.cpu.clear_pc();
        self.cpu.clear_i();
        self.cpu.clear_delay_timer();
        self.cpu.clear_sound_timer();
    }

    pub fn decrement_timers(&mut self) {
        self.cpu.decrement_timers();
    }

    pub fn get_v_registers(&self) -> *const u8 {
        self.cpu.get_v_registers().as_ptr()
    }
}
