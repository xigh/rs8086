use std::collections::HashMap;

use super::{IoAddrT, OpSizeT};

#[derive(Debug, Clone, Copy)]
pub enum IoSize {
    Byte,
    Word,
}

pub trait IOOps {
    fn read(&self, addr: IoAddrT, sz: IoSize) -> u16;
    fn write(&mut self, addr: IoAddrT, data: u16, sz: IoSize);
}

pub struct IOMap {
    map: HashMap<u16, Box<dyn IOOps>>,
}

impl IOMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, addr: IoAddrT, dev: Box<dyn IOOps>) {
        self.map.insert(addr, dev);
    }

    pub fn read(&self, addr: IoAddrT, sz: IoSize) -> Option<OpSizeT> {
        let dev = self.map.get(&addr)?;
        Some(dev.read(addr, sz))
    }

    pub fn write(&mut self, addr: IoAddrT, data: OpSizeT, sz: IoSize) -> bool {
        let Some(dev) = self.map.get_mut(&addr) else {
            return false;
        };
        dev.write(addr, data, sz);
        true
    }
}
