pub const COLUMNS: usize = 64;
pub const ROWS: usize = 32;
pub const DISPLAY_MEMORY_SIZE: usize = COLUMNS * ROWS;

pub struct Display {
    memory: [u8; DISPLAY_MEMORY_SIZE],
}

impl Display {
    pub fn new() -> Self {
        Display {
            memory: [0; DISPLAY_MEMORY_SIZE],
        }
    }

    pub fn clear(&mut self) {
        self.memory = [0; DISPLAY_MEMORY_SIZE];
    }

    pub fn draw(&mut self, sprite: &[u8], location: (usize, usize)) -> bool {
        let (x, y) = location;
        let x = self.wrap(x, ROWS);
        let y = self.wrap(y, COLUMNS);
        let mut is_pixel_erased = false;
        for (i, byte) in sprite.into_iter().enumerate() {
            let location = (COLUMNS * (y + i)) + x;
            let before = self.memory[location];
            let after = before ^ *byte;
            self.memory[location] = after;
            is_pixel_erased = is_pixel_erased || self.was_pixel_ersased(before, after);
        }
        is_pixel_erased
    }

    fn wrap(&self, value: usize, max_value: usize) -> usize {
        let mut wrapped_value: usize = value;
        while wrapped_value > max_value {
            wrapped_value = wrapped_value - max_value;
        }
        wrapped_value
    }

    fn was_pixel_ersased(&self, before: u8, after: u8) -> bool {
        before & after != before
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;

    #[test]
    fn display_was_pixel_erased() {
        let disp = Display::new();

        let before = 0xFF;
        let after = 0xF0;
        assert_eq!(disp.was_pixel_ersased(before, after), true);

        let before = 0xF0;
        let after = 0xFF;
        assert_eq!(disp.was_pixel_ersased(before, after), false);

        let before = 0xF0;
        let after = 0x20;
        assert_eq!(disp.was_pixel_ersased(before, after), true);
    }

    #[test]
    fn display_wrap() {
        let disp = Display::new();

        let x = 20;
        let column = 11;
        assert_eq!(disp.wrap(x, column), 9);
    }

    #[test]
    fn display_draw() {
        let mut disp = Display::new();

        // -- 1 -- //
        let data: Vec<u8> = vec![0x01, 0x02, 0x03];
        let location = (2, 2);

        disp.clear();
        let pixel_erased = disp.draw(&data, location);

        assert_eq!(disp.memory[130], 0x01);
        assert_eq!(disp.memory[194], 0x02);
        assert_eq!(disp.memory[258], 0x03);
        assert_eq!(pixel_erased, false);

        // -- 2 -- //
        let data: Vec<u8> = vec![0xF0, 0xF0, 0xF0];
        let location = (2, 2);

        let pixel_erased = disp.draw(&data, location);

        assert_eq!(disp.memory[130], 0xF1);
        assert_eq!(disp.memory[194], 0xF2);
        assert_eq!(disp.memory[258], 0xF3);
        assert_eq!(pixel_erased, false);

        // -- 3 -- //
        let data: Vec<u8> = vec![0x01, 0x02, 0x03];
        let location = (2, 2);

        let pixel_erased = disp.draw(&data, location);

        assert_eq!(disp.memory[130], 0xF0);
        assert_eq!(disp.memory[194], 0xF0);
        assert_eq!(disp.memory[258], 0xF0);
        assert_eq!(pixel_erased, true);
    }
}
