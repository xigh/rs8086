use super::{Result, IOMap, MemAddrT, MemMap, MemOps, MemSize, OpSizeT, Config, dump};

mod ram;
use ram::DeviceRAM;

mod rom;
use rom::DeviceROM;

pub trait Device {
    fn name(&self) -> String;
}

pub fn init_devices(cfg: &Config, vm: &mut MemMap, io: &mut IOMap) -> Result<()> {
    DeviceRAM::register(cfg, vm, io)?; // todo: RAM can be loaded from file (snapshot)
    DeviceROM::register(cfg, vm, io)?; // todo: ROM can be loaded from file
    Ok(())
}
