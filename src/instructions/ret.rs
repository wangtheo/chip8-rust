use super::Instruction;
use super::State;

/// Represents the RET instruction (returns from subroutine)
#[derive(Debug)]
pub struct RET;

impl<'a> Instruction for RET {
    fn execute(&self, state: &mut State) {
        state.program_counter = state.stack.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ret() {
        let mut state = State::mock(&[]);
        state.stack.push(3).unwrap();
        let ret = RET;
        ret.execute(&mut state);
        state.program_counter += 2;
        assert_eq!(state.program_counter, 5);
        assert_eq!(state.stack.top(), None);
    }

    #[test]
    #[should_panic]
    fn test_ret_empty_stack() {
        let mut state = State::mock(&[]);
        let ret = RET;
        ret.execute(&mut state);
    }
}
