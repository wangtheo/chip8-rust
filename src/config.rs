use sdl2::keyboard::Keycode;
use serde::Deserialize;
use serde_json;
use std::{collections::HashMap, fs::File};

/// Represents the JSON config file
#[derive(Deserialize)]
pub struct Config {
    pub ticks_per_frame: u8,
    pub frames_per_second: u32,
    pub pixel_size: u32,
    pub active_color: Color,
    pub inactive_color: Color,
    keyboard: HashMap<String, String>,
}

impl Config {
    /// Reads a config struct from a file path
    pub fn from_file(path: &str) -> Self {
        let file = File::open(path).unwrap_or_else(|_| panic!("Could not open file at {}", path));
        serde_json::from_reader(file)
            .unwrap_or_else(|_| panic!("Could not deserialize file at {}", path))
    }

    /// Returns the default key mappings for the keys not given in the config file
    fn default_keyboard() -> HashMap<String, String> {
        [
            ("1", "1"),
            ("2", "2"),
            ("3", "3"),
            ("C", "4"),
            ("4", "Q"),
            ("5", "W"),
            ("6", "E"),
            ("D", "R"),
            ("7", "A"),
            ("8", "S"),
            ("9", "D"),
            ("E", "F"),
            ("A", "Z"),
            ("0", "X"),
            ("B", "C"),
            ("F", "V"),
        ]
        .iter()
        .map(|(x, y)| (String::from(*x), String::from(*y)))
        .collect()
    }

    /// Returns an array of keycodes representing the keyboard mapping (for SDL)
    /// The index of an element is its chip8 keycode, the element itself is the SDL keycode
    pub fn get_keyboard(&self) -> [Keycode; 16] {
        let mut result = [Keycode::Num2; 16];
        let default_keyboard = Self::default_keyboard();
        for i in 0..16 {
            let key_name: &String = if self.keyboard.contains_key(&format!("{:x}", i)) {
                &self.keyboard[&format!("{:X}", i)]
            } else {
                &default_keyboard[&format!("{:X}", i)]
            };
            result[i] = Keycode::from_name(key_name).unwrap_or_else(|| {
                panic!(
                    "Could not find key with name {}. Please use an SDL key name!",
                    key_name
                )
            });
        }
        result
    }
}

#[derive(Deserialize, Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub fn black() -> Self {
        Color(
            u8::min_value(),
            u8::min_value(),
            u8::min_value(),
            u8::min_value(),
        )
    }

    pub fn white() -> Self {
        Color(
            u8::max_value(),
            u8::max_value(),
            u8::max_value(),
            u8::max_value(),
        )
    }
}

/// Default configuration
impl Default for Config {
    fn default() -> Self {
        Config {
            ticks_per_frame: 9,
            pixel_size: 10,
            frames_per_second: 60,
            active_color: Color::white(),
            inactive_color: Color::black(),
            keyboard: Self::default_keyboard(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_keyboard() {
        let config: Config = Default::default();
        assert_eq!(config.get_keyboard().len(), 16);
    }
}
