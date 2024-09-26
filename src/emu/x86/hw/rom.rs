use std::{
    path::PathBuf,
    fs::File,
    io::Read,
};

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
        dump(&bytes, bios_start as usize, 64);
    
        let bios_size = bytes.len();
        println!("device_rom: rom size= {}", bios_size);
        
        let legacy_bios_size = bios_size.min(0x10000) as MemAddrT;
        let bios_end = bios_start + legacy_bios_size;
        println!("device_rom: bios start={:08x}, end={:08x}", bios_start, bios_end);

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
            println!("device_rom: read addr={:05x}, sz={:?} [out of range]", addr, sz);
            return 0;
        }
        let b0 = self.bytes[offset] as OpSizeT;

        match sz {
            OpSize::Byte => {
                println!("device_rom: read addr={:05x}, offset={:05x}, sz={:?}, val={:02x}", addr, offset, sz, b0);
                b0
            },
            OpSize::Word => {
                if offset + 1 >= self.bytes.len() {
                    println!("device_rom: read addr={:05x}, sz={:?} [out of range]", addr, sz);
                    return 0;
                }
                let b1 = self.bytes[offset + 1] as OpSizeT;
                let w = (b0 << 8) | b1;
                println!("device_rom: read addr={:05x}, offset={:05x}, sz={:?}, val={:04x}", addr, offset, sz, w);
                w
            }
        }
    }

    fn write(&mut self, addr: MemAddrT, data: OpSizeT, sz: OpSize) {
        // do nothing
    }
}
