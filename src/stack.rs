use bytes::*;
use state::State;

pub trait Stack {
    fn push8(&mut self, value: u8);
    fn pop8(&mut self) -> u8;
    fn push16(&mut self, value: u16) {
        self.push8(high_order_byte(value));
        self.push8(low_order_byte(value));
    }
    fn pop16(&mut self) -> u16 {
        let low_order = self.pop8();
        let high_order = self.pop8();
        assemble_word(high_order, low_order)
    }
}

impl Stack for State {
    fn pop8(&mut self) -> u8 {
        self.sp += 1;
        self.memory.get(self.sp - 1)
    }

    fn push8(&mut self, value: u8) {
        self.memory.set(self.sp - 1, value);
        self.sp -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push8() {
        let mut state = State::new();

        state.sp = 0x80;

        state.push8(0x22);
        assert_eq!(state.memory.get(0x7f), 0x22);
        assert_eq!(state.sp, 0x7f);

        state.push8(0x28);
        assert_eq!(state.memory.get(0x7e), 0x28);
        assert_eq!(state.sp, 0x7e);
    }

    #[test]
    fn test_push16() {
        let mut state = State::new();

        state.sp = 0x80;

        state.push16(0x2228);
        assert_eq!(state.memory.get(0x7f), 0x22);
        assert_eq!(state.memory.get(0x7e), 0x28);
        assert_eq!(state.sp, 0x7e);
    }

    #[test]
    fn test_pop8() {
        let mut state = State::new();

        state.sp = 0xff;
        state.push8(0x68);
        assert_eq!(state.sp, 0xfe);

        assert_eq!(state.pop8(), 0x68);
        assert_eq!(state.sp, 0xff);
    }

    #[test]
    fn test_pop16() {
        let mut state = State::new();

        state.sp = 0xff;
        state.push16(0x9876);
        assert_eq!(state.sp, 0xfd);

        assert_eq!(state.pop16(), 0x9876);
        assert_eq!(state.sp, 0xff);
    }
}
