use std::cmp::Ordering;

use tracing::{debug, info, trace};

use lib8086::{Cc, Inst, Op, Reg16, Reg8, Sreg};

use crate::x86::MemAddrT;

use super::{Arg, Cpu, Decoder, Flags, OpSize};

impl Cpu {
    pub fn next_inst(&mut self) -> (Inst, u32, Vec<u8>) {
        struct EA<'a> {
            ip: u16,
            cpu: &'a Cpu,
            bytes: Vec<u8>,
        }

        impl<'a> Iterator for EA<'a> {
            type Item = u8;

            fn next(&mut self) -> Option<u8> {
                let Some(v) = self.cpu.read_mem(Sreg::CS, self.ip, OpSize::Byte) else {
                    return None;
                };
                self.bytes.push(v as u8);
                self.ip += 1;
                Some(v as u8)
            }
        }

        let ip = self.read_ip();
        let pc = self.calc_ea(Sreg::CS, ip);

        trace!("exec: pc={:04x}", pc);

        let mut ea = EA {
            ip,
            cpu: self,
            bytes: vec![],
        };
        let mut dec = Decoder::new(&mut ea);

        let inst = dec.next_i().unwrap(); // !!! we have to do something about this

        let bytes = ea.bytes;        
        (inst, pc, bytes)
    }

    pub fn tick(&mut self) {
        let (inst, _, _) = self.next_inst();
        debug!("tick: inst={:?}", inst);

        let mut nip = self.read_ip() + inst.size as u16;
        match inst.op {
            Op::Nop => (),
            Op::Add(a1, a2) => {
                let v1 = self.read_arg(&a1);
                let v2 = self.read_arg(&a2);
                let nv = v1.wrapping_add(v2);
                self.write_arg(&a1, nv);
                // todo: set flags
                match v1.cmp(&v2) {
                    Ordering::Less => self.set_flag(Flags::C),
                    Ordering::Equal => self.set_flag(Flags::Z),
                    Ordering::Greater => self.clear_flag(Flags::C),
                }
            }
            Op::Adc(a1, a2) => {
                let v1 = self.read_arg(&a1);
                let v2 = self.read_arg(&a2);
                let cf = self.is_flag_set(Flags::C) as u16;
                let nv = v1.wrapping_add(v2).wrapping_add(cf);
                self.write_arg(&a1, nv);
                // todo: set flags
            }
            Op::Sbb(a1, a2) => {
                let v1 = self.read_arg(&a1);
                let v2 = self.read_arg(&a2);
                let cf = self.is_flag_set(Flags::C) as u16;
                let nv = v1.wrapping_sub(v2).wrapping_sub(cf);
                self.write_arg(&a1, nv);
                // todo: set flags
            }
            Op::Sub(a1, a2) => {
                let v1 = self.read_arg(&a1);
                let v2 = self.read_arg(&a2);
                let nv = v1.wrapping_sub(v2);
                self.write_arg(&a1, nv);
                // todo: set flags
            }
            Op::And(a1, a2) => {
                let v1 = self.read_arg(&a1);
                let v2 = self.read_arg(&a2);
                self.write_arg(&a1, v1 & v2);
                // todo: set flags
            }
            Op::Or(a1, a2) => {
                let v1 = self.read_arg(&a1);
                let v2 = self.read_arg(&a2);
                self.write_arg(&a1, v1 | v2);
            }
            Op::Xor(a1, a2) => {
                let v1 = self.read_arg(&a1);
                let v2 = self.read_arg(&a2);
                self.write_arg(&a1, v1 ^ v2);
            }
            Op::Cmp(a1, a2) => {
                let v1 = self.read_arg(&a1);
                let v2 = self.read_arg(&a2);
                let res = v1.wrapping_sub(v2);

                // todo: do something clever with the flags
                match v1.cmp(&v2) {
                    Ordering::Less => self.set_flag(Flags::C),
                    Ordering::Equal => self.set_flag(Flags::Z),
                    Ordering::Greater => self.clear_flag(Flags::C),
                }
                match (v1 as i16).cmp(&(v2 as i16)) {
                    Ordering::Less => self.set_flag(Flags::S),
                    Ordering::Equal => self.set_flag(Flags::Z),
                    Ordering::Greater => self.clear_flag(Flags::S),
                }
            }
            Op::Call(a1) => match a1 {
                Arg::Imm16(rel16) => {
                    info!(" - CALL rel={} [nip={:04x}]", rel16, nip);
                    let sp = self.read_reg16(Reg16::SP);
                    let ss = self.read_sreg(Sreg::SS);
                    trace!(" - CALL: sp={:04x} <- 0x{:04x}", sp, nip);
                    self.write_mem(Sreg::SS, sp, nip, OpSize::Word);
                    self.write_reg16(Reg16::SP, sp.wrapping_sub(2));
                    nip = nip.wrapping_add_signed(rel16);
                    trace!(" - CALL: nip set to {:04x}", nip);
                }
                _ => panic!("unknown form of call"),
            },
            Op::Ret => {
                let nip_copy = nip;
                let sp = self.read_reg16(Reg16::SP);
                let nsp = sp.wrapping_add(2);
                trace!(" - RET: nsp={:04x}", nsp);
                nip = self.read_mem(Sreg::SS, nsp, OpSize::Word).unwrap();
                trace!(" - RET nip={:04x}", nip);
                self.write_reg16(Reg16::SP, nsp);
            }
            Op::Push(a1) => {
                let v = self.read_arg(&a1);
                let sp = self.read_reg16(Reg16::SP);
                let ss = self.read_sreg(Sreg::SS);
                trace!(" - PUSH: sp={:04x}", sp);

                self.write_mem(Sreg::SS, sp, v, OpSize::Word);
                self.write_reg16(Reg16::SP, sp.wrapping_sub(2));
            }
            Op::Pop(a1) => {
                let sp = self.read_reg16(Reg16::SP);
                let nsp = sp.wrapping_add(2);
                trace!(" - POP: nsp={:04x}", sp);
                let v = self.read_mem(Sreg::SS, nsp, OpSize::Word).unwrap();
                trace!(" - POP {:?} <- {:04x}", a1, v);
                self.write_arg(&a1, v as u16);
                self.write_reg16(Reg16::SP, nsp);
            }
            Op::Daa => todo!(),
            Op::Das => todo!(),
            Op::Aaa => {
                // todo: not sure if this is correct
                let mut ax = self.read_reg16(Reg16::AX);
                let mut ah = (ax >> 8) as u8;
                let mut al = (ax & 0xFF) as u8;
                let al_low = ax & 0x0F;
                if al_low > 9 || self.is_flag_set(Flags::A) {
                    // https://www.pcjs.org/documents/manuals/intel/8086/ops/AAA/

                    // todo: check !!!! this is what happens in 286+
                    // ax = ax.wrapping_add(0x106);

                    // todo: but this is what we want for 8086:
                    al = al.wrapping_add(0x06);
                    ah = ah.wrapping_add(0x01);
                    ax = (ah as u16) << 8 | al as u16;

                    self.set_flag(Flags::A);
                    self.set_flag(Flags::C);
                } else {
                    self.clear_flag(Flags::A);
                    self.clear_flag(Flags::C);
                }
                ax = ax & 0xFF0F;
                self.write_reg16(Reg16::AX, ax);
            }
            Op::Aad(b1) => {
                // todo: not sure if this is correct
                let al = self.read_reg8(Reg8::AL);
                let ah = self.read_reg8(Reg8::AH);
                let al = al + (ah * b1 as u8);
                self.write_reg8(Reg8::AL, al);
                self.write_reg8(Reg8::AH, 0);
            }
            Op::Aam(b1) => {
                // todo: not sure if this is correct
                let al = self.read_reg8(Reg8::AL);
                let ah = al / b1;
                self.write_reg8(Reg8::AL, al % b1);
                self.write_reg8(Reg8::AH, ah);
            }
            Op::Aas => {
                // todo: not sure if this is correct
                let mut ax = self.read_reg16(Reg16::AX);
                let al_low = ax & 0x0F;
                if al_low > 9 || self.is_flag_set(Flags::A) {
                    ax = ax.wrapping_sub(0x0600);
                    let ah = (ax >> 8) as u8;
                    let ah = ah.wrapping_sub(1);
                    let al = ax & 0xff;
                    ax = (ah as u16) << 8 | al as u16;
                    self.set_flag(Flags::A);
                    self.set_flag(Flags::C);
                } else {
                    self.clear_flag(Flags::A);
                    self.clear_flag(Flags::C);
                    ax = ax & 0xFF0F;
                }
                self.write_reg16(Reg16::AX, ax);
            }
            Op::Inc(_) => todo!(),
            Op::Dec(_) => todo!(),
            Op::Jcc(cc, disp) => {
                let cond = match cc {
                    Cc::O => self.is_flag_set(Flags::C),
                    Cc::NO => !self.is_flag_set(Flags::C),
                    Cc::B => self.is_flag_set(Flags::C),
                    Cc::NB => !self.is_flag_set(Flags::C),
                    Cc::E => self.is_flag_set(Flags::Z),
                    Cc::NE => !self.is_flag_set(Flags::Z),
                    Cc::BE => self.is_flag_set(Flags::C) || self.is_flag_set(Flags::Z),
                    Cc::NBE => !self.is_flag_set(Flags::C) && !self.is_flag_set(Flags::Z),
                    Cc::S => self.is_flag_set(Flags::S),
                    Cc::NS => !self.is_flag_set(Flags::S),
                    Cc::P => self.is_flag_set(Flags::P),
                    Cc::NP => !self.is_flag_set(Flags::P),
                    Cc::L => self.is_flag_set(Flags::S) != self.is_flag_set(Flags::O),
                    Cc::NL => self.is_flag_set(Flags::S) == self.is_flag_set(Flags::O),
                    Cc::LE => {
                        self.is_flag_set(Flags::Z)
                            || (self.is_flag_set(Flags::S) != self.is_flag_set(Flags::O))
                    }
                    Cc::NLE => {
                        !self.is_flag_set(Flags::Z)
                            && (self.is_flag_set(Flags::S) == self.is_flag_set(Flags::O))
                    }
                };
                trace!(" - J: cc={:?} cond={}", cc, cond);
                if cond {
                    nip = nip.wrapping_add(disp as u16);
                }
            }
            Op::JmpFar(seg, off) => {
                let seg = self.read_arg(&seg);
                self.write_sreg(Sreg::CS, seg);
                nip = self.read_arg(&off);
            }
            Op::Cbw => {
                let al = self.read_reg8(Reg8::AL);
                let signed_al = al as i8;
                let signed_ax = signed_al as i16;
                self.write_reg16(Reg16::AX, signed_ax as u16);
            }
            Op::Cwd => {
                let ax = self.read_reg16(Reg16::AX);
                let signed_ax = ax as i16;
                let signed_dx_ax = signed_ax as i32;
                let dx_ax = signed_dx_ax as u32;
                let ax = (dx_ax & 0xffff) as u16;
                let dx = (dx_ax >> 16) as u16;
                self.write_reg16(Reg16::AX, ax);
                self.write_reg16(Reg16::DX, dx);
            }
            Op::Test(_, _) => todo!(),
            Op::Xchg(_, _) => todo!(),
            Op::Mov(a1, a2) => {
                let v2 = self.read_arg(&a2);
                trace!("MOV {:?} <- {:04X}", a1, v2);
                self.write_arg(&a1, v2);
            }
            Op::Lea(_, _) => todo!(),
            Op::In(a1, a2) => {
                let port = self.read_arg(&a1);
                let val = self.read_arg(&a2);
                trace!("IN: port {:04X} -> {:04X}", port, val);
            }
            Op::Out(a1, a2) => {
                let port = self.read_arg(&a1);
                let val = self.read_arg(&a2);
                self.write_io(port, val, OpSize::Byte);
            }
            Op::Hlt => {
                self.halted = true;
            }
            Op::Cmc => {
                self.toggle_flag(Flags::C);
            }
            Op::Clc => {
                self.clear_flag(Flags::C);
            }
            Op::Stc => {
                self.set_flag(Flags::C);
            }
            Op::Cli => {
                self.clear_flag(Flags::I);
            }
            Op::Sti => {
                self.set_flag(Flags::I);
            }
            Op::Cld => {
                self.clear_flag(Flags::D);
            }
            Op::Std => {
                self.set_flag(Flags::D);
            }

            Op::Error => todo!(),
            Op::Invalid(_) => todo!(),
        }

        self.write_ip(nip);
    }
}
