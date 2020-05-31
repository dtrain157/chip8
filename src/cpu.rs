use super::display::Display;
use super::keyboard::Keyboard;
use super::memory::{Memory, MemoryError};
use super::stack::{Stack, StackError};
use rand::prelude::*;
use std::error;
use std::fmt;

const REGISTER_COUNT: usize = 16;
pub struct CPU {
    //program counter
    pc: u16,
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

    pub fn process_opcode(&mut self, opcode: u16, display: &mut Display, memory: &mut Memory, keyboard: &Keyboard) -> Result<(), CPUError> {
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
            (0x0, 0x0, 0xE, 0x0) => display.clear(),
            //RET
            (0x0, 0x0, 0xE, 0xE) => {
                self.pc = match self.stack.pop() {
                    Ok(val) => val,
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
                let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = res;
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
                let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[x] = res;
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
                let n1: u8 = random();
                self.v[x] = n1 & kk;
            }
            //DRW Vx Vy
            (0xD, _, _, _) => {
                let data = match memory.read_multiple_bytes(self.i as usize, n) {
                    Ok(data) => data,
                    Err(e) => return Err(CPUError::ErrorAccessingMemory(e)),
                };
                let location = (self.v[x] as usize, self.v[y] as usize);
                println!("LOCATION>>>> {:?}", location);
                println!("DATA>>>> {:?}", data);
                self.v[0xF] = display.draw(data, location) as u8;
            }
            //SKP Vx
            (0xE, _, 0x9, 0xE) => match keyboard.get_key_pressed() {
                Some(key) => {
                    if self.v[x] == key {
                        self.pc = self.pc + 2;
                    }
                }
                None => {}
            },
            //SKNP Vx
            (0xE, _, 0xA, 0x1) => match keyboard.get_key_pressed() {
                Some(key) => {
                    if self.v[x] != key {
                        self.pc = self.pc + 2;
                    }
                }
                None => {}
            },
            //LD Vx DT
            (0xF, _, 0x0, 0x7) => self.v[x] = self.delay_timer,
            //LD Vx K
            (0xF, _, 0x0, 0xA) => match keyboard.get_key_pressed() {
                Some(key) => self.v[x] = key,
                None => should_update_pc_after_processing = false,
            },
            //LD DT Vx
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.v[x],
            //LD ST Vx
            (0xF, _, 0x1, 0x8) => self.sound_timer = self.v[x],
            //ADD I Vx
            (0xF, _, 0x1, 0xE) => self.i = self.i + (self.v[x] as u16),
            //LD F Vx
            (0xF, _, 0x2, 0x9) => self.i = memory.get_location_of_font_character(self.v[x]) as u16,
            //LD B Vx
            (0xF, _, 0x3, 0x3) => {
                let hundreds = self.v[x] / 100;
                let tens = (self.v[x] - hundreds * 100) / 10;
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
                for j in 0..(x+1) {
                    match memory.write_byte((self.i as usize) + j, self.v[j]) {
                        Ok(_) => {}
                        Err(e) => return Err(CPUError::ErrorAccessingMemory(e)),
                    }
                }
            }
            //LD Vx I
            (0xF, _, 0x6, 0x5) => {
                for j in 0..(x+1) {
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

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_v_registers(&self) -> &[u8] {
        &self.v[..]
    }

    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn clear_pc(&mut self) {
        self.pc = 0x200;
    }

    pub fn clear_i(&mut self) {
        self.i = 0;
    }

    pub fn clear_delay_timer(&mut self) {
        self.delay_timer = 0;
    }

    pub fn clear_sound_timer(&mut self) {
        self.sound_timer = 0;
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer - 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer = self.sound_timer - 1;
        }
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
            CPUError::InvalidOpcodeEncountered(opcode, addr) => write!(f, "Unknown opcode encountered as addr {:#04X}: {:#04X}", opcode, addr),
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
        let keyboard = Keyboard::new();

        let opcode = 0x2111; //call function at addr 0x111
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.pc, 0x111);

        let opcode = 0x2222; //call function at addr 0x222
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.pc, 0x222);

        let opcode = 0x00EE; //return from first function
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.pc, 0x113);

        let opcode = 0x00EE; //return from second function
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn cpu_jmp() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x1859; //call function at addr 0x111
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.pc, 0x859);
    }

    #[test]
    fn cpu_ld_vx_byte_se_sne() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();
        assert_eq!(cpu.pc, 0x200);

        let opcode = 0x6822; //load 0x22 into v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[8], 0x22);
        assert_eq!(cpu.pc, 0x202);

        let opcode = 0x3822; //skip the next instruction (condition v[8] = 0x22 is true)
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.pc, 0x206);

        let opcode = 0x3821; //do not skip the next instruction (condition v[8] = 0x21 is false)
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.pc, 0x208);
    }

    #[test]
    fn cpu_se_vx_vy() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6822; //load 0x22 into v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[8], 0x22);
        assert_eq!(cpu.pc, 0x202);

        let opcode = 0x6922; //load 0x22 into v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[9], 0x22);
        assert_eq!(cpu.pc, 0x204);

        let opcode = 0x5890; //skip the next instruction (condition v[8] = v[9] is true)
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.pc, 0x208);
    }

    #[test]
    fn cpu_add_vx_byte() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6822; //load 0x22 into v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[8], 0x22);
        assert_eq!(cpu.pc, 0x202);

        let opcode = 0x7822; //add 0x22 to v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[8], 0x44);
        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn cpu_ld_vx_vy() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6822; //load 0x22 into v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[8], 0x22);
        assert_eq!(cpu.pc, 0x202);

        let opcode = 0x8980; //set v[9] equal to v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[9], 0x22);
        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    fn cpu_or_vx_vy() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6101; //load 0x01 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x62AA; //load 0xAA into v[2]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x8121; //or v[1] and v[2]; store the result in v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0xAB);
    }

    #[test]
    fn cpu_and_vx_vy() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6102; //load 0x02 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x62AA; //load 0xAA into v[2]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x8122; //and v[1] and v[2]; store the result in v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x02);
    }

    #[test]
    fn cpu_xor_vx_vy() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x61FA; //load 0xFA into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x62AA; //load 0xAA into v[2]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x8123; //xor v[1] and v[2]; store the result in v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x50);
    }

    #[test]
    fn cpu_add_vx_vy() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6101; //load 0x01 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x6201; //load 0x01 into v[2]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x63FF; //load 0xFF into v[3]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x8124; //add v[1] and v[2]; store the result in v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x02);
        assert_eq!(cpu.v[0xF], 0);

        let opcode = 0x6101; //load 0x01 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x8134; //add v[1] and v[3]; store the result in v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x00);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn cpu_sub_vx_vy() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6102; //load 0x02 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x6201; //load 0x01 into v[2]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x8125; //subtract v[2] from v[1], store the result in v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x01);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn cpu_shr_vx() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6110; //load 0x10 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x8106; //shift v[1] right
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x08);
        assert_eq!(cpu.v[0xF], 0);

        let opcode = 0x6181; //load 0x81 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x8106; //shift v[1] right
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x40);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn cpu_subn_vx_vy() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6101; //load 0x01 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x6202; //load 0x02 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x8127; //subtract v[1] from v[2], store the result in v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x01);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn cpu_shl_vx() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6110; //load 0x10 into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x810E; //shift v[1] right
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x20);
        assert_eq!(cpu.v[0xF], 0);

        let opcode = 0x61AA; //load 0xAA into v[1]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        let opcode = 0x810E; //shift v[1] right
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();

        assert_eq!(cpu.v[1], 0x54);
        assert_eq!(cpu.v[0xF], 1);
    }

    #[test]
    fn cpu_sne_vx_vy() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6822; //load 0x22 into v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[8], 0x22);
        assert_eq!(cpu.pc, 0x202);

        let opcode = 0x6923; //load 0x23 into v[8]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[9], 0x23);
        assert_eq!(cpu.pc, 0x204);

        let opcode = 0x9890; //skip the next instruction (condition v[8] != v[9] is true)
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.pc, 0x208);
    }

    #[test]
    fn cpu_ld_i_addr() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0xA1AF; //load the value 0x1AF into I
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.i, 0x01AF);
    }

    #[test]
    fn cpu_jp_vo_addr() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6022; //load 0x22 into v[0]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0], 0x22);

        let opcode = 0xB1AF; //jump to address v[0] + 0x1AF (0x22 + 0x1AF = 0x1D1)
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.pc, 0x1D3);
    }

    // This test might fail, if the random number generated happens to be 0x22.
    // There is only a 1/256 chance of this happening, though, so most of the time
    // the test should succeed. If it fails in successive test runs, there is probably
    // an issue.
    #[test]
    fn cpu_rnd_vx_byte() {
        let mut cpu = CPU::new();
        let mut display = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6022; //load 0x22 into v[0]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0], 0x22);

        let opcode = 0xC0FF; //save a random number into v[0]
        cpu.process_opcode(opcode, &mut display, &mut memory, &keyboard).unwrap();
        assert_ne!(cpu.v[0], 0x22);
    }

    #[test]
    fn cpu_drw_vx_vy_n() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6000; //load 0x00 into v[0]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0], 0x00);

        let opcode = 0x6100; //load 0x00 into v[1]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[1], 0x00);

        let opcode = 0xA000; //load 0x000 into I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.i, 0x000);

        let opcode = 0xD015; //Draw 5 bytes onto the display at (0,0). This is expected to draw the "0" character onto the display.
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();

        let display_byte1 = &disp.memory[0..8];
        let display_byte2 = &disp.memory[crate::display::COLUMNS..crate::display::COLUMNS + 8];
        let display_byte3 = &disp.memory[crate::display::COLUMNS * 2..(crate::display::COLUMNS * 2) + 8];
        let display_byte4 = &disp.memory[crate::display::COLUMNS * 3..(crate::display::COLUMNS * 3) + 8];
        let display_byte5 = &disp.memory[crate::display::COLUMNS * 4..(crate::display::COLUMNS * 4) + 8];

        let expected_byte1 = vec![1, 1, 1, 1, 0, 0, 0, 0];
        let expected_byte2 = vec![1, 0, 0, 1, 0, 0, 0, 0];
        let expected_byte3 = vec![1, 0, 0, 1, 0, 0, 0, 0];
        let expected_byte4 = vec![1, 0, 0, 1, 0, 0, 0, 0];
        let expected_byte5 = vec![1, 1, 1, 1, 0, 0, 0, 0];


        assert!(expected_byte1.len() == display_byte1.len() && expected_byte1 == display_byte1);
        assert!(expected_byte2.len() == display_byte2.len() && expected_byte2 == display_byte2);
        assert!(expected_byte3.len() == display_byte3.len() && expected_byte3 == display_byte3);
        assert!(expected_byte4.len() == display_byte4.len() && expected_byte4 == display_byte4);
        assert!(expected_byte5.len() == display_byte5.len() && expected_byte5 == display_byte5);
    }

    #[test]
    fn cpu_drw_vx_vy_n_2() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0xA911; //load the value 2321 into I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6200; //load 0x00 into v[2]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6300; //load 0x00 into v[3]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();

        memory.write_byte(2321, 192).unwrap();
        memory.write_byte(2322, 128).unwrap();

        let opcode = 0xD232; //Draw 2 bytes onto the display at (0,0)
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();


        let display_byte1 = &disp.memory[0..8];
        let display_byte2 = &disp.memory[crate::display::COLUMNS..crate::display::COLUMNS + 8];

        let expected_byte1 = vec![1, 1, 0, 0, 0, 0, 0, 0];
        let expected_byte2 = vec![1, 0, 0, 0, 0, 0, 0, 0];


        assert!(expected_byte1.len() == display_byte1.len() && expected_byte1 == display_byte1);
        assert!(expected_byte2.len() == display_byte2.len() && expected_byte2 == display_byte2);
    }

    #[test]
    fn cpu_ld_dt_vx() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6022; //load 0x22 into v[0]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0], 0x22);

        let opcode = 0xF015; //load v[0] into DT
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.delay_timer, 0x22);

        let opcode = 0xF107; //load DT into v[1]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[1], 0x22);
    }

    #[test]
    fn cpu_ld_st_vx() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6022; //load 0x22 into v[0]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0], 0x22);

        let opcode = 0xF018; //load v[0] into ST
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.sound_timer, 0x22);
    }

    #[test]
    fn cpu_add_i_vx() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6022; //load 0x22 into v[0]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0], 0x22);

        let opcode = 0x6133; //load 0x33 into v[1]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[1], 0x33);

        let opcode = 0xF01E; //add v[0] to I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.i, 0x22);

        let opcode = 0xF11E; //add v[1] to I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.i, 0x55);
    }

    #[test]
    fn cpu_ld_f_vx() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6002; //load 0x2 into v[0]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0], 0x02);

        let opcode = 0xF029; //load address of font character in v[0] into I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.i, 0x0A);
    }

    #[test]
    fn cpu_ld_b_vx() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x60B7; //load 0xB7 into v[0]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0], 0xB7);

        let opcode = 0xA1AF; //load the value 0x1AF into I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.i, 0x01AF);

        let opcode = 0xF033; //store BCD representation of Vx in memory locations I, I+1, and I+2.
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(memory.read_byte(cpu.i as usize).unwrap(), 1);
        assert_eq!(memory.read_byte((cpu.i + 1) as usize).unwrap(), 8);
        assert_eq!(memory.read_byte((cpu.i + 2) as usize).unwrap(), 3);
    }

    #[test]
    fn cpu_ld_i_vx() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        let opcode = 0x6000; //load 0x00 into v[0]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6101; //load 0x01 into v[1]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6202; //load 0x02 into v[2]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6303; //load 0x03 into v[3]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6404; //load 0x04 into v[4]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6505; //load 0x05 into v[5]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6606; //load 0x06 into v[6]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6707; //load 0x07 into v[7]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6808; //load 0x08 into v[8]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6909; //load 0x09 into v[9]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6A0A; //load 0x0A into v[A]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6B0B; //load 0x0B into v[B]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6C0C; //load 0x0C into v[C]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6D0D; //load 0x0D into v[D]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6E0E; //load 0x0E into v[E]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        let opcode = 0x6F0F; //load 0x0F into v[F]
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();

        let opcode = 0xA1AF; //load the value 0x1AF into I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();

        let opcode = 0xFF55; //store V[0]..v[F] into memory starting at addr in I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(memory.read_byte(cpu.i as usize).unwrap(), 0x0);
        assert_eq!(memory.read_byte((cpu.i + 1) as usize).unwrap(), 0x1);
        assert_eq!(memory.read_byte((cpu.i + 2) as usize).unwrap(), 0x2);
        assert_eq!(memory.read_byte((cpu.i + 3) as usize).unwrap(), 0x3);
        assert_eq!(memory.read_byte((cpu.i + 4) as usize).unwrap(), 0x4);
        assert_eq!(memory.read_byte((cpu.i + 5) as usize).unwrap(), 0x5);
        assert_eq!(memory.read_byte((cpu.i + 6) as usize).unwrap(), 0x6);
        assert_eq!(memory.read_byte((cpu.i + 7) as usize).unwrap(), 0x7);
        assert_eq!(memory.read_byte((cpu.i + 8) as usize).unwrap(), 0x8);
        assert_eq!(memory.read_byte((cpu.i + 9) as usize).unwrap(), 0x9);
        assert_eq!(memory.read_byte((cpu.i + 10) as usize).unwrap(), 0xA);
        assert_eq!(memory.read_byte((cpu.i + 11) as usize).unwrap(), 0xB);
        assert_eq!(memory.read_byte((cpu.i + 12) as usize).unwrap(), 0xC);
        assert_eq!(memory.read_byte((cpu.i + 13) as usize).unwrap(), 0xD);
        assert_eq!(memory.read_byte((cpu.i + 14) as usize).unwrap(), 0xE);
        assert_eq!(memory.read_byte((cpu.i + 15) as usize).unwrap(), 0xF);
    }

    #[test]
    fn cpu_ld_vx_i() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        memory.write_byte(0x400, 0x00).unwrap();
        memory.write_byte(0x401, 0x01).unwrap();
        memory.write_byte(0x402, 0x02).unwrap();
        memory.write_byte(0x403, 0x03).unwrap();
        memory.write_byte(0x404, 0x04).unwrap();
        memory.write_byte(0x405, 0x05).unwrap();
        memory.write_byte(0x406, 0x06).unwrap();
        memory.write_byte(0x407, 0x07).unwrap();
        memory.write_byte(0x408, 0x08).unwrap();
        memory.write_byte(0x409, 0x09).unwrap();
        memory.write_byte(0x40A, 0x0A).unwrap();
        memory.write_byte(0x40B, 0x0B).unwrap();
        memory.write_byte(0x40C, 0x0C).unwrap();
        memory.write_byte(0x40D, 0x0D).unwrap();
        memory.write_byte(0x40E, 0x0E).unwrap();
        memory.write_byte(0x40F, 0x0F).unwrap();

        let opcode = 0xA400; //load the value 0x400 into I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();

        let opcode = 0xFF65; //read V[0]..v[F] in from memory starting at addr in I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0x0], 0x00);
        assert_eq!(cpu.v[0x1], 0x01);
        assert_eq!(cpu.v[0x2], 0x02);
        assert_eq!(cpu.v[0x3], 0x03);
        assert_eq!(cpu.v[0x4], 0x04);
        assert_eq!(cpu.v[0x5], 0x05);
        assert_eq!(cpu.v[0x6], 0x06);
        assert_eq!(cpu.v[0x7], 0x07);
        assert_eq!(cpu.v[0x8], 0x08);
        assert_eq!(cpu.v[0x9], 0x09);
        assert_eq!(cpu.v[0xA], 0x0A);
        assert_eq!(cpu.v[0xB], 0x0B);
        assert_eq!(cpu.v[0xC], 0x0C);
        assert_eq!(cpu.v[0xD], 0x0D);
        assert_eq!(cpu.v[0xE], 0x0E);
        assert_eq!(cpu.v[0xF], 0x0F);
    }

    #[test]
    fn cpu_ld_vx_i_2() {
        let mut cpu = CPU::new();
        let mut disp = Display::new();
        let mut memory = Memory::new();
        let keyboard = Keyboard::new();

        memory.write_byte(0x400, 0x00).unwrap();
        memory.write_byte(0x401, 0x01).unwrap();
        memory.write_byte(0x402, 0x02).unwrap();
        memory.write_byte(0x403, 0x03).unwrap();
        memory.write_byte(0x404, 0x04).unwrap();
        memory.write_byte(0x405, 0x05).unwrap();
        memory.write_byte(0x406, 0x06).unwrap();
        memory.write_byte(0x407, 0x07).unwrap();
        memory.write_byte(0x408, 0x08).unwrap();
        memory.write_byte(0x409, 0x09).unwrap();
        memory.write_byte(0x40A, 0x0A).unwrap();
        memory.write_byte(0x40B, 0x0B).unwrap();
        memory.write_byte(0x40C, 0x0C).unwrap();
        memory.write_byte(0x40D, 0x0D).unwrap();
        memory.write_byte(0x40E, 0x0E).unwrap();
        memory.write_byte(0x40F, 0x0F).unwrap();

        let opcode = 0xA400; //load the value 0x400 into I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();

        let opcode = 0xF265; //read V[0]..v[2] in from memory starting at addr in I
        cpu.process_opcode(opcode, &mut disp, &mut memory, &keyboard).unwrap();
        assert_eq!(cpu.v[0x0], 0x00);
        assert_eq!(cpu.v[0x1], 0x01);
        assert_eq!(cpu.v[0x2], 0x02);
        assert_eq!(cpu.v[0x3], 0x00);
        assert_eq!(cpu.v[0x4], 0x00);
        assert_eq!(cpu.v[0x5], 0x00);
        assert_eq!(cpu.v[0x6], 0x00);
        assert_eq!(cpu.v[0x7], 0x00);
        assert_eq!(cpu.v[0x8], 0x00);
        assert_eq!(cpu.v[0x9], 0x00);
        assert_eq!(cpu.v[0xA], 0x00);
        assert_eq!(cpu.v[0xB], 0x00);
        assert_eq!(cpu.v[0xC], 0x00);
        assert_eq!(cpu.v[0xD], 0x00);
        assert_eq!(cpu.v[0xE], 0x00);
        assert_eq!(cpu.v[0xF], 0x00);
    }


    
}

/*
//SKP Vx
(0xE, _, 0x9, 0xE) => match keyboard.get_key_pressed() {
    Some(key) => {
        if self.v[x] == key {
            self.pc = self.pc + 2;
        }
    }
    None => {}
},
//SKNP Vx
(0xE, _, 0xA, 0x1) => match keyboard.get_key_pressed() {
    Some(key) => {
        if self.v[x] != key {
            self.pc = self.pc + 2;
        }
    }
    None => {}
},
//LD Vx K
(0xF, _, 0x0, 0xA) => match keyboard.get_key_pressed() {
    Some(key) => self.v[x] = key,
    None => should_update_pc_after_processing = false,
},
*/
