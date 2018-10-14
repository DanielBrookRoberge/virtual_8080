use std::collections::VecDeque;

use bytes::*;
use flags::Flags;
use memory::Memory;
use program::Program;
use stack::Stack;

static INSTRUCTION_LENGTH: [u16; 256] = [
    1, 3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 0x00..0x0f
    1, 3, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 0x10..0x1f
    1, 3, 3, 1, 1, 1, 2, 1, 1, 1, 3, 1, 1, 1, 2, 1, // 0x20..0x2f
    1, 3, 3, 1, 1, 1, 2, 1, 1, 1, 3, 1, 1, 1, 2, 1, // 0x30..0x3f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x40..0x4f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x50..0x5f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x60..0x6f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x70..0x7f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x80..0x8f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0x90..0x9f
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0xa0..0xaf
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0xb0..0xbf
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 3, 3, 3, 2, 1, // 0xc0..0xcf
    1, 1, 3, 2, 3, 1, 2, 1, 1, 1, 3, 2, 3, 3, 2, 1, // 0xd0..0xdf
    1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1, // 0xc0..0xcf
    1, 1, 3, 1, 3, 1, 2, 1, 1, 1, 3, 1, 3, 3, 2, 1, // 0xc0..0xcf
];

#[derive(Debug, Clone, Copy)]
pub struct Snapshot {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

#[derive(Debug)]
pub struct State {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub cc: Flags,
    pub int_enable: bool,
    pub memory: Memory,
    pub jumped: bool,
    pub trace_history: VecDeque<Snapshot>,
}

impl State {
    pub fn new() -> State {
        State {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            cc: Flags::new(),
            int_enable: false,
            memory: Memory::new(),
            jumped: false,
            trace_history: VecDeque::with_capacity(50),
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            a: self.a,
            b: self.b,
            c: self.c,
            d: self.d,
            e: self.e,
            h: self.h,
            l: self.l,
            sp: self.sp,
            pc: self.pc,
        }
    }

    pub fn is_plus(s: &State) -> bool {
        !s.cc.s
    }

    pub fn is_minus(s: &State) -> bool {
        s.cc.s
    }

    pub fn is_nz(s: &State) -> bool {
        !s.cc.z
    }

    pub fn is_z(s: &State) -> bool {
        s.cc.z
    }

    pub fn is_nc(s: &State) -> bool {
        !s.cc.cy
    }

    pub fn is_c(s: &State) -> bool {
        s.cc.cy
    }

    pub fn is_parity_odd(s: &State) -> bool {
        !s.cc.p
    }

    pub fn is_parity_even(s: &State) -> bool {
        s.cc.p
    }

    pub fn unconditionally(_s: &State) -> bool {
        true
    }

    pub fn advance(&mut self, opcode: u8) {
        if !self.jumped {
            self.pc += INSTRUCTION_LENGTH[opcode as usize];
        }
        self.jumped = false;
    }

    pub fn get_bc(&self) -> u16 {
        assemble_word(self.b, self.c)
    }

    pub fn get_de(&self) -> u16 {
        assemble_word(self.d, self.e)
    }

    pub fn get_hl_address(&self) -> u16 {
        assemble_word(self.h, self.l)
    }

    pub fn get_m(&self) -> u8 {
        self.memory.get(self.get_hl_address())
    }

    pub fn set_m(&mut self, value: u8) {
        let address = self.get_hl_address();
        self.memory.set(address, value)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = high_order_byte(value);
        self.c = low_order_byte(value);
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = high_order_byte(value);
        self.e = low_order_byte(value);
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = high_order_byte(value);
        self.l = low_order_byte(value);
    }

    pub fn get_operand(&self, opcode: u8) -> u8 {
        match opcode & 0x07 {
            0x0 => self.b,
            0x1 => self.c,
            0x2 => self.d,
            0x3 => self.e,
            0x4 => self.h,
            0x5 => self.l,
            0x6 => self.get_m(),
            0x7 => self.a,
            _ => panic!("shouldn't happen"),
        }
    }

    pub fn set_register(&mut self, opcode: u8, val: u8) {
        match (opcode >> 3) & 0x07 {
            0x0 => self.b = val,
            0x1 => self.c = val,
            0x2 => self.d = val,
            0x3 => self.e = val,
            0x4 => self.h = val,
            0x5 => self.l = val,
            0x6 => self.set_m(val),
            0x7 => self.a = val,
            _ => panic!("shouldn't happen"),
        }
    }

