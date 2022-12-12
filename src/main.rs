mod bus;
mod dram;
mod exception;
mod param;
mod cpu;
mod csr;

use std::{env, io};
use std::fs::File;
use std::io::Read;
use crate::cpu::CPU;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if (args.len() != 2) && (args.len() != 3) {
        panic!("Usage: rvemu-for-book <filename> <(option) image>");
    }
    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;
    let mut cpu = CPU::new(binary);
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
    }
    cpu.dump_registers();
    cpu.dump_csrs();
    cpu.dump_pc();

    Ok(())
}
