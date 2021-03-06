use super::Instruction;
use super::State;
use crate::variables::Read;

/// Represents the SYS instruction (goto a routine at SYS.0)
#[derive(Debug)]
pub struct SYS<T: Read<usize>>(pub T);

impl<T: Read<usize>> Instruction for SYS<T> {
    fn execute(&self, state: &mut State) {
        state.program_counter = self.0.read(state) - 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, tribble::B12};

    #[test]
    fn test_sys() {
        let mut state = State::mock(&[]);
        let sys = SYS(B12(B4(0b1011), B4(0b1001), B4(0b0010)));
        sys.execute(&mut state);
        state.program_counter += 2;
        assert_eq!(state.program_counter, 0b1011_1001_0010);
    }
}
