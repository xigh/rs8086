pub type MemAddrT = u32;
pub type IoAddrT = u16;
pub type OpSizeT = u16;

mod op;
pub use op::{Op, Rep, Inst, Arg, Invalid, Sreg, Cc, Reg16, Reg8};

mod dec;
pub use dec::Decoder;
