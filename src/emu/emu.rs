use std::path::PathBuf;

use super::{Result, Cpu, Sreg, Reg16, Config, OpSizeT};

pub fn emulate(file: &str) -> Result<()> {
    let cfg = Config {
        bios_file: PathBuf::from(file),
        ram_size: 0xf0000,
        bios_addr: 0xf0000,
    };

    let mut cpu = Cpu::new(&cfg)?;

    // initialize registers
    cpu.write_sreg(Sreg::CS, ((cfg.bios_addr & 0xffff_0000) >> 4) as OpSizeT); // todo
    cpu.write_ip((cfg.bios_addr & 0x0000_ffff) as OpSizeT); // todo

    cpu.write_flags(0);

    cpu.write_sreg(Sreg::DS, 0x0000); // todo
    cpu.write_sreg(Sreg::ES, 0x0000); // todo
    cpu.write_sreg(Sreg::SS, 0x0000); // todo

    cpu.write_reg16(Reg16::AX, 0);
    cpu.write_reg16(Reg16::BX, 0x0810);
    cpu.write_reg16(Reg16::CX, 0x1122);
    cpu.write_reg16(Reg16::DX, 0xaaaa);
    cpu.write_reg16(Reg16::SI, 0);
    cpu.write_reg16(Reg16::DI, 0);
    cpu.write_reg16(Reg16::BP, 0);
    cpu.write_reg16(Reg16::SP, 0xfffe as OpSizeT);

    while !cpu.is_halted() {
        cpu.dump_regs();
        cpu.tick();

        println!("press return to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }

    Ok(())
}
