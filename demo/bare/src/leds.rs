pub fn set_leds(mask: u32) {
    unsafe {
        *(0x10000000 as *mut u32) = mask;
    }
}