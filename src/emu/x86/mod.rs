#![allow(unused)]

use core::panic;
use std::{cmp::Ordering, collections::HashMap, fs::File, io::Read};

use tracing::debug;

use crate::{inst_to_string, Result};
use lib8086::{Arg, Cc, Decoder, Inst, Op, Reg16, Reg8, Sreg, MemAddrT, IoAddrT, OpSizeT};

#[derive(Debug, Clone, Copy)]
pub enum OpSize {
    Byte,
    Word,
}

mod mem;
use mem::{MemOps, MemMap};

mod dump;
pub use dump::dump;

mod io;
use io::{IOOps, IOMap};

mod exec;
mod args;

mod hw;
use hw::init_devices;

mod cfg;
pub use cfg::Config;

#[derive(Debug, Clone, Copy)]
pub enum Flags {
    C = 0,
    P = 2,
    A = 4,
    Z = 6,
    S = 7,
    T = 8,
    I = 9,
    D = 10,
    O = 11,
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
            "CS={:04X} DS={:04X} SS={:04X} ES={:04X} IP={:04X}",
            self.sregs.cs,
            self.sregs.ds,
            self.sregs.ss,
            self.sregs.es,
            self.ip,
        );

        println!("FL={:04X} CF={:01X} PF={:01X} AF={:01X} ZF={:01X} SF={:01X} TF={:01X} IF={:01X} DF={:01X} OF={:01X}",
            self.flags,
            self.is_flag_set(Flags::C) as u8,
            self.is_flag_set(Flags::P) as u8,
            self.is_flag_set(Flags::A) as u8,
            self.is_flag_set(Flags::Z) as u8,
            self.is_flag_set(Flags::S) as u8,
            self.is_flag_set(Flags::T) as u8,
            self.is_flag_set(Flags::I) as u8,
            self.is_flag_set(Flags::D) as u8,
            self.is_flag_set(Flags::O) as u8,
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
        debug!("write_ip: val={:04x}", val);
        self.ip = val;
    }

    pub fn read_flags(&self) -> u16 {
        self.flags
    }

    pub fn write_flags(&mut self, val: u16) {
        self.flags = val;
    }

    fn flag_mask(f: Flags) -> u16 {
        1 << f as u16
    }

    pub fn set_flag(&mut self, f: Flags) {
        let m = Self::flag_mask(f);
        self.flags |= m;
        debug!("set_flag: f={:?} mask={:04x} flags={:04x}", f, m, self.flags);
    }

    pub fn clear_flag(&mut self, f: Flags) {
        let m = Self::flag_mask(f);
        self.flags &= !m;
        debug!("clear_flag: f={:?} mask={:04x} flags={:04x}", f, m, self.flags);
    }

    pub fn toggle_flag(&mut self, f: Flags) {
        let m = Self::flag_mask(f);
        self.flags ^= m;
        debug!("toggle_flag: f={:?} mask={:04x} flags={:04x}", f, m, self.flags);
    }

    pub fn is_flag_set(&self, f: Flags) -> bool {
        let m = Self::flag_mask(f);
        (self.flags & m) != 0
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn read_mem_ea(&self, ea: MemAddrT, sz: OpSize) -> Option<OpSizeT> {
        self.mem_map.read(ea, sz)
    }

    pub fn write_mem_ea(&mut self, ea: MemAddrT, val: OpSizeT, sz: OpSize) {
        self.mem_map.write(ea, val, sz);
    }

    pub fn read_mem(&self, seg: Sreg, off: u16, sz: OpSize) -> Option<OpSizeT> {
        let ea = self.calc_ea(seg, off);
        self.read_mem_ea(ea, sz)
    }

    pub fn write_mem(&mut self, seg: Sreg, off: u16, val: OpSizeT, sz: OpSize) {
        let ea = self.calc_ea(seg, off);
        self.write_mem_ea(ea, val, sz);
    }

    pub fn read_io(&self, port: u16, sz: OpSize) -> OpSizeT {
        self.io_map.read(port, sz).unwrap_or(OpSizeT::default())
    }

    pub fn write_io(&mut self, port: IoAddrT, val: OpSizeT, sz: OpSize) {
        self.io_map.write(port, val, sz);
    }
}
