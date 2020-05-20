pub enum CPUError {
    ComputationError(String),
}

pub struct CPU {
    //data registers
    v: [u8; 16],
    //address register
    i: u16,
    //program counter
    pc: u16,
    //timers
    delay_timer: u16,
    sound_timer: u16,
    //stack
    stack: Stack,
}

impl CPU {
    fn process_opcode(&mut self, opcode: u16, display: &mut Display) -> Result<(), CPUError> {
        //get typical opcode values from opcode
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
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
            (0x0, 0x0, 0xE, 0xE) => self.pc = self.stack.pop(),
            //JP addr
            (0x1, _, _, _) => self.pc = nnn,
            //CALL addr
            (0x2, _, _, _) => {
                self.stack.push(self.pc);
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
                let res = self.v[x] + self.v[y];
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
            //8XY6[a]
            //8XY7[a]
            //8XYE[a]
            //9XY0
            //ANNN
            //BNNN
            //CXNN
            //DXYN
            //EX9E
            //EXA1
            //FX07
            //FX0A
            //FX15
            //FX18
            //FX1E
            //FX29
            //FX33
            //FX55
            //FX65
            _ => {
                return Err(CPUError::ComputationError(format!(
                    "Unknown opcode: {:#04x}",
                    opcode
                )))
            }
        }

        Ok(())
    }
}
