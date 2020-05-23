use std::error;
use std::fmt;

pub const MEMORY_SIZE: usize = 4096; //support 4k of memory

pub const FONT_CHARACTERS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //"0" Character
    0x20, 0x60, 0x20, 0x20, 0x70, //"1" Character
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //"2" Character
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //"3" Character
    0x90, 0x90, 0xF0, 0x10, 0x10, //"4" Character
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //"5" Character
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //"6" Character
    0xF0, 0x10, 0x20, 0x40, 0x40, //"7" Character
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //"8" Character
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //"9" Character
    0xF0, 0x90, 0xF0, 0x90, 0x90, //"A" Character
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //"B" Character
    0xF0, 0x80, 0x80, 0x80, 0xF0, //"C" Character
    0xE0, 0x90, 0x90, 0x90, 0xE0, //"D" Character
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //"E" Character
    0xF0, 0x80, 0xF0, 0x80, 0x80, //"F" Character
];

const FONT_CHARACTER_SIZE: usize = 5; //size in bytes

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let mut mem = Memory { memory: [0; MEMORY_SIZE] };
        mem.clear();
        mem
    }

    pub fn clear(&mut self) {
        self.memory = [0; MEMORY_SIZE];
        for i in 0..FONT_CHARACTERS.len() {
            self.memory[i] = FONT_CHARACTERS[i];
        }
    }

    pub fn read_word(&self, index: usize) -> Result<u16, MemoryError> {
        let first_byte = self.memory.get(index);
        let second_byte = self.memory.get(index + 1);

        match (first_byte, second_byte) {
            (Some(b1), Some(b2)) => {
                let data = ((*b1 as u16) << 8) | (*b2 as u16);
                return Ok(data);
            }
            _ => return Err(MemoryError::InvalidAddress(index)),
        }
    }

    pub fn read_byte(&self, index: usize) -> Result<u8, MemoryError> {
        let byte = self.memory.get(index);

        match byte {
            Some(byte) => Ok(*byte),
            _ => return Err(MemoryError::InvalidAddress(index)),
        }
    }

    pub fn read_multiple_bytes(&self, index: usize, bytes: u8) -> Result<&[u8], MemoryError> {
        let from = index;
        let to = index + (bytes as usize);
        if (from + to) >= MEMORY_SIZE {
            return Err(MemoryError::InvalidAddress(from + to));
        }
        Ok(&self.memory[from..to])
    }

    pub fn write_byte(&mut self, index: usize, byte: u8) -> Result<(), MemoryError> {
        if (index as usize) >= MEMORY_SIZE {
            return Err(MemoryError::InvalidAddress(index));
        }
        self.memory[index as usize] = byte;
        Ok(())
    }

    pub fn get_location_of_font_character(&self, character: u8) -> usize {
        (character as usize) * FONT_CHARACTER_SIZE
    }
}

#[derive(Debug)]
pub enum MemoryError {
    InvalidAddress(usize),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MemoryError::InvalidAddress(addr) => write!(f, "Tried to read from an invalid address in the memory: {:#04X}!", addr),
        }
    }
}

impl error::Error for MemoryError {}

#[cfg(test)]
mod memory_tests {
    use super::*;

    #[test]
    fn memory_read_word() {
        let mut mem = Memory::new();
        mem.memory[0] = 1 as u8;
        mem.memory[1] = 2 as u8;
        let data = mem.read_word(0).unwrap();
        assert_eq!(data, 0x102);
    }

    #[test]
    fn memory_read_multiple_bytes() {
        let mut mem = Memory::new();
        mem.memory[1] = 2 as u8;
        mem.memory[2] = 3 as u8;
        mem.memory[3] = 4 as u8;
        mem.memory[4] = 5 as u8;

        let expected_result = vec![240, 2, 3, 4, 5];
        let actual_result = mem.read_multiple_bytes(0, 5).unwrap();
        assert!(expected_result.len() == actual_result.len() && expected_result == actual_result);
    }

    #[test]
    fn memory_write_byte() {
        let mut mem = Memory::new();
        mem.write_byte(1, 0x5).unwrap();
        mem.write_byte(2, 0x6).unwrap();
        let data1 = mem.read_byte(1).unwrap();
        let data2 = mem.read_byte(2).unwrap();
        assert_eq!(data1, 0x5);
        assert_eq!(data2, 0x6);
    }
}
