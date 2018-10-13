use bytes::*;
use machine::Machine;
use state::State;
use stack::Stack;
use program::Program;

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
            s.set_bc(s.get_arg16());
        }
        0x02 => {
            // STAX B
            s.memory.set(s.get_bc(), s.a);
        }
        0x03 => {
            // INX B
            s.set_bc(s.get_bc().wrapping_add(1));
        }
        0x04 => {
            // INR B
            s.b = s.b.wrapping_add(1);
            s.set_flags_no_carry(s.b);
        }
        0x05 => {
            // DCR B
            s.b = s.b.wrapping_sub(1);
            s.set_flags_no_carry(s.b);
        }
        0x06 => {
            s.b = s.get_arg8();
        } // MVI B,byte
        0x07 => {
            // RLC
            let x = s.a;
            s.a = x.rotate_left(1);
            s.cc.cy = (x & 0x80) == 1;
        }
        0x08 => (), // NOP
        0x09 => {
            // DAD B
            s.add16(s.get_bc());
        }
        0x0a => {
            // LDAX B
            s.a = s.memory.get(s.get_bc());
        }
        0x0b => {
            // DCX B
            s.set_bc(s.get_bc().wrapping_sub(1));
        }
        0x0c => {
            // INR C
            s.c = s.c.wrapping_add(1);
            s.set_flags_no_carry(s.c);
        }
        0x0d => {
            // DCR C
            s.c = s.c.wrapping_sub(1);
            s.set_flags_no_carry(s.c);
        }
        0x0e => {
            s.c = s.get_arg8();
        } // MVI C,byte
        0x0f => {
            // RRC
            let x = s.a;
            s.a = x.rotate_right(1);
            s.cc.cy = (x & 0x01) == 1;
        }

        0x10 => (), // NOP
        0x11 => {
            // LXI D,word
            s.set_de(s.get_arg16());
        }
        0x12 => {
            // STAX D
            s.memory.set(s.get_de(), s.a);
        }
        0x13 => {
            // INX D
            s.set_de(s.get_de().wrapping_add(1));
        }
        0x14 => {
            // INR D
            s.d = s.d.wrapping_add(1);
            s.set_flags_no_carry(s.d);
        }
        0x15 => {
            // DCR D
            s.d = s.d.wrapping_sub(1);
            s.set_flags_no_carry(s.d);
        }
        0x16 => {
            s.d = s.get_arg8();
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
            s.add16(s.get_de());
        }
        0x1a => {
            // LDAX D
            s.a = s.memory.get(s.get_de());
        }
        0x1b => {
            // DCX D
            s.set_de(s.get_de().wrapping_sub(1));
        }
        0x1c => {
            // INR E
            s.e = s.e.wrapping_add(1);
            s.set_flags_no_carry(s.e);
        }
        0x1d => {
            // DCR E
            s.e = s.e.wrapping_sub(1);
            s.set_flags_no_carry(s.e);
        }
        0x1e => {
            s.e = s.get_arg8();
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
            s.set_hl(s.get_arg16());
        }
        0x22 => {
            // SHLD a16
            let address = s.get_arg16();
            s.memory.set(address, s.l);
            s.memory.set(address + 1, s.h);
        }
        0x23 => {
            // INX H
            s.set_hl(s.get_hl_address().wrapping_add(1));
        }
        0x24 => {
            // INR H
            s.h = s.h.wrapping_add(1);
            s.set_flags_no_carry(s.h);
        }
        0x25 => {
            // DCR H
            s.h = s.h.wrapping_sub(1);
            s.set_flags_no_carry(s.h);
        }
        0x26 => {
            s.h = s.get_arg8();
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
            s.add16(s.get_hl_address());
        }
        0x2a => {
            // LHLD a16
            let address = s.get_arg16();
            s.l = s.memory.get(address);
            s.h = s.memory.get(address + 1);
        }
        0x2b => {
            // DCX H
            s.set_hl(s.get_hl_address().wrapping_sub(1));
        }
        0x2c => {
            // INR L
            s.l = s.l.wrapping_add(1);
            s.set_flags_no_carry(s.l);
        }
        0x2d => {
            // DCR L
            s.l = s.l.wrapping_sub(1);
            s.set_flags_no_carry(s.l);
        }
        0x2e => {
            s.l = s.get_arg8();
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
            s.memory.set(s.get_arg16(), s.a);
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
            s.set_m(s.get_arg8());
        }
        0x37 => {
            s.cc.cy = true;
        } // STC
        0x38 => (), // NOP
        0x39 => {
            // DAD SP
            s.add16(s.sp);
        }
        0x3a => {
            // LDA a16
            s.a = s.memory.get(s.get_arg16());
        }
        0x3b => {
            s.sp -= 1;
        } // DCX SP
        0x3c => {
            // INR A
            s.a = s.a.wrapping_add(1);
            s.set_flags_no_carry(s.a);
        }
        0x3d => {
            // DCR A
            s.a = s.a.wrapping_sub(1);
            s.set_flags_no_carry(s.a);
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
            s.set_m(s.get_operand(opcode));
        }
        0x78..=0x7f => {
            // MOV A, *
            s.a = s.get_operand(opcode);
        }

        0x80..=0x87 => {
            // ADD *
            s.add8(s.get_operand(opcode));
        }
        0x88..=0x8f => {
            // ADC *
            s.adc8(s.get_operand(opcode));
        }

        0x90..=0x97 => {
            // SUB *
            s.sub8(s.get_operand(opcode));
        }
        0x98..=0x9f => {
            // SBB *
            s.sbb8(s.get_operand(opcode));
        }

        0xa0..=0xa7 => {
            // ANA *
            s.and8(s.get_operand(opcode));
        }
        0xa8..=0xaf => {
            // XRA *
            s.xor8(s.get_operand(opcode));
        }

        0xb0..=0xb7 => {
            // ORA *
            s.or8(s.get_operand(opcode));
        }
        0xb8..=0xbf => {
            // CMP *
            s.cmp8(s.get_operand(opcode));
        }

        0xc0 => {
            // RNZ
            s.ret_if(State::is_nz);
        }
        0xc1 => {
            // POP B
            let val = s.pop16();
            s.set_bc(val);
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
            s.push16(s.get_bc());
        }
        0xc6 => {
            // ADI byte
            s.add8(s.get_arg8());
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
            s.adc8(s.get_arg8());
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
            s.push16(s.get_de());
        }
        0xd6 => {
            // SUI byte
            s.sub8(s.get_arg8());
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
            s.sbb8(s.get_arg8());
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
            s.push16(s.get_hl_address());
        }
        0xe6 => {
            // ANI byte
            s.and8(s.get_arg8());
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
            s.xor8(s.get_arg8());
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
            s.push16(assemble_word(s.a, s.cc.serialize()));
        }
        0xf6 => {
            // ORI byte
            s.or8(s.get_arg8());
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
            s.cmp8(s.get_arg8());
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
