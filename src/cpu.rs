use bytes::*;
use machine::Machine;
use state::State;
use stack::Stack;

static OPCODE_TIMING: [usize; 256] = [
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x00..0x0f
    4, 10, 7, 5, 5, 5, 7, 4, 4, 10, 7, 5, 5, 5, 7, 4, //0x10..0x1f
    4, 10, 16, 5, 5, 5, 7, 4, 4, 10, 16, 5, 5, 5, 7, 4, //etc
    4, 10, 13, 5, 10, 10, 10, 4, 4, 10, 13, 5, 5, 5, 7, 4,

    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5, //0x40..0x4f
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    5, 5, 5, 5, 5, 5, 7, 5, 5, 5, 5, 5, 5, 5, 7, 5,
    7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 7, 5,

    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4, //0x80..8x4f
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,
    4, 4, 4, 4, 4, 4, 7, 4, 4, 4, 4, 4, 4, 4, 7, 4,

    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11, //0xc0..0xcf
    11, 10, 10, 10, 17, 11, 7, 11, 11, 10, 10, 10, 10, 17, 7, 11,
    11, 10, 10, 18, 17, 11, 7, 11, 11, 5, 10, 5, 17, 17, 7, 11,
    11, 10, 10, 4, 17, 11, 7, 11, 11, 5, 10, 4, 17, 17, 7, 11,
];

fn unimplemented_instruction(s: &mut State) {
    println!(
        "Error: unimplemented instruction 0x{:02x} at 0x{:04x}",
        s.get_opcode(),
        s.pc
    );
    println!("{:?}", s);
    panic!("unimplemented");
}

