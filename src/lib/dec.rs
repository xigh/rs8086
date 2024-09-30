use crate::op::{Arg, Cc, Inst, Invalid, Op, Reg16, Reg8, Rep, Sreg};


pub struct Decoder<'a> {
    sreg: Option<Sreg>,
    rep: Option<Rep>,
    size: usize,
    lock: bool,
    line: &'a mut dyn Iterator<Item = u8>,
}

impl<'a> Decoder<'a> {
    pub fn new(line: &'a mut dyn Iterator<Item = u8>) -> Decoder<'a> {
        Decoder {
            sreg: None,
            rep: None,
            size: 0,
            lock: false,
            line,
        }
    }
}

fn modrm8(b: u8) -> (Arg, Arg) {
    let modrm = b >> 6;
    let reg = (b >> 3) & 0x7;
    let rm = b & 0x7;
    let arg1 = match modrm {
        0 => Arg::Reg8(From::from(rm)),
        1 => unimplemented!("modrm8 1"),
        2 => unimplemented!("modrm8 2"),
        3 => Arg::Reg8(From::from(rm)),
        _ => unreachable!(),
    };
    let arg2 = Arg::Reg8(From::from(reg));
    (arg1, arg2)
}

fn modrm16(b: u8) -> (Arg, Arg) {
    let modrm = b >> 6;
    let reg = (b >> 3) & 0x7;
    let rm = b & 0x7;
    let arg1 = match modrm {
        0 => Arg::Reg16(From::from(rm)),
        1 => unimplemented!("modrm16 1"),
        2 => unimplemented!("modrm16 2"),
        3 => Arg::Reg16(From::from(rm)),
        _ => unreachable!(),
    };
    let arg2 = Arg::Reg16(From::from(reg));
    (arg1, arg2)
}

impl<'a> Decoder<'a> {
    fn nextb(&mut self) -> Option<u8> {
        let n = self.line.next();
        if n.is_some() {
            self.size += 1;
        }
        n
    }

    fn nextw(&mut self) -> Option<u16> {
        let b1 = self.nextb()?;
        let b2 = self.nextb()?; 
        Some((b2 as u16) << 8 | b1 as u16)
    }

