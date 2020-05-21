use super::display::Display;
use super::memory::{Memory, MemoryError};
use super::stack::{Stack, StackError};
use rand::Rng;
use std::error;
use std::fmt;

const REGISTER_COUNT: usize = 16;
pub struct CPU {
    //program counter
    pub pc: u16,
    //data registers
    v: [u8; REGISTER_COUNT],
    //address register
    i: u16,
    //timers
    delay_timer: u8,
    sound_timer: u8,
    //stack
    stack: Stack,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0x200,
            v: [0; REGISTER_COUNT],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: Stack::new(),
        }
    }

    pub fn process_opcode(&mut self, opcode: u16, display: &mut Display, memory: &mut Memory) -> Result<(), CPUError> {
        let mut should_update_pc_after_processing = true;

        //get typical opcode values from opcode
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        let nibble1 = (opcode & 0xF000) >> 12;
        let nibble2 = (opcode & 0x0F00) >> 8;
        let nibble3 = (opcode & 0x00F0) >> 4;
        let nibble4 = opcode & 0x000F;

        match (nibble1, nibble2, nibble3, nibble4) {
            //CLS
            (0x0, 0x0, 0xE, 0x0) => display.clear_momory(),
            //RET
            (0x0, 0x0, 0xE, 0xE) => {
                self.pc = match self.stack.pop() {
                    Ok(val) => val - 2,
                    Err(e) => return Err(CPUError::ErrorAccessingStack(e)),
                };
            }
            //JP addr
            (0x1, _, _, _) => {
                self.pc = nnn;
                should_update_pc_after_processing = false;
            }
            //CALL addr
            (0x2, _, _, _) => {
                match self.stack.push(self.pc) {
                    Ok(_) => {}
                    Err(e) => return Err(CPUError::ErrorAccessingStack(e)),
                }
                should_update_pc_after_processing = false;
                self.pc = nnn;
            }
            //SE Vx byte
            (0x3, _, _, _) => {
                if self.v[x] == kk {
                    self.pc = self.pc + 2;
                }
            }
            //SNE Vx byte
            (0x4, _, _, _) => {
                if self.v[x] != kk {
                    self.pc = self.pc + 2;
                }
            }
            //SE Vx Vy
            (0x5, _, _, 0x0) => {
                if self.v[x] == self.v[y] {
                    self.pc = self.pc + 2;
                }
            }
            //LD Vx byte
            (0x6, _, _, _) => self.v[x] = kk,
            //ADD Vx byte
            (0x7, _, _, _) => self.v[x] = self.v[x] + kk,
            //LD Vx Vy
            (0x8, _, _, 0x0) => self.v[x] = self.v[y],
            //OR Vx Vy
            (0x8, _, _, 0x1) => self.v[x] = self.v[x] | self.v[y],
            //AND Vx Vy
            (0x8, _, _, 0x2) => self.v[x] = self.v[x] & self.v[y],
            //XOR Vx Vy
            (0x8, _, _, 0x3) => self.v[x] = self.v[x] ^ self.v[y],
            //ADD Vx Vy
            (0x8, _, _, 0x4) => {
                let res = (self.v[x] as u16) + (self.v[y] as u16);
                if res > 255 {
                    self.v[0xF as usize] = 1;
                }
                self.v[x] = (res & 0xFF) as u8;
            }
            //SUB Vx Vy
            (0x8, _, _, 0x5) => {
                if self.v[x] > self.v[y] {
                    self.v[0xF] = 1
                } else {
                    self.v[0xF] = 0
                }
                self.v[x] = self.v[x] - self.v[y];
            }
            //SHR Vx
            (0x8, _, _, 0x6) => {
                if self.v[x] & 0x1 == 0x1 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = self.v[x] >> 1;
            }
            //SUBN Vx Vy
            (0x8, _, _, 0x7) => {
                if self.v[y] > self.v[x] {
                    self.v[0xF] = 1
                } else {
                    self.v[0xF] = 0
                }
                self.v[x] = self.v[y] - self.v[x];
            }
            //SHL Vx
            (0x8, _, _, 0xE) => {
                if self.v[x] & 0x80 == 0x80 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = self.v[x] << 1;
            }
            //SNE Vx, Vy
            (0x9, _, _, 0x0) => {
                if self.v[x] != self.v[y] {
                    self.pc = self.pc + 2;
                }
            }
            //LD I addr
            (0xA, _, _, _) => self.i = nnn,
            //JP V0, addr
            (0xB, _, _, _) => self.pc = (self.v[0] as u16) + nnn,
            //RND Vx byte
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();
                let n1: u8 = rng.gen();
                self.v[x] = n1 & kk;
            }
            //DRW Vx Vy
            (0xD, _, _, _) => {
                let data = match memory.read_multiple_bytes(self.i as usize, n) {
                    Ok(data) => data,
                    Err(e) => return Err(CPUError::ErrorAccessingMemory(e)),
                };
                //implement display logic here
                todo! {}
            }
            //SKP Vx
            (0xE, _, 0x9, 0xE) => {
                //implement input logic here
                todo! {}
            }
            //SKNP Vx
            (0xE, _, 0xA, 0x1) => {
                //implement input logic here
                todo! {}
            }
            //LD Vx DT
            (0xF, _, 0x0, 0x7) => self.v[x] = self.delay_timer,
            //LD Vx K
            (0xF, _, 0x0, 0xA) => {
                //implement input logic here
                todo! {}
            }
            //LD DT Vx
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.v[x],
            //LD ST Vx
            (0xF, _, 0x1, 0x8) => self.sound_timer = self.v[x],
            //ADD I Vx
            (0xF, _, 0x1, 0xE) => self.i = self.i + (self.v[x] as u16),
            //LD F Vx
            (0xF, _, 0x2, 0x9) => {
                //implement display logic here
                todo! {}
            }
            //LD B Vx
            (0xF, _, 0x3, 0x3) => {
                let hundreds = self.v[x] % 100;
                let tens = (self.v[x] - hundreds * 100) % 10;
                let ones = self.v[x] - hundreds * 100 - tens * 10;
                match memory.write_byte(self.i as usize, hundreds) {
                    Ok(_) => {}
                    Err(e) => return Err(CPUError::ErrorAccessingMemory(e)),
                }
                match memory.write_byte((self.i + 1) as usize, tens) {
                    Ok(_) => {}
                    Err(e) => return Err(CPUError::ErrorAccessingMemory(e)),
                }
                match memory.write_byte((self.i + 2) as usize, ones) {
                    Ok(_) => {}
                    Err(e) => return Err(CPUError::ErrorAccessingMemory(e)),
                }
            }
            //LD I Vx
            (0xF, _, 0x5, 0x5) => {
                for j in 0..REGISTER_COUNT - 1 {
                    match memory.write_byte((self.i as usize) + j, self.v[j]) {
                        Ok(_) => {}
                        Err(e) => return Err(CPUError::ErrorAccessingMemory(e)),
                    }
                }
            }
            //LD Vx I
            (0xF, _, 0x6, 0x5) => {
                for j in 0..REGISTER_COUNT - 1 {
                    self.v[j] = match memory.read_byte((self.i as usize) + j) {
                        Ok(byte) => byte,
                        Err(e) => return Err(CPUError::ErrorAccessingMemory(e)),
                    }
                }
            }
            _ => return Err(CPUError::InvalidOpcodeEncountered(opcode, self.pc)),
        }

        if should_update_pc_after_processing {
            self.pc = self.pc + 2;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum CPUError {
    ErrorAccessingStack(StackError),
    ErrorAccessingMemory(MemoryError),
    InvalidOpcodeEncountered(u16, u16),
}

impl fmt::Display for CPUError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CPUError::ErrorAccessingStack(ref e) => e.fmt(f),
            CPUError::ErrorAccessingMemory(ref e) => e.fmt(f),
            CPUError::InvalidOpcodeEncountered(opcode, addr) => {
                write!(f, "Unknown opcode encountered as addr {:#04X}: {:#04X}", opcode, addr)
            }
        }
    }
}

