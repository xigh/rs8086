use lib8086::Arg;

use super::{Cpu, OpSize};

impl Cpu {
    pub fn arg_size(&self, arg: &Arg) -> OpSize {
        match arg {
            Arg::Reg8(_) => OpSize::Byte,
            Arg::Reg16(_) => OpSize::Word,
            Arg::Imm8(_) => OpSize::Byte,
            Arg::Uimm8(_) => OpSize::Byte,
            Arg::Imm16(_) => OpSize::Word,
            Arg::Uimm16(_) => OpSize::Word,
            Arg::Sreg(_) => OpSize::Word,
            _ => unimplemented!(),
        }
    }

    pub fn read_arg(&self, arg: &Arg) -> u16 {
        match arg {
            Arg::Reg8(reg) => {
                let b = self.read_reg8(*reg);
                println!(" - read-arg: reg8 {:?} = {:02X}", reg, b);
                b as u16
            }
            Arg::Reg16(reg) => {
                let w = self.read_reg16(*reg);
                println!(" - read-arg: reg16 {:?} = {:04X}", reg, w);
                w
            }
            Arg::Imm8(imm) => *imm as u16,
            Arg::Uimm8(imm) => *imm as u16,
            Arg::Imm16(imm) => *imm as u16,
            Arg::Uimm16(imm) => *imm as u16,
            Arg::Sreg(sreg) => self.read_sreg(*sreg),
            Arg::Mem(_mem) => unimplemented!(),
        }
    }

    pub fn write_arg(&mut self, arg: &Arg, val: u16) {
        match arg {
            Arg::Reg8(reg) => {
                let val = val as u8;
                println!(" - write-arg: reg8 {:?} = {:02X}", reg, val);
                self.write_reg8(*reg, (val & 0xff) as u8);
            }
            Arg::Reg16(reg) => {
                println!(" - write-arg: reg16 {:?} = {:04X}", reg, val);
                self.write_reg16(*reg, val);
            }
            Arg::Imm8(_) => panic!("Cannot write to imm8"),
            Arg::Uimm8(_) => panic!("Cannot write to uimm8"),
            Arg::Imm16(_) => panic!("Cannot write to imm16"),
            Arg::Uimm16(_) => panic!("Cannot write to uimm16"),
            Arg::Sreg(sreg) => self.write_sreg(*sreg, val),
            Arg::Mem(_mem) => unimplemented!(),
        }
    }
}