    pub fn predicate_for(opcode: u8) -> impl (Fn(&State) -> bool) {
        match (opcode >> 3) & 0x07 {
            0x0 => State::is_nz,
            0x1 => State::is_z,
            0x2 => State::is_nc,
            0x3 => State::is_c,
            0x4 => State::is_parity_odd,
            0x5 => State::is_parity_even,
            0x6 => State::is_plus,
            0x7 => State::is_minus,
            _ => panic!("shouldn't happen"),
        }
    }

    pub fn set_flags_no_carry(&mut self, result: u8) {
        self.cc.set_z(result);
        self.cc.set_s(result);
        self.cc.set_p(result);
    }

    pub fn set_flags(&mut self, result: u16) {
        self.set_flags_no_carry(low_order_byte(result));
        self.cc.cy = result > 0xff;
    }

    pub fn add8(&mut self, addend: u8) {
        let result = (self.a as u16) + (addend as u16);
        self.set_flags(result);
        self.a = low_order_byte(result);
    }

    pub fn add16(&mut self, addend: u16) {
        let val = self.get_hl_address();
        let result = (val as u32) + (addend as u32);
        self.cc.cy = result > 0xffff;
        self.set_hl(result as u16);
    }

    pub fn adc8(&mut self, addend: u8) {
        let result = (self.a as u16) + (addend as u16) + (self.cc.z as u16);
        self.set_flags(result);
        self.a = low_order_byte(result);
    }

    pub fn sub8(&mut self, subtractand: u8) {
        let result = self.a.wrapping_sub(subtractand);
        self.set_flags(result as u16);
        self.cc.cy = self.a < subtractand;
        self.a = result;
    }

    pub fn sbb8(&mut self, subtractand: u8) {
        let result = self
            .a
            .wrapping_sub(subtractand)
            .wrapping_sub(self.cc.cy as u8);

        self.set_flags(result as u16);
        self.cc.cy = self.a < subtractand;
        self.a = result;
    }

    pub fn and8(&mut self, operand: u8) {
        let result = self.a & operand;
        self.set_flags(result as u16);
        self.a = result;
    }

    pub fn xor8(&mut self, operand: u8) {
        let result = self.a ^ operand;
        self.set_flags(result as u16);
        self.a = result;
    }

    pub fn or8(&mut self, operand: u8) {
        let result = self.a | operand;
        self.set_flags(result as u16);
        self.a = result;
    }

    pub fn cmp8(&mut self, operand: u8) {
        let result = self.a.wrapping_sub(operand);
        self.set_flags(result as u16);
        self.cc.cy = self.a < operand;
    }

    pub fn jump_if(&mut self, predicate: impl Fn(&State) -> bool) {
        let new_address = assemble_word(self.get_arg(2), self.get_arg(1));
        if predicate(self) {
            self.pc = new_address;
            self.jumped = true;
        }
    }

    pub fn call_if(&mut self, predicate: impl Fn(&State) -> bool) {
        let new_address = assemble_word(self.get_arg(2), self.get_arg(1));
        if predicate(self) {
            let ret = self.pc + 3;
            self.push16(ret);
            self.pc = new_address;
            self.jumped = true;
        }
    }

    pub fn rst_to(&mut self, target: u16) {
        let ret = self.pc + 1;
        self.push16(ret);
        self.pc = target;
        self.jumped = true;
    }

    pub fn ret_if(&mut self, predicate: impl Fn(&State) -> bool) {
        if predicate(self) {
            self.pc = self.pop16();
            self.jumped = true;
        }
    }

    pub fn stack_debug(&self, n: usize) {
        println!("Stack pointer: {:04x}", self.sp);
        for i in 0..n {
            println!("{:02x}", self.memory.get(self.sp + (i as u16)));
        }
    }

