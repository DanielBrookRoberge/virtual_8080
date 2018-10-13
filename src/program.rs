use bytes::assemble_word;
use state::State;

pub trait Program {
    fn get_arg(&self, offset: u16) -> u8;

    fn get_opcode(&self) -> u8 {
        self.get_arg(0)
    }

    fn get_arg8(&self) -> u8 {
        self.get_arg(1)
    }

    fn get_arg16(&self) -> u16 {
        assemble_word(self.get_arg(2), self.get_arg(1))
    }
}

impl Program for State {
    fn get_arg(&self, offset: u16) -> u8 {
        self.memory.get(self.pc + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opcode() {
        let mut state = State::new();

        state.memory.load(0x4096, vec![0xab, 0xec]);
        state.memory.load(0xaaaa, vec![0x02]);

        state.pc = 0x4097;
        assert_eq!(state.get_opcode(), 0xec);

        state.pc = 0xaaaa;
        assert_eq!(state.get_opcode(), 0x02);
    }

    #[test]
    fn test_get_arg8() {
        let mut state = State::new();

        state.memory.load(0xbeef, vec![0xab, 0xca, 0xfe]);

        state.pc = 0xbeef;
        assert_eq!(state.get_arg8(), 0xca);
    }

    #[test]
    fn test_get_arg16() {
        let mut state = State::new();

        state.memory.load(0xbeef, vec![0xab, 0xca, 0xfe]);

        state.pc = 0xbeef;
        assert_eq!(state.get_arg16(), 0xfeca);
    }
}
