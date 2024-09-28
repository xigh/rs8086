use std::env::args;

use tracing::{error, Level};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod emu;
use emu::{emulate, EmuOpts};

mod dis;
pub use dis::inst_to_string;

mod x86;
pub use x86::{Config, Cpu, Flags, OpSize};

pub use lib8086::{Arg, Cc, Decoder, Inst, Op, OpSizeT, Reg16, Reg8, Rep, Sreg};

fn main() -> Result<()> {
    let mut binary = String::new();
    let mut opts = EmuOpts::default();
    let mut level = Level::ERROR;
    let mut hide_header = false;

    for arg in args().skip(1) {
        if arg.starts_with("-") {
            if arg == "-test" {
                opts.test_mode = true;
                continue;
            }

            if arg == "-wait-for-enter" {
                opts.wait_for_enter = true;
                continue;
            }

            if arg == "-log-trace" {
                level = Level::TRACE;
                continue;
            }

            if arg == "-log-debug" {
                level = Level::DEBUG;
                continue;
            }

            if arg == "-log-info" {
                level = Level::INFO;
                continue;
            }

            if arg == "-log-warn" {
                level = Level::WARN;
                continue;
            }

            if arg == "-log-error" {
                level = Level::ERROR;
                continue;
            }

            if arg == "-dump-regs-each-step" {
                opts.dump_regs_each_step = true;
                continue;
            }

            if arg == "-dump-regs-on-halt" {
                opts.dump_regs_on_halt = true;
                continue;
            }

            if arg == "-hide-header" {
                hide_header = true;
                continue;
            }

            eprintln!("Unknown option: {}", arg);
            return Ok(());
        }

        if !binary.is_empty() {
            error!("Multiple files are not supported");
            return Ok(());
        }

        binary = arg;
    }

    if !hide_header {
        println!("rs8086 Emulator v{}", env!("CARGO_PKG_VERSION"));
        println!("Copyright (C) 2024 Philippe Anel <philippe@dremml.com>\n");
    }

    tracing_subscriber::fmt()
        .with_max_level(level)
        .without_time()
        .with_target(true)
        .init();

    emulate(&binary, opts)
}