    pub fn operate8(&mut self, opcode: u8, operand: u8) {
        match (opcode >> 3) & 0x07 {
            0x0 => self.add8(operand),
            0x1 => self.adc8(operand),
            0x2 => self.sub8(operand),
            0x3 => self.sbb8(operand),
            0x4 => self.and8(operand),
            0x5 => self.xor8(operand),
            0x6 => self.or8(operand),
            0x7 => self.cmp8(operand),
            _ => panic!("Shouldn't happen"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let state = State::new();

        assert_eq!(state.a, 0);
        assert_eq!(state.b, 0);
        assert_eq!(state.c, 0);
        assert_eq!(state.d, 0);
        assert_eq!(state.e, 0);
        assert_eq!(state.h, 0);
        assert_eq!(state.l, 0);
        assert_eq!(state.sp, 0);
        assert_eq!(state.pc, 0);
    }

    #[test]
    fn test_get_hl_address() {
        let mut state = State::new();

        state.h = 0xfe;
        state.l = 0xed;
        assert_eq!(state.get_hl_address(), 0xfeed);

        state.h = 0x80;
        state.l = 0x86;
        assert_eq!(state.get_hl_address(), 0x8086);
    }

    #[test]
    fn test_get_m() {
        let mut state = State::new();

        state.memory.set(0xcafe, 0xff);

        state.h = 0xca;
        state.l = 0xfe;

        assert_eq!(state.get_m(), 0xff);
    }

    #[test]
    fn test_set_m() {
        let mut state = State::new();

        state.memory.set(0xcafe, 0xff);

        state.h = 0xca;
        state.l = 0xfe;

        assert_eq!(state.get_m(), 0xff);
    }

    #[test]
    fn test_set_bc() {
        let mut state = State::new();

        state.set_bc(0x1020);
        assert_eq!(state.b, 0x10);
        assert_eq!(state.c, 0x20);
        assert_eq!(state.get_bc(), 0x1020);

        state.set_bc(0xabcd);
        assert_eq!(state.b, 0xab);
        assert_eq!(state.c, 0xcd);
        assert_eq!(state.get_bc(), 0xabcd);
    }

    #[test]
    fn test_set_de() {
        let mut state = State::new();

        state.set_de(0x1020);
        assert_eq!(state.d, 0x10);
        assert_eq!(state.e, 0x20);
        assert_eq!(state.get_de(), 0x1020);

        state.set_de(0xabcd);
        assert_eq!(state.d, 0xab);
        assert_eq!(state.e, 0xcd);
        assert_eq!(state.get_de(), 0xabcd);
    }

    #[test]
    fn test_set_hl() {
        let mut state = State::new();

        state.set_hl(0x1020);
        assert_eq!(state.h, 0x10);
        assert_eq!(state.l, 0x20);

        state.set_hl(0xabcd);
        assert_eq!(state.h, 0xab);
        assert_eq!(state.l, 0xcd);
    }

    #[test]
    fn test_get_operand() {
        let mut state = State::new();

        state.a = 0xa0;
        state.b = 0xb0;
        state.c = 0xc0;
        state.d = 0xd0;
        state.e = 0xe0;
        state.h = 0x32;
        state.l = 0x64;
        state.memory.set(0x3264, 0x12);

        assert_eq!(state.get_operand(0x80), 0xb0);
        assert_eq!(state.get_operand(0x89), 0xc0);
        assert_eq!(state.get_operand(0xa2), 0xd0);
        assert_eq!(state.get_operand(0xab), 0xe0);
        assert_eq!(state.get_operand(0x44), 0x32);
        assert_eq!(state.get_operand(0x4d), 0x64);
        assert_eq!(state.get_operand(0x56), 0x12);
        assert_eq!(state.get_operand(0x5f), 0xa0);
    }

    #[test]
    fn test_set_flags() {
        let mut state = State::new();

        state.set_flags(0);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.p, true);
        assert_eq!(state.cc.cy, false);

        state.set_flags(0xf0);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.p, true);
        assert_eq!(state.cc.cy, false);

        state.set_flags(0x1f8);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.p, false);
        assert_eq!(state.cc.cy, true);

        state.set_flags(0x100);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.p, true);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_add8() {
        let mut state = State::new();

        state.a = 0x08;
        state.add8(0x12);
        assert_eq!(state.a, 0x1a);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, false);

        state.a = 0xfe;
        state.add8(0x02);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_add16() {
        let mut state = State::new();

        state.h = 0x20;
        state.l = 0x40;
        state.add16(0x0808);
        assert_eq!(state.h, 0x28);
        assert_eq!(state.l, 0x48);
        assert_eq!(state.cc.cy, false);

        state.h = 0xff;
        state.l = 0xff;
        state.add16(0x0001);
        assert_eq!(state.h, 0x00);
        assert_eq!(state.l, 0x00);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_sub8() {
        let mut state = State::new();

        state.a = 0x08;
        state.sub8(0x12);
        assert_eq!(state.a, 0xf6);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, true);

        state.a = 0xfe;
        state.sub8(0x02);
        assert_eq!(state.a, 0xfc);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, false);

