use std::path::PathBuf;

use super::MemAddrT;

pub struct Config {
    pub bios_file: PathBuf,
    pub ram_size: MemAddrT,
    pub bios_addr: MemAddrT,
}
