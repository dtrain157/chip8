use std::fmt;

pub const COLUMNS: usize = 64;
pub const ROWS: usize = 32;
pub const DISPLAY_MEMORY_SIZE: usize = COLUMNS * ROWS;

pub struct Display {
    pub memory: [u8; DISPLAY_MEMORY_SIZE],
}

impl Display {
    pub fn new() -> Self {
        Display { memory: [0; DISPLAY_MEMORY_SIZE] }
    }

    pub fn clear(&mut self) {
        self.memory = [0; DISPLAY_MEMORY_SIZE];
    }

    pub fn draw(&mut self, sprite: &[u8], location: (usize, usize)) -> bool {
        let (x, y) = location;
        let x = self.wrap(x, COLUMNS);
        let y = self.wrap(y, ROWS);
        let mut is_pixel_erased = false;
        for (i, byte) in sprite.into_iter().enumerate() {
            let yi = self.wrap(y + i, ROWS);
            for j in 0..8 {
                let xi = self.wrap(x + j, COLUMNS);
                let insert_location = (COLUMNS * yi) + xi;
                let bit = (*byte >> (7 - j)) & 0x01;
                if self.memory[insert_location] != bit {
                    self.memory[insert_location] = 1;
                } else {
                    if self.memory[insert_location] == 1 {
                        self.memory[insert_location] = 0;
                        is_pixel_erased = true;
                    }
                }
            }
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
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for j in 0..ROWS - 1 {
            for i in 0..COLUMNS - 1 {
                write!(f, "{} ", self.memory[(COLUMNS * j) + i])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod display_tests {
    use super::*;

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

        assert_eq!(disp.memory[(COLUMNS * 2) + 9], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 8], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 8], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 9], 1);
        assert_eq!(pixel_erased, false);

        // -- 2 -- //
        let data: Vec<u8> = vec![0xF0, 0xF0, 0xF0];
        let location = (2, 2);

        let pixel_erased = disp.draw(&data, location);

        assert_eq!(disp.memory[(COLUMNS * 2) + 2], 1);
        assert_eq!(disp.memory[(COLUMNS * 2) + 3], 1);
        assert_eq!(disp.memory[(COLUMNS * 2) + 4], 1);
        assert_eq!(disp.memory[(COLUMNS * 2) + 5], 1);
        assert_eq!(disp.memory[(COLUMNS * 2) + 6], 0);
        assert_eq!(disp.memory[(COLUMNS * 2) + 7], 0);
        assert_eq!(disp.memory[(COLUMNS * 2) + 8], 0);
        assert_eq!(disp.memory[(COLUMNS * 2) + 9], 1);

        assert_eq!(disp.memory[(COLUMNS * 3) + 2], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 3], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 4], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 5], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 6], 0);
        assert_eq!(disp.memory[(COLUMNS * 3) + 7], 0);
        assert_eq!(disp.memory[(COLUMNS * 3) + 8], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 9], 0);

        assert_eq!(disp.memory[(COLUMNS * 4) + 2], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 3], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 4], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 5], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 6], 0);
        assert_eq!(disp.memory[(COLUMNS * 4) + 7], 0);
        assert_eq!(disp.memory[(COLUMNS * 4) + 8], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 9], 1);
        assert_eq!(pixel_erased, false);

        // -- 3 -- //
        let data: Vec<u8> = vec![0x01, 0x02, 0x03];
        let location = (2, 2);

        let pixel_erased = disp.draw(&data, location);

        assert_eq!(disp.memory[(COLUMNS * 2) + 2], 1);
        assert_eq!(disp.memory[(COLUMNS * 2) + 3], 1);
        assert_eq!(disp.memory[(COLUMNS * 2) + 4], 1);
        assert_eq!(disp.memory[(COLUMNS * 2) + 5], 1);
        assert_eq!(disp.memory[(COLUMNS * 2) + 6], 0);
        assert_eq!(disp.memory[(COLUMNS * 2) + 7], 0);
        assert_eq!(disp.memory[(COLUMNS * 2) + 8], 0);
        assert_eq!(disp.memory[(COLUMNS * 2) + 9], 0);

        assert_eq!(disp.memory[(COLUMNS * 3) + 2], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 3], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 4], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 5], 1);
        assert_eq!(disp.memory[(COLUMNS * 3) + 6], 0);
        assert_eq!(disp.memory[(COLUMNS * 3) + 7], 0);
        assert_eq!(disp.memory[(COLUMNS * 3) + 8], 0);
        assert_eq!(disp.memory[(COLUMNS * 3) + 9], 0);

        assert_eq!(disp.memory[(COLUMNS * 4) + 2], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 3], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 4], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 5], 1);
        assert_eq!(disp.memory[(COLUMNS * 4) + 6], 0);
        assert_eq!(disp.memory[(COLUMNS * 4) + 7], 0);
        assert_eq!(disp.memory[(COLUMNS * 4) + 8], 0);
        assert_eq!(disp.memory[(COLUMNS * 4) + 9], 0);
        assert_eq!(pixel_erased, true);
    }
}
