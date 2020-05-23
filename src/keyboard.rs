pub struct Keyboard {
    key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { key_pressed: None }
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }

    pub fn press_key(&mut self, key: u8) {
        self.key_pressed = Some(key);
    }

    pub fn release_key(&mut self) {
        self.key_pressed = None;
    }
}
