use metal_std::{gpio, time::delay};


pub fn main() {
    loop {
        gpio::set(0xA);

        delay(2_000_000);
    }
}