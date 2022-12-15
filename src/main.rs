mod bus;
mod dram;
mod exception;
mod param;
mod cpu;
mod csr;
mod plic;
mod clint;
mod uart;
mod interrupt;
mod virtio;

use std::{env, io};
use std::fs::File;
use std::io::Read;
use crate::cpu::CPU;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if (args.len() != 2) && (args.len() != 3) {
        panic!("Usage: R-RISCV <filename> <(option) image>");
    }
    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let mut disk_image = Vec::new();
    if args.len() == 3 {
        let mut file = File::open(&args[2])?;
        file.read_to_end(&mut disk_image)?;
    }

    let mut cpu = CPU::new(binary, disk_image);
    loop {
        let instr = match cpu.fetch() {
            Ok(instr) => instr,
            Err(e) => {
                cpu.handle_exception(e);
                if e.is_fatal() {
                    println!("{}", e);
                    break;
                }
                continue;
            }
        };
        match cpu.execute(instr) {
            // Break the loop if an error occurs.
            Ok(new_pc) => cpu.pc = new_pc,
            Err(e) => {
                cpu.handle_exception(e);
                if e.is_fatal() {
                    println!("{}", e);
                    break;
                }
            }
        };
        match cpu.check_pending_interrupt() {
            Some(interrupt) => cpu.handle_interrupt(interrupt),
            None => ()
        }
    }
    cpu.dump_registers();
    cpu.dump_csrs();
    cpu.dump_pc();

    Ok(())
}
