pub mod byte;
pub mod iregister;
pub mod nibble;
pub mod tribble;
pub mod vregister;
pub mod sound_timer;
pub mod delay_timer;
pub mod memory_at;

use super::chip8::State;

/// Trait for variables that can be read from chip-8's state
pub trait Read<T> {
    fn read(&self, from: &State) -> T;
}

/// Trait for variables that can be written to chip-8's state
pub trait Write<'a, T> {
    fn write(&self, to: &'a mut State) -> &'a mut T;
}
