use tracing::{info, warn, trace};

use super::{Result, Device, MemMap, IOMap, MemAddrT, OpSizeT, OpSize, MemOps, Config};

pub struct DeviceRAM {
    start: MemAddrT,
    bytes: Vec<u8>,
}

impl DeviceRAM {
    pub fn register(cfg: &Config, vm: &mut MemMap, io: &mut IOMap) -> Result<()> {
        let dev = Self{
            start: 0,
            bytes: vec![0; cfg.ram_size as usize],
        };

        // todo: handle overlap with rom and device ordering
        vm.register(0x00000, 0xf0000, Box::new(dev));

        Ok(())
    }
}

impl MemOps for DeviceRAM {
    fn name(&self) -> String {
        "RAM".to_string()
    }

    fn read(&self, addr: MemAddrT, sz: OpSize) -> OpSizeT {
        let addr = addr as usize;
        let offset = addr - self.start as usize;

        info!("read {:x} offset {:x}", addr, offset);
        if offset >= self.bytes.len() {
            warn!("out of bounds");
            return 0;
        }

        let b0 = self.bytes[offset] as OpSizeT;
        match sz {
            OpSize::Byte => b0 as OpSizeT,
            OpSize::Word => {
                if offset + 1 >= self.bytes.len() {
                    warn!("out of bounds");
                    return 0;
                }

                let b1 = self.bytes[offset + 1] as OpSizeT;
                let w = (b1 << 8) | b0;

                trace!("got {:04x}", w);

                w
            }
        }
    }

    fn write(&mut self, addr: MemAddrT, data: OpSizeT, sz: OpSize) {
        let addr = addr as usize;
        let offset = addr - self.start as usize;

        info!("write {:x} offset {:x} size {:?}", addr, offset, sz);
        if offset >= self.bytes.len() {
            warn!("out of bounds");
            return;
        }

        match sz {
            OpSize::Byte => self.bytes[offset] = data as u8,
            OpSize::Word => {
                if offset + 1 >= self.bytes.len() {
                    warn!("out of bounds");
                    return;
                }

                self.bytes[offset] = (data & 0xff) as u8;
                self.bytes[offset + 1] = ((data >> 8) & 0xff) as u8;
            }
        }
    }
}
