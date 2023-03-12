pub struct I2CTransaction {
    addr: u32,
}

impl I2CTransaction {
    fn new(addr: u32) -> Self {
        Self {
            addr,
        }
    }

    pub fn write(&self, data: u8) {
        unsafe {
            *(0x10010000 as *mut u32) = self.addr;
            *(0x10010004 as *mut u8) = data;
        }
    }

    pub fn read(&self) -> u8 {
        unsafe {
            *(0x10010000 as *mut u32) = self.addr;
            *(0x10010008 as *mut u8)
        }
    }

    pub fn write_bytes(&self, data: &[u8]) {
        for byte in data {
            self.write(*byte);
        }
    }

    pub fn end(&self) {
        unsafe {
            *(0x10010000 as *mut u32) = self.addr;
            *(0x1001000C as *mut u32) = 0;
        }
    }
}

pub fn begin(addr: u32) -> I2CTransaction {
    I2CTransaction::new(addr)
}