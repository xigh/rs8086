use std::env::args;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod emu;
use emu::emulate;

mod dis;
pub use dis::inst_to_string;

mod x86;
pub use x86::{Cpu, Config};

pub use lib8086::{Arg, Cc, Decoder, Inst, Op, Reg16, Reg8, Rep, Sreg};

fn main() -> Result<()> {
    println!("8086 Emulator");
    println!("Copyright (C) 2024 Philippe Anel <philippe@dremml.com>");
    println!();

    let mut binary = String::new();

    for arg in args().skip(1) {
        if arg.starts_with("-") {
            println!("Unknown option: {}", arg);
            continue;
        }

        if !binary.is_empty() {
            println!("Multiple files are not supported");
            return Ok(());
        }

        binary = arg;
    }

    emulate(&binary)?;

    Ok(())
}
