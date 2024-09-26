#![allow(unused)]

use core::panic;
use std::{cmp::Ordering, collections::HashMap, fs::File, io::Read};

use crate::{inst_to_string, Result};
use lib8086::{Arg, Cc, Decoder, Inst, Op, Reg16, Reg8, Sreg, MemAddrT, IoAddrT, OpSizeT};

mod mem;
use mem::{MemOps, MemMap, MemSize};

mod dump;
pub use dump::dump;

mod io;
use io::{IOMap, IoSize};

mod exec;
mod args;

mod hw;
use hw::init_devices;

mod cfg;
pub use cfg::Config;

pub enum Flags {
    Z,
    S,
    P,
    I,
    C,
    O,
    D,
}

#[derive(Debug, Default)]
pub struct Regs {
    pub ax: u16,
    pub bx: u16,
    pub cx: u16,
    pub dx: u16,
    pub sp: u16,
    pub bp: u16,
    pub si: u16,
    pub di: u16,
}

#[derive(Debug, Default)]
pub struct Sregs {
    pub cs: u16,
    pub ds: u16,
    pub ss: u16,
    pub es: u16,
}

pub struct Cpu {
    regs: Regs,
    sregs: Sregs,
    ip: u16,
    flags: u16,
    halted: bool,
    io_map: IOMap,
    mem_map: MemMap,
}

impl Cpu {
    pub fn new(cfg: &Config) -> Result<Self> {
        let mut io_map = IOMap::new();
        let mut mem_map = MemMap::new();

        init_devices(cfg,&mut mem_map, &mut io_map)?;

        Ok(Self {
            regs: Regs::default(),
            sregs: Sregs::default(),
            ip: 0,
            flags: 0,
            halted: false,
            io_map,
            mem_map,
        })
    }

    pub fn dump_regs(&self) {
        println!(
            "AX={:04X} BX={:04X} CX={:04X} DX={:04X} SP={:04X} BP={:04X} SI={:04X} DI={:04X}",
            self.regs.ax,
            self.regs.bx,
            self.regs.cx,
            self.regs.dx,
            self.regs.sp,
            self.regs.bp,
            self.regs.si,
            self.regs.di,
        );

        println!(
            "CS={:04X} DS={:04X} SS={:04X} ES={:04X} IP={:04X} FL={:04X}",
            self.sregs.cs,
            self.sregs.ds,
            self.sregs.ss,
            self.sregs.es,
            self.ip,
            self.flags,
        );
    }

