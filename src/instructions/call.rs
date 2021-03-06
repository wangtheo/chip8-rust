use super::{Instruction, State};
use crate::variables::Read;

/// Represents the CALL instruction (call subroutine at CALL.0)
#[derive(Debug)]
pub struct CALL<T: Read<usize>>(pub T);

impl<T: Read<usize>> Instruction for CALL<T> {
    fn execute(&self, state: &mut State) {
        state.stack.push(state.program_counter).unwrap();
        state.program_counter = self.0.read(state) - 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::variables::{nibble::B4, tribble::B12};

    #[test]
    fn test_call() {
        let mut state = State::mock(&[]);
        let program_counter = state.program_counter;
        let call = CALL(B12(B4(0b1000), B4(0b0010), B4(0b0001)));
        call.execute(&mut state);
        state.program_counter += 2;
        assert_eq!(state.stack.top().unwrap(), program_counter);
        assert_eq!(state.program_counter, 0b1000_0010_0001);
    }
}
