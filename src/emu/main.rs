use std::env::args;

use tracing::{error, Level};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod emu;
use emu::emulate;

mod dis;
pub use dis::inst_to_string;

mod x86;
pub use x86::{Cpu, Config};

pub use lib8086::{Arg, Cc, Decoder, Inst, Op, Reg16, Reg8, Rep, Sreg, OpSizeT};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .without_time()
        .with_target(true)
        .init();

    println!("8086 Emulator");
    println!("Copyright (C) 2024 Philippe Anel <philippe@dremml.com>\n");

    let mut binary = String::new();

    for arg in args().skip(1) {
        if arg.starts_with("-") {
            error!("Unknown option: {}", arg);
            continue;
        }

        if !binary.is_empty() {
            error!("Multiple files are not supported");
            return Ok(());
        }

        binary = arg;
    }

    emulate(&binary)
}