    pub fn read_reg8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::AL => (self.regs.ax & 0xff) as u8,
            Reg8::BL => (self.regs.bx & 0xff) as u8,
            Reg8::CL => (self.regs.cx & 0xff) as u8,
            Reg8::DL => (self.regs.dx & 0xff) as u8,
            Reg8::AH => ((self.regs.ax >> 8) & 0xff) as u8,
            Reg8::BH => ((self.regs.bx >> 8) & 0xff) as u8,
            Reg8::CH => ((self.regs.cx >> 8) & 0xff) as u8,
            Reg8::DH => ((self.regs.dx >> 8) & 0xff) as u8,
        }
    }

    pub fn write_reg8(&mut self, reg: Reg8, val: u8) {
        match reg {
            Reg8::AL => self.regs.ax = (self.regs.ax & 0xff00) | val as u16,
            Reg8::BL => self.regs.bx = (self.regs.bx & 0xff00) | val as u16,
            Reg8::CL => self.regs.cx = (self.regs.cx & 0xff00) | val as u16,
            Reg8::DL => self.regs.dx = (self.regs.dx & 0xff00) | val as u16,
            Reg8::AH => self.regs.ax = (self.regs.ax & 0x00ff) | ((val as u16) << 8),
            Reg8::BH => self.regs.bx = (self.regs.bx & 0x00ff) | ((val as u16) << 8),
            Reg8::CH => self.regs.cx = (self.regs.cx & 0x00ff) | ((val as u16) << 8),
            Reg8::DH => self.regs.dx = (self.regs.dx & 0x00ff) | ((val as u16) << 8),
        }
    }

    pub fn read_reg16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AX => self.regs.ax,
            Reg16::BX => self.regs.bx,
            Reg16::CX => self.regs.cx,
            Reg16::DX => self.regs.dx,
            Reg16::SP => self.regs.sp,
            Reg16::BP => self.regs.bp,
            Reg16::SI => self.regs.si,
            Reg16::DI => self.regs.di,
        }
    }

    pub fn write_reg16(&mut self, reg: Reg16, val: u16) {
        match reg {
            Reg16::AX => self.regs.ax = val,
            Reg16::BX => self.regs.bx = val,
            Reg16::CX => self.regs.cx = val,
            Reg16::DX => self.regs.dx = val,
            Reg16::SP => self.regs.sp = val,
            Reg16::BP => self.regs.bp = val,
            Reg16::SI => self.regs.si = val,
            Reg16::DI => self.regs.di = val,
        }
    }

    pub fn read_sreg(&self, sreg: Sreg) -> u16 {
        match sreg {
            Sreg::CS => self.sregs.cs,
            Sreg::DS => self.sregs.ds,
            Sreg::SS => self.sregs.ss,
            Sreg::ES => self.sregs.es,
        }
    }

    pub fn write_sreg(&mut self, sreg: Sreg, val: u16) {
        match sreg {
            Sreg::CS => self.sregs.cs = val,
            Sreg::DS => self.sregs.ds = val,
            Sreg::SS => self.sregs.ss = val,
            Sreg::ES => self.sregs.es = val,
        }
    }

    // todo: later 
    pub fn calc_ea(&self, seg: Sreg, offset: u16) -> MemAddrT {
        // on 8086, we can access 1MB memory, thus we need to use 32-bit address
        let base = self.read_sreg(seg) as MemAddrT;
        let base = base.wrapping_shl(4);
        let ea = base.wrapping_add(offset as MemAddrT);
        ea
    }

    pub fn read_ip(&self) -> u16 {
        self.ip
    }

    pub fn write_ip(&mut self, val: u16) {
        self.ip = val;
    }

    pub fn read_flags(&self) -> u16 {
        self.flags
    }

    pub fn write_flags(&mut self, val: u16) {
        self.flags = val;
    }

    fn flag_mask(f: Flags) -> u16 {
        match f {
            Flags::O => 0x0800,
            Flags::D => 0x0400,
            Flags::I => 0x0200,
            Flags::Z => 0x0040,
            Flags::S => 0x0080,
            Flags::P => 0x0004,
            Flags::C => 0x0001,
        }
    }

    pub fn set_flag(&mut self, f: Flags) {
        self.flags |= Self::flag_mask(f);
    }

    pub fn clear_flag(&mut self, f: Flags) {
        self.flags &= !Self::flag_mask(f);
    }

    pub fn toggle_flag(&mut self, f: Flags) {
        self.flags ^= Self::flag_mask(f);
    }

    pub fn is_flag_set(&self, f: Flags) -> bool {
        (self.flags & Self::flag_mask(f)) != 0
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn read_mem(&self, seg: Sreg, off: u16, sz: MemSize) -> Option<OpSizeT> {
        let ea = self.calc_ea(seg, off);
        self.mem_map.read(ea, sz)
    }

    pub fn write_mem(&mut self, seg: Sreg, off: u16, val: OpSizeT, sz: MemSize) {
        let ea = self.calc_ea(seg, off);
        self.mem_map.write(ea, val as u16, sz);
    }

    fn io_read(&self, port: u16, sz: IoSize) -> OpSizeT {
        self.io_map.read(port, sz).unwrap_or(OpSizeT::default())
    }

    fn io_write(&mut self, port: IoAddrT, val: OpSizeT, sz: IoSize) {
        self.io_map.write(port, val, sz);
    }
}
