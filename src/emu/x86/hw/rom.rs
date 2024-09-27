use std::{
    path::PathBuf,
    fs::File,
    io::Read,
};

use tracing::{info, trace, warn};

use super::{Result, Device, MemMap, IOMap, MemAddrT, OpSizeT, OpSize, MemOps, Config, dump};

pub struct DeviceROM {
    start: MemAddrT,
    bytes: Vec<u8>,
}

impl DeviceROM {
    pub fn register(cfg: &Config, vm: &mut MemMap, io: &mut IOMap) -> Result<()> {
        let mut f = File::open(&cfg.bios_file)?;
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;

        let bios_start = cfg.bios_addr;
        let bios_size = bytes.len() as MemAddrT;
        let bios_end = bios_start + bios_size;
        info!("bios start={:08x}, end={:08x} (size={})", bios_start, bios_end, bios_size);
        dump(&bytes, bios_start as usize, bios_size as usize);

        vm.register(bios_start, bios_end, Box::new(Self{
            start: bios_start,
            bytes,
        }));
        Ok(())
    }
}

impl MemOps for DeviceROM {
    fn name(&self) -> String {
        "ROM".to_string()
    }

    fn read(&self, addr: MemAddrT, sz: OpSize) -> OpSizeT {
        let addr = addr as usize;
        let offset = addr.wrapping_sub(self.start as usize);

        if offset >= self.bytes.len() {
            warn!("read addr={:05x}, sz={:?} [out of range]", addr, sz);
            return 0;
        }
        let b0 = self.bytes[offset] as OpSizeT;

        match sz {
            OpSize::Byte => {
                trace!("read addr={:05x}, offset={:05x}, sz={:?}, val={:02x}", addr, offset, sz, b0);
                b0
            },
            OpSize::Word => {
                if offset + 1 >= self.bytes.len() {
                    warn!("read addr={:05x}, sz={:?} [out of range]", addr, sz);
                    return 0;
                }
                let b1 = self.bytes[offset + 1] as OpSizeT;
                let w = (b0 << 8) | b1;
                info!("read addr={:05x}, offset={:05x}, sz={:?}, val={:04x}", addr, offset, sz, w);
                w
            }
        }
    }

    fn write(&mut self, addr: MemAddrT, data: OpSizeT, sz: OpSize) {
        warn!("write [blocked] addr={:05x}, data={:04x}, sz={:?}", addr, data, sz);
    }
}
