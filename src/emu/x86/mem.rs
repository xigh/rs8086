use std::collections::HashMap;
use lib8086::Op;

use super::{MemAddrT, OpSizeT, OpSize};

pub trait MemOps {
    fn name(&self) -> String;
    fn read(&self, addr: MemAddrT, sz: OpSize) -> OpSizeT;
    fn write(&mut self, addr: MemAddrT, data: OpSizeT, sz: OpSize);
    // todo: atomic operations
}

pub struct MemMap {
    // (start, end) -> 8086 supports 1MB memory -> 32-bit address
    map: HashMap<(MemAddrT, MemAddrT), Box<dyn MemOps>>,
}

impl MemMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, start: MemAddrT, end: MemAddrT, dev: Box<dyn MemOps>) {
        self.map.insert((start, end), dev);
    }

    pub fn read(&self, addr: MemAddrT, sz: OpSize) -> Option<OpSizeT> {
        for (range, dev) in self.map.iter() {
            if addr >= range.0 && addr <= range.1 {
                return Some(dev.read(addr, sz));
            }
        }
        None
    }
    
    pub fn write(&mut self, addr: MemAddrT, data: OpSizeT, sz: OpSize) -> bool {
        for (range, dev) in self.map.iter_mut() {
            if addr >= range.0 && addr <= range.1 {
                dev.write(addr, data, sz);
                return true;
            }
        }
        false
    }
}
