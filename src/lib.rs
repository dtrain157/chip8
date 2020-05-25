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
        let opcode = self.memory.read_word(self.cpu.pc as usize).unwrap();
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

    pub fn get_pc(&self) -> u16 {
        self.cpu.pc
    }

    pub fn get_v0(&self) -> u8 {
        self.cpu.v[0]
    }

    pub fn get_v1(&self) -> u8 {
        self.cpu.v[1]
    }

    pub fn get_v2(&self) -> u8 {
        self.cpu.v[2]
    }

    pub fn get_v3(&self) -> u8 {
        self.cpu.v[3]
    }

    pub fn get_v4(&self) -> u8 {
        self.cpu.v[4]
    }

    pub fn get_v5(&self) -> u8 {
        self.cpu.v[5]
    }

    pub fn get_v6(&self) -> u8 {
        self.cpu.v[6]
    }

    pub fn get_v7(&self) -> u8 {
        self.cpu.v[7]
    }

    pub fn get_v8(&self) -> u8 {
        self.cpu.v[8]
    }

    pub fn get_v9(&self) -> u8 {
        self.cpu.v[9]
    }

    pub fn get_va(&self) -> u8 {
        self.cpu.v[0xA]
    }

    pub fn get_vb(&self) -> u8 {
        self.cpu.v[0xB]
    }

    pub fn get_vc(&self) -> u8 {
        self.cpu.v[0xC]
    }

    pub fn get_vd(&self) -> u8 {
        self.cpu.v[0xD]
    }

    pub fn get_ve(&self) -> u8 {
        self.cpu.v[0xE]
    }

    pub fn get_vf(&self) -> u8 {
        self.cpu.v[0xF]
    }
}
