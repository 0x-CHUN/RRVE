use crate::exception::Exception;
use crate::exception::Exception::{LoadAccessFault, StoreAMOAccessFault};
use crate::param::*;

pub struct CLINT {
    mtime: u64,
    mtimecmp: u64,
}

impl CLINT {
    pub fn new() -> Self {
        Self { mtime: 0, mtimecmp: 0 }
    }

    pub fn load(&self, addr: u64, size: u64) -> Result<u64, Exception> {
        if size != 64 {
            return Err(LoadAccessFault(addr));
        }
        match addr {
            CLINT_MTIMECMP => Ok(self.mtimecmp),
            CLINT_MTIME => Ok(self.mtime),
            _ => Err(LoadAccessFault(addr)),
        }
    }

    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), Exception> {
        if size != 64 {
            return Err(LoadAccessFault(addr));
        }
        match addr {
            CLINT_MTIMECMP => Ok(self.mtimecmp = value),
            CLINT_MTIME => Ok(self.mtime = value),
            _ => Err(StoreAMOAccessFault(addr)),
        }
    }
}