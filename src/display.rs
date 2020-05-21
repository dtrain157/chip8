pub const DISPLAY_MEMORY_SIZE: usize = 2048; //support 2k of video memory

pub struct Display {
    memory: [u8; DISPLAY_MEMORY_SIZE],
}

impl Display {
    pub fn new() -> Self {
        Display {
            memory: [0; DISPLAY_MEMORY_SIZE],
        }
    }

    pub fn clear_momory(&mut self) {
        self.memory = [0; 2048];
    }
}