impl error::Error for CPUError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            CPUError::ErrorAccessingStack(ref e) => Some(e),
            CPUError::ErrorAccessingMemory(ref e) => Some(e),
            CPUError::InvalidOpcodeEncountered(_opcode, _addr) => None,
        }
    }
}

#[cfg(test)]
mod cpu_tests {
    use super::*;

    #[test]
    fn cpu_call_ret() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();

        let opcode = 0x2111; //call function at addr 0x111
        cpu.process_opcode(opcode, &mut display, &mut memory).unwrap();

        assert_eq!(cpu.pc, 0x111);

        let opcode = 0x2222; //call function at addr 0x222
        cpu.process_opcode(opcode, &mut display, &mut memory).unwrap();

        assert_eq!(cpu.pc, 0x222);

        let opcode = 0x00EE; //return from first function
        cpu.process_opcode(opcode, &mut display, &mut memory).unwrap();

        assert_eq!(cpu.pc, 0x111);

        let opcode = 0x00EE; //return from second function
        cpu.process_opcode(opcode, &mut display, &mut memory).unwrap();

        assert_eq!(cpu.pc, 0x200);
    }

    #[test]
    fn cpu_jmp() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();

        let opcode = 0x1859; //call function at addr 0x111
        cpu.process_opcode(opcode, &mut display, &mut memory).unwrap();

        assert_eq!(cpu.pc, 0x859);
    }

    #[test]
    fn cpu_ld_vx_byte_se() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        assert_eq!(cpu.pc, 0x200);

        let opcode = 0x6822; //load 0x22 into v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory).unwrap();
        assert_eq!(cpu.v[8], 0x22);
        assert_eq!(cpu.pc, 0x202);

        let opcode = 0x3822; //skip the next instruction (condition v[8] = 0x22 is true)
        cpu.process_opcode(opcode, &mut display, &mut memory).unwrap();
        assert_eq!(cpu.pc, 0x206);
    }
}
