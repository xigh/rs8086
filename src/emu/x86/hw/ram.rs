use super::{Result, Device, MemMap, IOMap, MemAddrT, OpSizeT, MemSize, MemOps, Config};

pub struct DeviceRAM {
    bytes: Vec<u8>,
}

impl DeviceRAM {
    pub fn register(_cfg: &Config, vm: &mut MemMap, io: &mut IOMap) -> Result<()> {
        let dev = Self{
            bytes: vec![0; 0x100000],
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

    fn read(&self, addr: MemAddrT, sz: MemSize) -> OpSizeT {
        let addr = addr as usize;
        match sz {
            MemSize::Byte => self.bytes[addr] as OpSizeT,
            MemSize::Word => {
                let b1 = self.bytes[addr] as u16;
                let b2 = self.bytes[addr + 1] as u16;
                (b1 << 8) | b2
            }
        }
    }

    fn write(&mut self, addr: MemAddrT, data: OpSizeT, sz: MemSize) {
        let addr = addr as usize;
        match sz {
            MemSize::Byte => self.bytes[addr] = data as u8,
            MemSize::Word => {
                self.bytes[addr] = (data & 0xff) as u8;
                self.bytes[addr + 1] = ((data >> 8) & 0xff) as u8;
            }
        }
    }
}
