use lib8086::{Arg, Cc, Inst, Op, Reg16, Reg8, Rep, MemAddrT};

pub fn arg_to_string(a: &Arg) -> String {
    match a {
        Arg::Reg8(r) => match r {
            &Reg8::AL => "al",
            &Reg8::CL => "cl",
            &Reg8::DL => "dl",
            &Reg8::BL => "bl",
            &Reg8::AH => "ah",
            &Reg8::CH => "ch",
            &Reg8::DH => "dh",
            &Reg8::BH => "bh",
        }.to_string(),
        Arg::Reg16(r) => match r {
            &Reg16::AX => "ax",
            &Reg16::CX => "cx",
            &Reg16::DX => "dx",
            &Reg16::BX => "bx",
            &Reg16::SP => "sp",
            &Reg16::BP => "bp",
            &Reg16::SI => "si",
            &Reg16::DI => "di",
        }.to_string(),
        Arg::Imm8(i) => format!("0x{:02X}", i),
        Arg::Uimm8(i) => format!("0x{:02X}", i),
        Arg::Imm16(i) => format!("0x{:04X}", i),
        Arg::Uimm16(i) => format!("0x{:04X}", i),
        _ => format!("??? {:?}", a),
    }   
}

pub fn inst_to_string(pc: MemAddrT, inst: &Inst) -> String {
    let mut s = String::new();
    if inst.lock {
        s.push_str("lock ");
    }
    match inst.rep {
        Some(Rep::Rep) => s.push_str("rep "),
        Some(Rep::Repne) => s.push_str("repne "),
        _ => (),
    }

    match &inst.op {
        Op::Nop => s.push_str("nop"),
        Op::Add(a1, a2) => {
            s.push_str("add ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Adc(a1, a2) => {
            s.push_str("adc ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Sbb(a1, a2) => {
            s.push_str("sbb ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Sub(a1, a2) => {
            s.push_str("sub ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::And(a1, a2) => {
            s.push_str("and ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Or(a1, a2) => {
            s.push_str("or ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Xor(a1, a2) => {
            s.push_str("xor ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Cmp(a1, a2) => {
            s.push_str("cmp ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Push(a1) => {
            s.push_str("push ");
            s.push_str(arg_to_string(&a1).as_str());
        },
        Op::Pop(a1) => {
            s.push_str("pop ");
            s.push_str(arg_to_string(&a1).as_str());
        },
        Op::Ret => s.push_str("ret"),

        Op::Aaa => s.push_str("aaa"),

        Op::Aad(b1) => {
            s.push_str("aad ");
            if *b1 != 0xa {
                s.push_str(format!("0x{:02x}", b1).as_str());
            }
        },
        Op::Aam(b1) => {
            s.push_str("aam ");
            if *b1 != 0xa {
                s.push_str(format!("0x{:02x}", b1).as_str());
            }
        },


        Op::Aas => s.push_str("aas"),

        Op::Daa => s.push_str("daa"),
        Op::Das => s.push_str("das"),

        Op::Inc(_) => s.push_str("inc"),
        Op::Dec(_) => s.push_str("dec"),
        Op::Jcc(cc, disp) => {
            s.push_str("j");
            s.push_str(match cc {
                Cc::O => "o",
                Cc::NO => "no",
                Cc::B => "b",
                Cc::NB => "nb",
                Cc::E => "e",
                Cc::NE => "ne",
                Cc::BE => "be",
                Cc::NBE => "nbe",
                Cc::S => "s",
                Cc::NS => "ns",
                Cc::P => "p",
                Cc::NP => "np",
                Cc::L => "l",
                Cc::NL => "nl",
                Cc::LE => "le",
                Cc::NLE => "nle",
            });
            s.push_str(" ");
            s.push_str(format!("0x{:04x}", pc as i32 + inst.size as i32 + *disp as i32).as_str());
        },
        Op::JmpFar(a1, a2) => {
            s.push_str("jmp far ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(":");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Test(a1, a2) => {
            s.push_str("test ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Xchg(a1, a2) => {
            s.push_str("xchg ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Mov(a1, a2) => {
            s.push_str("mov ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Lea(_, _) => s.push_str("lea"),
        Op::In(a1, a2) => {
            s.push_str("in ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        },
        Op::Out(a1, a2) => {
            s.push_str("out ");
            s.push_str(arg_to_string(&a1).as_str());
            s.push_str(", ");
            s.push_str(arg_to_string(&a2).as_str());
        }
        Op::Hlt => {
            s.push_str("hlt");
        }
        Op::Cmc => {
            s.push_str("cmc");
        }
        Op::Clc => {
            s.push_str("clc");
        }
        Op::Stc => {
            s.push_str("stc");
        }
        Op::Cli => {
            s.push_str("cli");
        }
        Op::Sti => {
            s.push_str("sti");
        }
        Op::Cld => {
            s.push_str("cld");
        }
        Op::Std => {
            s.push_str("std");
        }
        Op::Error => s.push_str("error"),
        Op::Invalid(_) => s.push_str("invalid"),
    }
    
    s
}