    fn next_0(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Add(a0, a1))
            },
            0x1 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Add(a0, a1))
            }
            0x2 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Add(a1, a0))
            },
            0x3 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Add(a1, a0))
            }
            0x4 => {
                let b1 = self.nextb()?;
                Some(Op::Add(Arg::Reg8(Reg8::AL), Arg::Uimm8(b1)))
            },
            0x5 => {
                let w = self.nextw()?;
                Some(Op::Add(Arg::Reg16(Reg16::AX), Arg::Uimm16(w)))
            },
            0x6 => {
                Some(Op::Push(Arg::Sreg(Sreg::ES)))
            },
            0x7 => {
                Some(Op::Pop(Arg::Sreg(Sreg::ES)))
            },

            0x8 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Or(a0, a1))
            },
            0x9 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Or(a1, a0))
            },
            0xa => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Or(a0, a1))
            },
            0xb => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Or(a1, a0))
            },
            0xc => {
                let b1 = self.nextb()?;
                Some(Op::Or(Arg::Reg8(Reg8::AL), Arg::Uimm8(b1)))
            },
            0xd => {
                let w = self.nextw()?;
                Some(Op::Or(Arg::Reg16(Reg16::AX), Arg::Uimm16(w)))
            },
            0xe => {
                Some(Op::Push(Arg::Sreg(Sreg::CS)))
            },
            0xf => {
                Some(Op::Invalid(Invalid::UnexpectedByte(b0)))
            },

            _ => unreachable!(),
        }
    }

    fn next_1(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Adc(a0, a1))
            }
            0x1 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Adc(a1, a0))
            }
            0x2 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Adc(a0, a1))
            }
            0x3 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Adc(a1, a0))
            }
            0x4 => {
                let b1 = self.nextb()?;
                Some(Op::Adc(Arg::Reg8(Reg8::AL), Arg::Uimm8(b1)))
            }
            0x5 => {
                let w = self.nextw()?;
                Some(Op::Adc(Arg::Reg16(Reg16::AX), Arg::Uimm16(w)))
            }
            0x6 => {
                Some(Op::Push(Arg::Sreg(Sreg::SS)))
            }
            0x7 => {
                Some(Op::Pop(Arg::Sreg(Sreg::SS)))
            }

            0x8 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Sbb(a0, a1))
            }
            0x9 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Sbb(a1, a0))
            }
            0xa => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Sbb(a0, a1))
            }
            0xb => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Sbb(a1, a0))
            }
            0xc => {
                let b1 = self.nextb()?;
                Some(Op::Sbb(Arg::Reg8(Reg8::AL), Arg::Uimm8(b1)))
            }
            0xd => {
                let w = self.nextw()?;
                Some(Op::Sbb(Arg::Reg16(Reg16::AX), Arg::Uimm16(w)))
            }
            0xe => {
                Some(Op::Push(Arg::Sreg(Sreg::DS)))
            }
            0xf => {
                Some(Op::Pop(Arg::Sreg(Sreg::DS)))
            }

            _ => unreachable!(),
        }
    }

    fn next_2(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::And(a0, a1))
            }
            0x1 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::And(a1, a0))
            }
            0x2 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::And(a0, a1))
            }
            0x3 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::And(a1, a0))
            }
            0x4 => {
                let b1 = self.nextb()?;
                Some(Op::And(Arg::Reg8(Reg8::AL), Arg::Uimm8(b1)))
            }
            0x5 => {
                let w = self.nextw()?;
                Some(Op::And(Arg::Reg16(Reg16::AX), Arg::Uimm16(w)))
            }
            0x6 => {
                if self.sreg == Some(Sreg::ES) {
                    return Some(Op::Invalid(Invalid::TooManyPrefix));
                }
                self.sreg = Some(Sreg::ES);
                self.next_o()
            }
            0x7 => {
                Some(Op::Daa)
            }

            0x8 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Sub(a0, a1))
            }
            0x9 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Sub(a1, a0))
            }
            0xa => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Sub(a0, a1))
            }
            0xb => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Sub(a1, a0))
            }
            0xc => {
                let b1 = self.nextb()?;
                Some(Op::Sub(Arg::Reg8(Reg8::AL), Arg::Uimm8(b1)))
            }
            0xd => {
                let w = self.nextw()?;
                Some(Op::Sub(Arg::Reg16(Reg16::AX), Arg::Uimm16(w)))
            }
            0xe => {
                if self.sreg == Some(Sreg::CS) {
                    return Some(Op::Invalid(Invalid::TooManyPrefix));
                }
                self.sreg = Some(Sreg::CS);
                self.next_o()
            }
            0xf => {
                Some(Op::Das)
            }

            _ => unreachable!(),
        }
    }

    fn next_3(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Xor(a0, a1))
            },
            0x1 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Xor(a1, a0))
            },
            0x2 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Xor(a0, a1))
            },
            0x3 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Xor(a1, a0))
            },
            0x4 => {
                let b1 = self.nextb()?;
                Some(Op::Xor(Arg::Reg8(Reg8::AL), Arg::Uimm8(b1)))
            },
            0x5 => {
                let w = self.nextw()?;
                Some(Op::Xor(Arg::Reg16(Reg16::AX), Arg::Uimm16(w)))
            },
            0x6 => {
                if self.sreg == Some(Sreg::SS) {
                    return Some(Op::Invalid(Invalid::TooManyPrefix));
                }
                self.sreg = Some(Sreg::SS);
                self.next_o()
            },
            0x7 => {
                Some(Op::Aaa)
            },

            0x8 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Cmp(a0, a1))
            },
            0x9 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Cmp(a1, a0))
            },
            0xa => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Cmp(a0, a1))
            },
            0xb => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Cmp(a1, a0))
            },
            0xc => {
                let b1 = self.nextb()?;
                Some(Op::Cmp(Arg::Reg8(Reg8::AL), Arg::Uimm8(b1)))
            },
            0xd => {
                let w = self.nextw()?;
                Some(Op::Cmp(Arg::Reg16(Reg16::AX), Arg::Uimm16(w)))
            },
            0xe => {
                if self.sreg == Some(Sreg::DS) {
                    return Some(Op::Invalid(Invalid::TooManyPrefix));
                }
                self.sreg = Some(Sreg::DS);
                self.next_o()
            },
            0xf => {
                Some(Op::Aas)
            },

            _ => unreachable!(),
        }
    }

    fn next_4(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0..=0x7 => Some(Op::Inc(Arg::Reg16(From::from(b0)))),
            0x8..=0xf => Some(Op::Dec(Arg::Reg16(From::from(b0 & 0x7)))),
            _ => unreachable!(),
        }
    }

    fn next_5(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0..=0x7 => Some(Op::Push(Arg::Reg16(From::from(b0)))),
            0x8..=0xf => Some(Op::Pop(Arg::Reg16(From::from(b0 & 0x7)))),
            _ => unreachable!(),
        }
    }

    fn next_6(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            // pusha, popa, push imm8, ...
            0x0..=0xf => Some(Op::Invalid(Invalid::UnexpectedByte(b0))),
            _ => unreachable!(),
        }
    }

    fn next_7(&mut self, b0: u8) -> Option<Op> {
        let b1 = self.nextb()? as i8;
        Some(Op::Jcc(Cc::from(b0 & 0xf), b1))
    }

    fn next_8(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => {
                let b1 = self.nextb()?;
                let (a0, _) = modrm16(b1);
                let ww = self.nextw()?;
                match (b1 >> 3) & 0x7 {
                    0b000 => Some(Op::Add(a0, Arg::Uimm16(ww))),
                    0b001 => Some(Op::Or(a0, Arg::Uimm16(ww))),
                    0b010 => Some(Op::Adc(a0, Arg::Uimm16(ww))),
                    0b011 => Some(Op::Sbb(a0, Arg::Uimm16(ww))),
                    0b100 => Some(Op::And(a0, Arg::Uimm16(ww))),
                    0b101 => Some(Op::Sub(a0, Arg::Uimm16(ww))),
                    0b110 => Some(Op::Xor(a0, Arg::Uimm16(ww))),
                    0b111 => Some(Op::Cmp(a0, Arg::Uimm16(ww))),
                    _ => unreachable!(),
                }
            }
            0x1 => {
                let b1 = self.nextb()?;
                let (a0, _) = modrm16(b1);
                let b = self.nextb()? as i8;
                match (b1 >> 3) & 0x7 {
                    0b000 => Some(Op::Add(a0, Arg::Imm8(b))),
                    0b001 => Some(Op::Or(a0, Arg::Imm8(b))),
                    0b010 => Some(Op::Adc(a0, Arg::Imm8(b))),
                    0b011 => Some(Op::Sbb(a0, Arg::Imm8(b))),
                    0b100 => Some(Op::And(a0, Arg::Imm8(b))),
                    0b101 => Some(Op::Sub(a0, Arg::Imm8(b))),
                    0b110 => Some(Op::Xor(a0, Arg::Imm8(b))),
                    0b111 => Some(Op::Cmp(a0, Arg::Imm8(b))),
                    _ => unreachable!(),
                }
            }
            0x2 => {
                let b1 = self.nextb()?;
                let (a0, _) = modrm16(b1);
                let ww = self.nextw()?;
                match (b1 >> 3) & 0x7 {
                    0b000 => Some(Op::Add(a0, Arg::Uimm16(ww))),
                    0b001 => Some(Op::Or(a0, Arg::Uimm16(ww))),
                    0b010 => Some(Op::Adc(a0, Arg::Uimm16(ww))),
                    0b011 => Some(Op::Sbb(a0, Arg::Uimm16(ww))),
                    0b100 => Some(Op::And(a0, Arg::Uimm16(ww))),
                    0b101 => Some(Op::Sub(a0, Arg::Uimm16(ww))),
                    0b110 => Some(Op::Xor(a0, Arg::Uimm16(ww))),
                    0b111 => Some(Op::Cmp(a0, Arg::Uimm16(ww))),
                    _ => unreachable!(),
                }
            }
            0x3 => {
                let b1 = self.nextb()?;
                let (a0, _) = modrm16(b1);
                let b = self.nextb()? as i8;
                match (b1 >> 3) & 0x7 {
                    0b000 => Some(Op::Add(a0, Arg::Imm8(b))),
                    0b001 => Some(Op::Or(a0, Arg::Imm8(b))),
                    0b010 => Some(Op::Adc(a0, Arg::Imm8(b))),
                    0b011 => Some(Op::Sbb(a0, Arg::Imm8(b))),
                    0b100 => Some(Op::And(a0, Arg::Imm8(b))),
                    0b101 => Some(Op::Sub(a0, Arg::Imm8(b))),
                    0b110 => Some(Op::Xor(a0, Arg::Imm8(b))),
                    0b111 => Some(Op::Cmp(a0, Arg::Imm8(b))),
                    _ => unreachable!(),
                }
            }
            0x4 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Test(a0, a1))
            }
            0x5 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Test(a0, a1))
            }
            0x6 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Xchg(a0, a1))
            }
            0x7 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Xchg(a0, a1))
            }

            0x8 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Mov(a0, a1))
            }
            0x9 => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Mov(a0, a1))
            }
            0xa => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm8(b1);
                Some(Op::Mov(a1, a0))
            }
            0xb => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Mov(a1, a0))
            }

            0xc => {
                Some(Op::Invalid(Invalid::UnexpectedByte(b0)))
            }
            0xd => {
                let b1 = self.nextb()?;
                let (a0, a1) = modrm16(b1);
                Some(Op::Lea(a0, a1))
            }
            0xe => {
                let b1 = self.nextb()?;
                let (_, a1) = modrm16(b1);
                let sr = b1 >> 3;
                if sr & 0b100 != 0 {
                    return Some(Op::Invalid(Invalid::UnexpectedBytes(b0, b1)))
                }
                Some(Op::Mov(Arg::Sreg(Sreg::from(sr)), a1))
            }
            0xf => {
                let b1 = self.nextb()?;
                let (_, a1) = modrm16(b1);
                match (b1 >> 3) & 0x7 {
                    0b000 => Some(Op::Pop(a1)),
                    _ => Some(Op::Invalid(Invalid::UnexpectedBytes(b0, b1))),
                }
            }

            _ => unreachable!(),
        }
    }

    fn next_9(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => Some(Op::Nop),
            0x1..=0xf => unimplemented!("0x{:02x}", b0),
            _ => unreachable!()
        }
    }

    fn next_a(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0..=0xf => unimplemented!("0x{:02x}", b0),
            _ => unreachable!(),
        }
    }

    fn next_b(&mut self, b0: u8) -> Option<Op> {
        let n0 = b0 & 0xf;
        if (0x0..=0x7).contains(&n0) {
            let b1 = self.nextb()?;
            Some(Op::Mov(
                Arg::Reg8(From::from(n0 & 0b11)),
                Arg::Uimm8(b1),
            ))
        } else if (0x8..=0xf).contains(&n0) {
            let w1 = self.nextw()?;
            Some(Op::Mov(
                Arg::Reg16(From::from(n0 & 0b11)),
                Arg::Uimm16(w1),
            ))
        } else {
            unreachable!()
        }
    }

    fn next_c(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => { // 0xc0 -> 
                unimplemented!("0x{:02x}", b0)
            },
            0x1 => { // 0xc1 -> ret
                unimplemented!("0x{:02x}", b0)
            },
            0x2 => { // 0xc2 -> ret
                unimplemented!("0x{:02x}", b0)
            },
            0x3 => { // 0xc3 -> ret
                Some(Op::Ret)
            },
            0x4..=0xf => unimplemented!("0x{:02x}", b0),
            _ => unreachable!(),
        }
    }

    fn next_d(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => {
                unimplemented!("0x{:02x}", b0)
            },
            0x1 => {
                unimplemented!("0x{:02x}", b0)
            },
            0x2 => {
                unimplemented!("0x{:02x}", b0)
            },
            0x3 => {
                unimplemented!("0x{:02x}", b0)
            },
            0x4 => {
                // 0xd4 -> aam
                let b1 = self.nextb()?;
                Some(Op::Aam(b1))
            },
            0x5 => {
                // 0xd5 -> aad
                let b1 = self.nextb()?;
                Some(Op::Aad(b1))
            },
            0x6 => {
                unimplemented!("0x{:02x}", b0)
            },
            0x7 => {
                unimplemented!("0x{:02x}", b0)
            },
            0x8 => {
                unimplemented!("0x{:02x}", b0)
            },
            0x9 => {
                unimplemented!("0x{:02x}", b0)
            },
            0xa => {
                unimplemented!("0x{:02x}", b0)
            },
            0xb => {
                unimplemented!("0x{:02x}", b0)
            },
            0xc => {
                unimplemented!("0x{:02x}", b0)
            },
            0xd => {
                unimplemented!("0x{:02x}", b0)
            },
            0xe => {
                unimplemented!("0x{:02x}", b0)
            },
            0xf => {
                unimplemented!("0x{:02x}", b0)
            },
            _ => unreachable!(),
        }
    }

    fn next_e(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => unimplemented!("0x{:02x}", b0),
            0x1 => unimplemented!("0x{:02x}", b0),
            0x2 => unimplemented!("0x{:02x}", b0),
            0x3 => unimplemented!("0x{:02x}", b0),
            0x4 => {
                // e4 -> in al, imm8
                let b1 = self.nextb()?;
                Some(Op::In(Arg::Reg8(Reg8::AL), Arg::Uimm8(b1)))
            }
            0x5 => unimplemented!("0x{:02x}", b0),
            0x6 => {
                // e6 -> out imm8, al
                let b1 = self.nextb()?;
                Some(Op::Out(Arg::Uimm8(b1), Arg::Reg8(Reg8::AL)))
            }
            0x7 => unimplemented!("0x{:02x}", b0),
            0x8 => unimplemented!("0x{:02x}", b0),
            0x9 => unimplemented!("0x{:02x}", b0),
            0xa => {
                // ea -> jmp far
                let w1 = self.nextw()?;
                let w2 = self.nextw()?;
                Some(Op::JmpFar(
                    Arg::Uimm16(w2), 
                    Arg::Uimm16(w1),
                ))
            }
            0xb => unimplemented!("0x{:02x}", b0),
            0xc => unimplemented!("0x{:02x}", b0),
            0xd => unimplemented!("0x{:02x}", b0),
            0xe => unimplemented!("0x{:02x}", b0),
            0xf => unimplemented!("0x{:02x}", b0),
            _ => unreachable!(),
        }
    }

    fn next_f(&mut self, b0: u8) -> Option<Op> {
        match b0 & 0xf {
            0x0 => {
                if self.lock {
                    return Some(Op::Invalid(Invalid::TooManyPrefix));
                }
                self.lock = true;
                self.next_o()
            }
            0x1 => { // 0xf1 -> int 1
                Some(Op::Invalid(Invalid::UnexpectedByte(b0)))
            },
            0x2 => { // 0xf2 -> repne/repnz
                if self.rep.is_some() {
                    return Some(Op::Invalid(Invalid::TooManyPrefix));
                }
                self.rep = Some(Rep::Repne);
                self.next_o()
            },
            0x3 => { // 0xf3 -> rep/repe/repz
                if self.rep.is_some() {
                    return Some(Op::Invalid(Invalid::TooManyPrefix));
                }
                self.rep = Some(Rep::Rep);
                self.next_o()
            }
            0x4 => { // 0xf4 -> hlt
                Some(Op::Hlt)
            },
            0x5 => { // 0xf5 -> cmc
                Some(Op::Cmc)
            },
            0x6 => { // 0xf6 -> grp3a
                unimplemented!()
            },
            0x7 => { // 0xf7 -> grp3b
                unimplemented!()
            },
            0x8 => { // 0xf8 -> clc
                Some(Op::Clc)
            },
            0x9 => { // 0xf9 -> stc
                Some(Op::Stc)
            },
            0xa => { // 0xfa -> cli
                Some(Op::Cli)
            },
            0xb => { // 0xfb -> sti
                Some(Op::Sti)
            },
            0xc => { // 0xfc -> cld
                Some(Op::Cld)
            },
            0xd => { // 0xfd -> std
                Some(Op::Std)
            },
            0xe => { 
                unimplemented!("0x{:02x}", b0) // grp4
            },
            0xf => {
                unimplemented!("0x{:02x}", b0) // grp5
            }
            _ => unreachable!(),
        }
    }

    fn next_o(&mut self) -> Option<Op> {
        let b0 = self.nextb()?;
        match b0 & 0xf0 {
            0x00 => self.next_0(b0),
            0x10 => self.next_1(b0),
            0x20 => self.next_2(b0),
            0x30 => self.next_3(b0),
            0x40 => self.next_4(b0),
            0x50 => self.next_5(b0),
            0x60 => self.next_6(b0),
            0x70 => self.next_7(b0),
            0x80 => self.next_8(b0),
            0x90 => self.next_9(b0),
            0xa0 => self.next_a(b0),
            0xb0 => self.next_b(b0),
            0xc0 => self.next_c(b0),
            0xd0 => self.next_d(b0),
            0xe0 => self.next_e(b0),
            0xf0 => self.next_f(b0), 
            _ => unreachable!(),
        }
    }

    pub fn next_i(&mut self) -> Option<Inst> {
        self.sreg = None;
        self.size = 0;
        self.lock = false;
        let op= self.next_o().unwrap(); // todo
        Some(Inst {
            lock: self.lock,
            rep: self.rep,
            seg: self.sreg,
            op,
            size: self.size as u8,
        })
    }
}
