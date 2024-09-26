use std::path::PathBuf;

use super::MemAddrT;

pub struct Config {
    pub bios_file: PathBuf,
    pub bios_addr: MemAddrT,
}
