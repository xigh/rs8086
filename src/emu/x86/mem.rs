use lib8086::Op;
use std::collections::HashMap;

use super::{MemAddrT, OpSize, OpSizeT};

pub trait MemOps {
    fn name(&self) -> String;
    fn read(&self, addr: MemAddrT, sz: OpSize) -> OpSizeT;
    fn write(&mut self, addr: MemAddrT, data: OpSizeT, sz: OpSize);
    // todo: atomic operations
}

// (start, end, priority) -> 8086 supports 1MB memory -> 32-bit address
type MemMapKey = (MemAddrT, MemAddrT, usize);

pub struct MemMap {
    map: HashMap<MemMapKey, Box<dyn MemOps>>,
}

impl MemMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, start: MemAddrT, end: MemAddrT, dev: Box<dyn MemOps>) {
        let priority = self.map.len();
        self.map.insert((start, end, priority), dev);
    }

    pub fn read(&self, addr: MemAddrT, sz: OpSize) -> Option<OpSizeT> {
        let key = self
            .map
            .keys()
            .filter(|&&(start, end, _)| start <= addr && addr < end)
            .max_by_key(|&&(start, end, priority)| priority)?;
        self.map.get(key).map(|dev| dev.read(addr, sz))
    }

    pub fn write(&mut self, addr: MemAddrT, data: OpSizeT, sz: OpSize) -> Option<()> {
        let key = self.map
            .keys()
            .filter(|&&(start, end, _)| start <= addr && addr < end)
            .max_by_key(|&&(start, end, priority)| priority)
            .cloned()?; // required to protect from immutable borrow (yes, rust can be hard and verbose)
        self.map.get_mut(&key).map(|dev| dev.write(addr, data, sz))
    }
}
