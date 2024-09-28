use std::path::PathBuf;

use tracing::{debug, error, trace};

use super::{Config, Cpu, OpSizeT, Reg16, Reg8, Sreg, Result, OpSize};

pub struct EmuOpts {
    pub test_mode: bool,
    pub wait_for_enter: bool,
}

pub fn emulate(file: &str, opts: EmuOpts) -> Result<()> {
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
    cpu.write_reg16(Reg16::BX, 0);
    cpu.write_reg16(Reg16::CX, 0);
    cpu.write_reg16(Reg16::DX, 0);
    cpu.write_reg16(Reg16::SI, 0);
    cpu.write_reg16(Reg16::DI, 0);
    cpu.write_reg16(Reg16::BP, 0);
    cpu.write_reg16(Reg16::SP, 0);

    loop {
        if opts.test_mode && cpu.is_halted() {
            // todo: check "expect" pseudo-instruction after ip (if any)

            let word_to_string = |w: u16| {
                let second_char = (w >> 8 & 0xFF) as u8;
                let first_char = (w & 0xFF) as u8;
                format!("{}{}", first_char as char, second_char as char)
            };

            let ip = cpu.read_ip();
            let cs = Sreg::CS;
            let mut ea = cpu.calc_ea(cs, ip);

            while let Some(w) = cpu.read_mem_ea(ea, OpSize::Word) {
                let name = word_to_string(w);
                match name.as_str() {
                    "AX" | "BX" | "CX" | "DX" | "SI" | "DI" | "BP" | "SP" => {
                        let reg = cpu.read_reg16(match name.as_str() {
                            "AX" => Reg16::AX,
                            "BX" => Reg16::BX,
                            "CX" => Reg16::CX,
                            "DX" => Reg16::DX,
                            "SI" => Reg16::SI,
                            "DI" => Reg16::DI,
                            "BP" => Reg16::BP,
                            "SP" => Reg16::SP,
                            _ => unreachable!(),
                        });
                        let ex = cpu.read_mem_ea(ea + 2, OpSize::Word).unwrap();
                        if reg != ex {
                            error!("{}: got 0x{:04X}, expected 0x{:04X}", name, reg, ex);
                            break;
                        }
                        trace!("{}: {:04X} [OK]", name, reg);
                        ea += 4;
                    }
                    "AL" | "BL" | "CL" | "DL" | "AH" | "BH" | "CH" | "DH" => {
                        let reg = cpu.read_reg8(match name.as_str() {
                            "AL" => Reg8::AL,
                            "BL" => Reg8::BL,
                            "CL" => Reg8::CL,
                            "DL" => Reg8::DL,
                            "AH" => Reg8::AH,
                            "BH" => Reg8::BH,
                            "CH" => Reg8::CH,
                            "DH" => Reg8::DH,
                            _ => unreachable!(),
                        });
                        let ex = cpu.read_mem_ea(ea + 2, OpSize::Byte).unwrap() as u8;
                        if reg != ex {
                            error!("{}: got 0x{:02X}, expected 0x{:02X}", name, reg, ex);
                            break;
                        }
                        trace!("{}: {:02X} [OK]", name, reg);
                        ea += 2;
                    }
                    "CS" | "ES" | "DS" | "SS" => {
                        let reg = cpu.read_sreg(match name.as_str() {
                            "CS" => Sreg::CS,
                            "ES" => Sreg::ES,
                            "DS" => Sreg::DS,
                            "SS" => Sreg::SS,
                            _ => unreachable!(),
                        });
                        let ex = cpu.read_mem_ea(ea + 2, OpSize::Word).unwrap();
                        if reg != ex {
                            error!("{}: got 0x{:04X}, expected 0x{:04X}", name, reg, ex);
                            break;
                        }
                        trace!("{}: {:04X} [OK]", name, reg);
                        ea += 4;
                    }
                    "CF" | "PF" | "AF" | "ZF" | "SF" | "TF" | "IF" | "DF" | "OF" => {
                        todo!("flag check");
                    }
                    _ => {
                        debug!("unknown expected debug value: {}", word_to_string(w));
                        break;
                    }
                }
            }
            return Ok(());
        }

        if cpu.is_halted() {
            println!("CPU halted: not handled yet");
            break;
        }

        cpu.dump_regs();
        cpu.tick();

        if opts.wait_for_enter {    
            println!("press return to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }

    Ok(())
}
