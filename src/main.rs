use std::{env, io};
use std::fs::File;
use std::io::Read;

struct CPU {
    // RISC-V 32 registers
    regs: [u64; 32],
    // pc register
    pc: u64,
    // memory
    dram: Vec<u8>,
}

pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;

const RVABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
    "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
    "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
];

impl CPU {
    fn new(code: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        regs[2] = DRAM_SIZE - 1;
        Self {
            regs,
            pc: 0,
            dram: code,
        }
    }
    // fetch 32 bits instruction
    fn fetch(&self) -> u32 {
        let index = self.pc as usize;

        let instr = self.dram[index] as u32
            | ((self.dram[index + 1] as u32) << 8)
            | ((self.dram[index + 2] as u32) << 16)
            | ((self.dram[index + 3] as u32) << 24);
        instr
    }

    fn execute(&mut self, instr: u32) {
        // decode
        let opcode = instr & 0b111_1111;
        let rd = ((instr >> 7) & 0b1_1111) as usize;
        let rs1 = ((instr >> 15) & 0b1_1111) as usize;
        let rs2 = ((instr >> 20) & 0b1_1111) as usize;
        let funct3 = (instr >> 12) & 0b111;
        let funct7 = (instr >> 25) & 0b111_1111;
        // x0 is hardwired zero
        self.regs[0] = 0;
        match opcode {
            0x13 => {
                // addi
                let imm = ((instr & 0xfff0_0000) as i64 >> 20) as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm);
            }
            0x33 => {
                // add
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }

            _ => {
                dbg!(format!("Invalid opcode: {:#x}", opcode));
            }
        }
    }

    pub fn dump_register(&mut self) {
        println!("{:-^80}", "registers");
        let mut output = String::new();
        self.regs[0] = 0;
        for i in (0..32).step_by(4) {
            let i0 = format!("x{}", i);
            let i1 = format!("x{}", i + 1);
            let i2 = format!("x{}", i + 2);
            let i3 = format!("x{}", i + 3);
            let line = format!(
                "{:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x}\n",
                i0, RVABI[i], self.regs[i],
                i1, RVABI[i + 1], self.regs[i + 1],
                i2, RVABI[i + 2], self.regs[i + 2],
                i3, RVABI[i + 3], self.regs[i + 3],
            );
            output = output + &line;
        }
        println!("{}", output);
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!(
            "Usage:\n\
            - cargo run <filename>"
        );
        return Ok(());
    }

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;
    let mut cpu = CPU::new(code);
    while cpu.pc < cpu.dram.len() as u64 {
        let inst = cpu.fetch();
        cpu.execute(inst);
        cpu.pc += 4;
    }
    cpu.dump_register();
    Ok(())
}
