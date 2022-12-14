use crate::clint::CLINT;
use crate::dram::Dram;
use crate::exception::Exception;
use crate::param::{CLINT_BASE, CLINT_END, DRAM_BASE, DRAM_END, PLIC_BASE, PLIC_END};
use crate::plic::PLIC;

pub struct Bus {
    dram: Dram,
    plic: PLIC,
    clint: CLINT,
}

impl Bus {
    pub fn new(code: Vec<u8>) -> Bus {
        Self {
            dram: Dram::new(code),
            plic: PLIC::new(),
            clint: CLINT::new(),
        }
    }

    pub fn load(&mut self, addr: u64, size: u64) -> Result<u64, Exception> {
        match addr {
            CLINT_BASE..=CLINT_END => self.clint.load(addr, size),
            PLIC_BASE..=PLIC_END => self.plic.load(addr, size),
            DRAM_BASE..=DRAM_END => self.dram.load(addr, size),
            _ => Err(Exception::LoadAccessFault(addr))
        }
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        match addr {
            CLINT_BASE..=CLINT_END => self.clint.store(addr, size, value),
            PLIC_BASE..=PLIC_END => self.plic.store(addr, size, value),
            DRAM_BASE..=DRAM_END => self.dram.store(addr, size, value),
            _ => Err(Exception::StoreAMOAccessFault(addr)),
        }
    }
}