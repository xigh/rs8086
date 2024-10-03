use std::path::PathBuf;

use tracing::{debug, trace};

use super::{
    inst_to_string, Config, Cpu, Flags, Inst, Op, OpSize, OpSizeT, Reg16, Reg8, Result, Sreg,
};

#[derive(Default)]
pub struct EmuOpts {
    pub test_mode: bool,
    pub wait_for_enter: bool,
    pub dump_regs_each_step: bool,
    pub dump_regs_on_halt: bool,
}

pub fn emulate(file: &str, opts: &EmuOpts) -> Result<()> {
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

    let mut prev_op = Inst::default();
    let mut prev_ip: u32 = 0;

    loop {
        let mut found_hlt_at = None;

        if opts.test_mode && cpu.is_halted() {
            let ip = cpu.read_ip();
            let cs = Sreg::CS;
            let ea = cpu.calc_ea(cs, ip);
            found_hlt_at = Some(ea);
        } else if opts.test_mode && prev_op.op == Op::Ret {
            let hlt_ea = prev_ip + prev_op.size as u32;
            // if ret followed by a halt we may have debug infos
            if let Some(b0) = cpu.read_mem_ea(hlt_ea, OpSize::Byte) {
                if b0 == 0xf4 {
                    found_hlt_at = Some(hlt_ea);
                }
            }
        }

        if let Some(hlt_ea) = found_hlt_at {
            trace!("debug: cpu is halted, executing tests");

            let word_to_string = |w: u16| {
                let second_char = (w >> 8 & 0xFF) as u8;
                let first_char = (w & 0xFF) as u8;
                format!("{}{}", first_char as char, second_char as char)
            };

            let mut file = String::new();
            let mut ea = hlt_ea + 1;

            debug!("after hlt ea: {:04x}", ea);

            'debug_loop: while let Some(line) = cpu.read_mem_ea(ea, OpSize::Word) {
                ea += 2;

                trace!("debug: line: {:04X} at 0x{:05X}", line, ea);

                let Some(w) = cpu.read_mem_ea(ea, OpSize::Word) else {
                    debug!("expect-data ^^ found, but failed to read cmd word");
                    return Ok(());
                };

                let name = word_to_string(w);
                match name.as_str() {
                    "--" => {
                        break 'debug_loop
                    }
                    "^^" => {
                        let debug_ea = ea;
                        ea += 2;
                        loop {
                            let Some(b) = cpu.read_mem_ea(ea, OpSize::Byte) else {
                                debug!("found '^^' at 0x{:05X}, but failed to read byte", debug_ea);
                                return Ok(());
                            };
                            ea += 1;
                            if b == 0 {
                                break;
                            }
                            let b = b as u8;
                            file.push(b as char);
                        }
                        trace!("debug: file: {:?}", file);
                        continue;
                    }
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
                            return Err(format!(
                                "{}:{}: {}: got 0x{:04X}, expected 0x{:04X}",
                                file, line, name, reg, ex
                            )
                            .into());
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
                            return Err(format!(
                                "{}:{}: {}: got 0x{:02X}, expected 0x{:02X}",
                                file, line, name, reg, ex
                            )
                            .into());
                        }
                        trace!("{}: {:02X} [OK]", name, reg);
                        ea += 3;
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
                            return Err(format!(
                                "{}:{}: {}: got 0x{:04X}, expected 0x{:04X}",
                                file, line, name, reg, ex
                            )
                            .into());
                        }
                        trace!("{}: {:04X} [OK]", name, reg);
                        ea += 4;
                    }
                    "CF" | "PF" | "AF" | "ZF" | "SF" | "TF" | "IF" | "DF" | "OF" => {
                        let f = cpu.is_flag_set(match name.as_str() {
                            "CF" => Flags::C,
                            "PF" => Flags::P,
                            "AF" => Flags::A,
                            "ZF" => Flags::Z,
                            "SF" => Flags::S,
                            "TF" => Flags::T,
                            "IF" => Flags::I,
                            "DF" => Flags::D,
                            "OF" => Flags::O,
                            _ => unreachable!(),
                        });
                        let ex = cpu.read_mem_ea(ea + 2, OpSize::Byte).unwrap();
                        if (ex != 0) != f {
                            return Err(format!(
                                "{}:{}: {}: got {}, expected {}",
                                file,
                                line,
                                name,
                                f,
                                ex != 0
                            )
                            .into());
                        }
                        trace!("{}: {} [OK]", name, f);
                        ea += 3;
                    }
                    _ => {
                        if file == "" {
                            break 'debug_loop;
                        }
                        return Err(format!(
                            "{}:{}: unknown expected debug value: {}",
                            file,
                            line,
                            word_to_string(w)
                        )
                        .into());
                    }
                }
            }
            if opts.dump_regs_on_halt {
                cpu.dump_regs();
            }
            println!("{}: tests successfull", file);
            return Ok(());
        }

        if cpu.is_halted() {
            if opts.dump_regs_on_halt {
                cpu.dump_regs();
            }
            break;
        }

        if opts.dump_regs_each_step {
            cpu.dump_regs();
        }

        let (inst, pc, bytes) = cpu.next_inst();
        let bytes = bytes
            .iter()
            .map(|b| format!("{:02x}", *b))
            .collect::<Vec<String>>();

        println!(
            "{:06X} {:16} {}",
            pc,
            bytes.join(" "),
            inst_to_string(pc, &inst)
        );

        prev_ip = pc;
        prev_op = inst;

        cpu.tick();

        if opts.wait_for_enter {
            println!("press return to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }

    Ok(())
}
