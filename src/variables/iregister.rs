use super::{Read, State, Write};

/// Represents the I register
#[derive(Debug)]
pub struct I;

/// We should be able to read a u16 from the I register
impl Read<u16> for I {
    fn read(&self, state: &State) -> u16 {
        state.registers.i_register
    }
}

/// We should be able to read a usize from the I register
impl Read<usize> for I {
    fn read(&self, state: &State) -> usize {
        usize::from(Read::<u16>::read(self, state))
    }
}

/// We should be able to write a u16 to the I register
impl Write<u16> for I {
    fn write(&self, state: &mut State, val: u16) {
        state.registers.i_register = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u16() {
        let i = I;
        let mut state = State::mock(&[]);
        state.registers.i_register = 14;
        let result: u16 = i.read(&state);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_read_usize() {
        let i = I;
        let mut state = State::mock(&[]);
        state.registers.i_register = 81;
        let result: usize = i.read(&state);
        assert_eq!(result, 81);
    }

    #[test]
    fn test_write_u16() {
        let i = I;
        let mut state = State::mock(&[]);
        i.write(&mut state, 9);
        assert_eq!(state.registers.i_register, 9);
    }
}