pub fn emulate_instruction(s: &mut State, m: &mut impl Machine) -> usize {
    let opcode = s.get_opcode();

    match opcode {
        0x00 => (), // NOP
        0x01 => {
            // LXI B,word
            let word = s.get_arg16();
            s.set_bc(word);
        }
        0x02 => {
            // STAX B
            let address = s.get_bc();
            let val = s.a;
            s.memory.set(address, val);
        }
        0x03 => {
            // INX B
            let new_value = s.get_bc().wrapping_add(1);
            s.set_bc(new_value);
        }
        0x04 => {
            // INR B
            let new_value = s.b.wrapping_add(1);
            s.set_flags_no_carry(new_value);
            s.b = new_value;
        }
        0x05 => {
            // DCR B
            let new_value = s.b.wrapping_sub(1);
            s.set_flags_no_carry(new_value);
            s.b = new_value
        }
        0x06 => {
            s.b = s.get_arg(1);
        } // MVI B,byte
        0x07 => {
            // RLC
            s.a = s.a.rotate_left(1);
            s.cc.cy = (s.a & 0x80) == 1;
        }
        0x08 => (), // NOP
        0x09 => {
            // DAD B
            let addend = s.get_bc();
            s.add16(addend);
        }
        0x0a => {
            // LDAX B
            let address = s.get_bc();
            s.a = s.memory.get(address);
        }
        0x0b => {
            // DCX B
            let new_value = s.get_bc().wrapping_sub(1);
            s.set_bc(new_value);
        }
        0x0c => {
            // INR C
            let new_value = s.c.wrapping_add(1);
            s.set_flags_no_carry(new_value);
            s.c = new_value;
        }
        0x0d => {
            // DCR C
            let new_value = s.c.wrapping_sub(1);
            s.set_flags_no_carry(new_value);
            s.c = new_value
        }
        0x0e => {
            s.c = s.get_arg(1);
        } // MVI C,byte
        0x0f => {
            // RRC
            let x = s.a;
            s.a = rotate_right(x);
            s.cc.cy = (x & 0x01) == 1;
        }

        0x10 => (), // NOP
        0x11 => {
            // LXI D,word
            let word = s.get_arg16();
            s.set_de(word);
        }
        0x12 => {
            // STAX D
            let address = s.get_de();
            let val = s.a;
            s.memory.set(address, val);
        }
        0x13 => {
            // INX D
            let new_value = s.get_de().wrapping_add(1);
            s.set_de(new_value);
        }
        0x14 => {
            // INR D
            let new_value = s.d.wrapping_add(1);
            s.set_flags_no_carry(new_value);
            s.d = new_value;
        }
        0x15 => {
            // DCR D
            let new_value = s.d.wrapping_sub(1);
            s.set_flags_no_carry(new_value);
            s.d = new_value
        }
        0x16 => {
            s.d = s.get_arg(1);
        } // MVI D,byte
        0x17 => {
            // RAL
            let x = s.a;
            s.a = (s.cc.cy as u8) | (x << 1);
            s.cc.cy = (x & 0x80) == 1;
        }
        0x18 => (), // NOP
        0x19 => {
            // DAD D
            let addend = s.get_de();
            s.add16(addend);
        }
        0x1a => {
            // LDAX D
            let address = s.get_de();
            s.a = s.memory.get(address);
        }
        0x1b => {
            // DCX D
            let new_value = s.get_de().wrapping_sub(1);
            s.set_de(new_value);
        }
        0x1c => {
            // INR E
            let new_value = s.e.wrapping_add(1);
            s.set_flags_no_carry(new_value);
            s.e = new_value;
        }
        0x1d => {
            // DCR E
            let new_value = s.e.wrapping_sub(1);
            s.set_flags_no_carry(new_value);
            s.e = new_value
        }
        0x1e => {
            s.e = s.get_arg(1);
        } // MVI E,byte
        0x1f => {
            // RAR
            let x = s.a;
            s.a = ((s.cc.cy as u8) << 7) | (x >> 1);
            s.cc.cy = (x & 0x01) == 1;
        }

        0x20 => (), // NOP
        0x21 => {
            // LXI H,word
            let word = s.get_arg16();
            s.set_hl(word);
        }
        0x22 => {
            // SHLD a16
            let address = s.get_arg16();
            let l = s.l;
            let h = s.h;
            s.memory.set(address, l);
            s.memory.set(address + 1, h);
        }
        0x23 => {
            // INX H
            let new_value = s.get_hl_address().wrapping_add(1);
            s.set_hl(new_value);
        }
        0x24 => {
            // INR H
            let new_value = s.h.wrapping_add(1);
            s.set_flags_no_carry(new_value);
            s.h = new_value;
        }
        0x25 => {
            // DCR H
            let new_value = s.h.wrapping_sub(1);
            s.set_flags_no_carry(new_value);
            s.h = new_value
        }
        0x26 => {
            s.h = s.get_arg(1);
        } // MVI H,byte
        0x27 => {
            // DAA
            if (s.a & 0x0f) > 9 {
                s.a += 6;
            }
            if (s.a & 0xf0) > 0x90 {
                s.add8(0x60);
            }
        }
        0x28 => (),                           // NOP
        0x29 => {
            // DAD H
            let addend = s.get_hl_address();
            s.add16(addend);
        }
        0x2a => {
            // LHLD a16
            let address = s.get_arg16();
            s.l = s.memory.get(address);
            s.h = s.memory.get(address + 1);
        }
        0x2b => {
            // DCX H
            let new_value = s.get_hl_address().wrapping_sub(1);
            s.set_hl(new_value);
        }
        0x2c => {
            // INR L
            let new_value = s.l.wrapping_add(1);
            s.set_flags_no_carry(new_value);
            s.l = new_value;
        }
        0x2d => {
            // DCR L
            let new_value = s.l.wrapping_sub(1);
            s.set_flags_no_carry(new_value);
            s.l = new_value
        }
        0x2e => {
            s.l = s.get_arg(1);
        } // MVI L,byte
        0x2f => {
            s.a = !s.a;
        } // CMA

        0x30 => (), // NOP
        0x31 => {
            // LXI SP,word
            s.sp = s.get_arg16();
        }
        0x32 => {
            // STA a16
            let new_address = s.get_arg16();
            let a = s.a;
            s.memory.set(new_address, a);
        }
        0x33 => {
            s.sp += 1;
        } // INX SP
        0x34 => {
            // INR M
            let new_value = s.get_m().wrapping_add(1);
            s.set_flags_no_carry(new_value);
            s.set_m(new_value);
        }
        0x35 => {
            // DCR M
            let new_value = s.get_m().wrapping_sub(1);
            s.set_flags_no_carry(new_value);
            s.set_m(new_value);
        }
        0x36 => {
            // MVI M,byte
            let val = s.get_arg(1);
            s.set_m(val);
        }
        0x37 => {
            s.cc.cy = true;
        } // STC
        0x38 => (), // NOP
        0x39 => {
            // DAD SP
            let addend = s.sp;
            s.add16(addend);
        }
        0x3a => {
            // LDA a16
            let new_address = s.get_arg16();
            s.a = s.memory.get(new_address);
        }
        0x3b => {
            s.sp -= 1;
        } // DCX SP
        0x3c => {
            // INR A
            let new_value = s.a.wrapping_add(1);
            s.set_flags_no_carry(new_value);
            s.a = new_value;
        }
        0x3d => {
            // DCR A
            let new_value = s.a.wrapping_sub(1);
            s.set_flags_no_carry(new_value);
            s.a = new_value
        }
        0x3e => {
            s.a = s.get_arg(1);
        } // MVI A,byte
        0x3f => s.cc.cy = !s.cc.cy, // CMC

        0x40..=0x47 => {
            // MOV B, *
            s.b = s.get_operand(opcode);
        }
        0x48..=0x4f => {
            // MOV C, *
            s.c = s.get_operand(opcode);
        }

        0x50..=0x57 => {
            // MOV D, *
            s.d = s.get_operand(opcode);
        }
        0x58..=0x5f => {
            // MOV E, *
            s.e = s.get_operand(opcode);
        }

        0x60..=0x67 => {
            // MOV H, *
            s.h = s.get_operand(opcode);
        }
        0x68..=0x6f => {
            // MOV L, *
            s.l = s.get_operand(opcode);
        }

        0x76 => unimplemented_instruction(s), // HLT
        0x70..=0x77 => {
            // MOV M, *
            let operand = s.get_operand(opcode);
            s.set_m(operand);
        }
        0x78..=0x7f => {
            // MOV A, *
            s.a = s.get_operand(opcode);
        }

        0x80..=0x87 => {
            // ADD *
            let addend = s.get_operand(opcode);
            s.add8(addend);
        }
        0x88..=0x8f => {
            // ADC *
            let addend = s.get_operand(opcode);
            s.adc8(addend);
        }

        0x90..=0x97 => {
            // SUB *
            let operand = s.get_operand(opcode);
            s.sub8(operand);
        }
        0x98..=0x9f => {
            // SBB *
            let operand = s.get_operand(opcode);
            s.sbb8(operand);
        }

        0xa0..=0xa7 => {
            // ANA *
            let operand = s.get_operand(opcode);
            s.and8(operand);
        }
        0xa8..=0xaf => {
            // XRA *
            let operand = s.get_operand(opcode);
            s.xor8(operand);
        }

        0xb0..=0xb7 => {
            // ORA *
            let operand = s.get_operand(opcode);
            s.or8(operand);
        }
        0xb8..=0xbf => {
            // CMP *
            let operand = s.get_operand(opcode);
            s.cmp8(operand);
        }

        0xc0 => {
            // RNZ
            s.ret_if(State::is_nz);
        }
        0xc1 => {
            // POP B
            let new_bc = s.pop16();
            s.set_bc(new_bc);
        }
        0xc2 => {
            // JNZ a16
            s.jump_if(State::is_nz);
        }
        0xc3 => {
            // JMP a16
            s.jump_if(State::unconditionally);
        }
        0xc4 => {
            // CNZ a16
            s.call_if(State::is_nz);
        }
        0xc5 => {
            // PUSH B
            let bc = s.get_bc();
            s.push16(bc);
        }
        0xc6 => {
            // ADI byte
            let addend = s.get_arg(1);
            s.add8(addend);
        }
        0xc7 => { s.rst_to(0x00); }, // RST 0
        0xc8 => {
            // RZ
            s.ret_if(State::is_z);
        }
        0xc9 => {
            // RET
            s.ret_if(State::unconditionally);
        }
        0xca => {
            // JZ a16
            s.jump_if(State::is_z);
        }
        0xcb => {
            // JMP a16
            s.jump_if(State::unconditionally);
        }
        0xcc => {
            // CZ a16
            s.call_if(State::is_z);
        }
        0xcd => {
            // CALL a16
            s.call_if(State::unconditionally);
        }
        0xce => {
            // ACI byte
            let addend = s.get_arg(1);
            s.adc8(addend);
        }
        0xcf => { s.rst_to(0x08); }, // RST 1

        0xd0 => {
            // RNC
            s.ret_if(State::is_nc);
        }
        0xd1 => {
            // POP D
            let new_de = s.pop16();
            s.set_de(new_de);
        }
        0xd2 => {
            // JNC a16
            s.jump_if(State::is_nc)
        }
        0xd3 => {
            // OUT byte
            m.output(s.get_arg(1), s.a);
        }
        0xd4 => {
            // CNC a16
            s.call_if(State::is_nc);
        }
        0xd5 => {
            // PUSH D
            let de = s.get_de();
            s.push16(de);
        }
        0xd6 => {
            // SUI byte
            let addend = s.get_arg(1);
            s.sub8(addend);
        }
        0xd7 => { s.rst_to(0x10); }, // RST 2
        0xd8 => {
            // RC
            s.ret_if(State::is_c);
        }
        0xd9 => {
            // RET
            s.ret_if(State::unconditionally);
        }
        0xda => {
            // JC a16
            s.jump_if(State::is_c);
        }
        0xdb => {
            // IN byte
            s.a = m.input(s.get_arg(1));
        }
        0xdc => {
            // CC a16
            s.call_if(State::is_c);
        }
        0xdd => {
            // CALL a16
            s.call_if(State::unconditionally);
        }
        0xde => {
            // SBI byte
            let addend = s.get_arg(1);
            s.sbb8(addend);
        }
        0xdf => { s.rst_to(0x18); } // RST 3

        0xe0 => {
            // RPO
            s.ret_if(State::is_parity_odd);
        }
        0xe1 => {
            // POP H
            let new_hl = s.pop16();
            s.set_hl(new_hl);
        }
        0xe2 => {
            // JPO a16
            s.jump_if(State::is_parity_odd);
        }
        0xe3 => {
            // XTHL
            let new_hl = s.pop16();
            let old_hl = s.get_hl_address();
            s.push16(old_hl);
            s.set_hl(new_hl);
        }
        0xe4 => {
            // CPO a16
            s.call_if(State::is_parity_odd);
        }
        0xe5 => {
            // PUSH H
            let hl = s.get_hl_address();
            s.push16(hl);
        }
        0xe6 => {
            // ANI byte
            let addend = s.get_arg(1);
            s.and8(addend);
        }
        0xe7 => { s.rst_to(0x20); } // RST 4
        0xe8 => {
            // RPE
            s.ret_if(|ref s| s.cc.p);
        }
        0xe9 => {
            // PCHL
            s.pc = s.get_hl_address();
        }
        0xea => {
            // JPE a16
            s.jump_if(|ref s| s.cc.p);
        }
        0xeb => {
            // XCHG
            let d = s.d;
            let h = s.h;
            s.d = h;
            s.h = d;

            let e = s.e;
            let l = s.l;
            s.e = l;
            s.l = e;
        }
        0xec => {
            // CPE a16
            s.call_if(|ref s| s.cc.p);
        }
        0xed => {
            // CALL a16
            s.call_if(State::unconditionally);
        }
        0xee => {
            // XRI byte
            let addend = s.get_arg(1);
            s.xor8(addend);
        }
        0xef => { s.rst_to(0x28); }, // RST 5

        0xf0 => {
            // RP
            s.ret_if(State::is_plus);
        }
        0xf1 => {
            // POP PSW
            let word = s.pop16();
            s.a = high_order_byte(word);
            s.cc.deserialize(low_order_byte(word));
        }
        0xf2 => {
            // JP a16
            s.jump_if(State::is_plus);
        }
        0xf3 => {
            s.int_enable = false;
        } // DI
        0xf4 => {
            // CP a16
            s.call_if(State::is_plus);
        }
        0xf5 => {
            // PUSH PSW
            let word = assemble_word(s.a, s.cc.serialize());
            s.push16(word);
        }
        0xf6 => {
            // ORI byte
            let addend = s.get_arg(1);
            s.or8(addend);
        }
        0xf7 => { s.rst_to(0x30); }, // RST 6
        0xf8 => {
            // RM
            s.ret_if(State::is_minus);
        }
        0xf9 => {
            // SPHL
            s.sp = s.get_hl_address();
        }
        0xfa => {
            // JM a16
            s.jump_if(State::is_minus);
        }
        0xfb => {
            s.int_enable = true;
        } // EI
        0xfc => {
            // CM a16
            s.call_if(State::is_minus);
        }
        0xfd => {
            // CALL a16
            s.call_if(State::unconditionally);
        }
        0xfe => {
            // CPI byte
            let addend = s.get_arg(1);
            s.cmp8(addend);
        }
        0xff => { s.rst_to(0x38); } // RST 7

        _ => unimplemented_instruction(s),
    }

    s.advance();
    OPCODE_TIMING[opcode as usize]
}

pub fn trigger_interrupt(s: &mut State, n: u16) {
    let pc = s.pc;
    s.push16(pc);

    s.pc = 0x08 * n;
    s.int_enable = false;
}
