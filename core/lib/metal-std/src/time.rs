use core::arch::asm;

pub fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            asm!("nop");
        }
    }
}