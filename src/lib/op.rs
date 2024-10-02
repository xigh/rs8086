#[derive(Debug)]
pub struct Inst {
    pub lock: bool,
    pub rep: Option<Rep>,
    pub seg: Option<Sreg>,
    pub op: Op,
    pub size: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rep {
    Rep,
    Repne,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Nop,

    Add(Arg, Arg),
    Adc(Arg, Arg),
    Sbb(Arg, Arg),
    Sub(Arg, Arg),

    And(Arg, Arg),
    Or(Arg, Arg),
    Xor(Arg, Arg),
    Cmp(Arg, Arg),

    Push(Arg),
    Pop(Arg),

    Aaa,
    Aad(u8),
    Aam(u8),
    Aas,

    Daa,
    Das,

    Inc(Arg),
    Dec(Arg),

    Jcc(Cc, i8),

    Call(Arg),
    Ret,

    JmpFar(Arg, Arg), // Arg can be Far

    Test(Arg, Arg),
    Xchg(Arg, Arg),

    Mov(Arg, Arg),
    Lea(Arg, Arg),

    In(Arg, Arg),
    Out(Arg, Arg),

    Cbw,
    Cwd,

    Hlt,
    Cmc,
    Clc,
    Stc,
    Cli,
    Sti,
    Cld,
    Std,

    Error,

    Invalid(Invalid),
}

#[derive(Debug, PartialEq)]
pub enum Cc {
    O,
    NO,
    B,
    NB,
    E,
    NE,
    BE,
    NBE,
    S,
    NS,
    P,
    NP,
    L,
    NL,
    LE,
    NLE,
}

impl From<u8> for Cc {
    fn from(b: u8) -> Self {
        match b {
            0 => Cc::O,
            1 => Cc::NO,
            2 => Cc::B,
            3 => Cc::NB,
            4 => Cc::E,
            5 => Cc::NE,
            6 => Cc::BE,
            7 => Cc::NBE,
            8 => Cc::S,
            9 => Cc::NS,
            10 => Cc::P,
            11 => Cc::NP,
            12 => Cc::L,
            13 => Cc::NL,
            14 => Cc::LE,
            15 => Cc::NLE,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Invalid {
    Unknown, // todo
    TooManyPrefix,
    UnexpectedByte(u8),
    UnexpectedBytes(u8, u8),
}

#[derive(Debug, PartialEq)]
pub enum Arg {
    Reg8(Reg8),
    Reg16(Reg16),
    Imm8(i8),
    Uimm8(u8),
    Imm16(i16),
    Uimm16(u16),
    Sreg(Sreg),
    Mem(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sreg {
    ES,
    CS,
    SS,
    DS,
}

impl From<u8> for Sreg {
    fn from(b: u8) -> Self {
        match b {
            0 => Sreg::ES,
            1 => Sreg::CS,
            2 => Sreg::SS,
            3 => Sreg::DS,
            _ => unreachable!("unexpected value Sreg={}", b),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg8 {
    AL = 0,
    CL = 1,
    DL = 2,
    BL = 3,
    AH = 4,
    CH = 5,
    DH = 6,
    BH = 7,
}

impl Into<u8> for Reg8 {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Reg8 {
    fn from(b: u8) -> Self {
        match b {
            0 => Reg8::AL,
            1 => Reg8::CL,
            2 => Reg8::DL,
            3 => Reg8::BL,
            4 => Reg8::AH,
            5 => Reg8::CH,
            6 => Reg8::DH,
            7 => Reg8::BH,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reg16 {
    AX = 0,
    CX = 1,
    DX = 2,
    BX = 3,
    SP = 4,
    BP = 5,
    SI = 6,
    DI = 7,
}

impl Into<u8> for Reg16 {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Reg16 {
    fn from(b: u8) -> Self {
        match b {
            0 => Reg16::AX,
            1 => Reg16::CX,
            2 => Reg16::DX,
            3 => Reg16::BX,
            4 => Reg16::SP,
            5 => Reg16::BP,
            6 => Reg16::SI,
            7 => Reg16::DI,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mem {
    Reg(Reg16),
    RegOff(Reg16, i8),
    RegOff16(Reg16, i16),
    RegOff32(Reg16, i32),
}
