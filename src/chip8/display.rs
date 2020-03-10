/// Struct representing the display of a chip-8 machine
#[derive(Debug)]
pub struct Display {
    pixels: [[bool; 64]; 32] 
}

impl Display {
    /// Creates a new display with no active pixels
    pub fn new() -> Display {
        Display {
            pixels: [[false; 64]; 32]
        }
    }

    /// XOR's a pixel at (x, y) onto the screen.
    /// Returns true if the pixel at (x, y) is turned from active to inactive
    pub fn xor(&mut self, x: u8, y: u8, val: bool) -> bool {
        let result = self.pixels[y][x] & val; 
        self.pixels[y][x] = self.pixels[y][x] ^ val;
        result
    }
}
