use std::{env::args, io::Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use lib8086::Decoder;

mod dis;
use dis::inst_to_string;

fn main() -> Result<()> {
    for arg in args().skip(1) {
        disasm(&arg)?;
    }

    Ok(())
}

fn disasm(file: &str) -> Result<()> {
    let mut f = std::fs::File::open(file)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    let mut pc = 0;
    let mut it = buf.iter().cloned();
    let mut dec = Decoder::new(&mut it);
    while let Some(inst) = dec.next_i() {
        let size = inst.size as usize;
        let npc = pc + size;
        let bytes = &buf[pc..npc];
        let bytes = bytes
            .iter()
            .map(|b| format!("{:02x}", *b))
            .collect::<Vec<String>>();
        println!("{:05X} {:16} {}", // ! 05x -> 1MB max
            pc,
            bytes.join(" "),
            inst_to_string(pc as u32, &inst));
        pc = npc;
    }

    Ok(())
}