        state.a = 0xe7;
        state.sub8(0xe7);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_adc8() {
        let mut state = State::new();

        state.a = 0x08;
        state.cc.cy = false;
        state.adc8(0x12);
        assert_eq!(state.a, 0x1a);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, false);

        state.a = 0xfe;
        state.cc.cy = false;
        state.adc8(0x02);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, true);

        state.a = 0xa8;
        state.cc.cy = true;
        state.adc8(0x11);
        assert_eq!(state.a, 0xba);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_sbb8() {
        let mut state = State::new();

        state.a = 0x08;
        state.cc.cy = false;
        state.sbb8(0x12);
        assert_eq!(state.a, 0xf6);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, true);

        state.a = 0xfe;
        state.cc.cy = false;
        state.sbb8(0x02);
        assert_eq!(state.a, 0xfc);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, false);

        state.a = 0xe7;
        state.cc.cy = false;
        state.sbb8(0xe7);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, false);

        state.a = 0xfe;
        state.cc.cy = true;
        state.sbb8(0x02);
        assert_eq!(state.a, 0xfb);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, false);

        state.a = 0x01;
        state.cc.cy = true;
        state.sbb8(0x00);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_and8() {
        let mut state = State::new();

        state.a = 0xaa;
        state.and8(0x0f);
        assert_eq!(state.a, 0x0a);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, false);

        state.a = 0xff;
        state.and8(0x00);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_xor8() {
        let mut state = State::new();

        state.a = 0xaa;
        state.xor8(0x3a);
        assert_eq!(state.a, 0x90);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, false);

        state.a = 0xff;
        state.xor8(0xff);
        assert_eq!(state.a, 0x00);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_or8() {
        let mut state = State::new();

        state.a = 0xaa;
        state.or8(0x0f);
        assert_eq!(state.a, 0xaf);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, false);

        state.a = 0xff;
        state.or8(0x00);
        assert_eq!(state.a, 0xff);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_cmp8() {
        let mut state = State::new();

        state.a = 0x08;
        state.cmp8(0x12);
        assert_eq!(state.a, 0x08);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, true);

        state.a = 0xfe;
        state.cmp8(0x02);
        assert_eq!(state.a, 0xfe);
        assert_eq!(state.cc.z, false);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, false);

        state.a = 0xe7;
        state.cmp8(0xe7);
        assert_eq!(state.a, 0xe7);
        assert_eq!(state.cc.z, true);
        assert_eq!(state.cc.s, false);
        assert_eq!(state.cc.cy, false);
    }

    #[test]
    fn test_jump_if() {
        let mut state = State::new();

        state.memory.set(0x20, 0xda);
        state.memory.set(0x21, 0x16);
        state.memory.set(0x22, 0x32);

        state.pc = 0x20;
        state.jump_if(|ref s| s.pc == 0x21);
        assert_eq!(state.pc, 0x20);

        state.jump_if(|ref s| s.pc < 0x21);
        assert_eq!(state.pc, 0x3216);
    }

    #[test]
    fn test_call_if() {
        let mut state = State::new();

        state.sp = 0xff;

        state.memory.set(0x3020, 0xda);
        state.memory.set(0x3021, 0x16);
        state.memory.set(0x3022, 0x32);

        state.pc = 0x3020;
        state.call_if(|ref s| s.pc > 0x4000);
        assert_eq!(state.pc, 0x3020);
        assert_eq!(state.sp, 0xff);
        assert_eq!(state.memory.get(0xfe), 0x00);
        assert_eq!(state.memory.get(0xfd), 0x00);

        state.pc = 0x3020;
        state.call_if(|ref s| s.pc < 0x4000);
        assert_eq!(state.pc, 0x3216);
        assert_eq!(state.sp, 0xfd);
        assert_eq!(state.memory.get(0xfe), 0x30);
        assert_eq!(state.memory.get(0xfd), 0x23);
    }

    #[test]
    fn test_ret_if() {
        let mut state = State::new();

        state.sp = 0xff;
        state.push16(0x9876);

        state.pc = 0xcafe;
        state.ret_if(|ref s| s.a != 0);
        assert_eq!(state.pc, 0xcafe);
        assert_eq!(state.jumped, false);

        state.ret_if(|ref s| s.a == 0);
        assert_eq!(state.pc, 0x9876);
        assert_eq!(state.jumped, true);
    }
}
