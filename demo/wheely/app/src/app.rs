use metal_std::{gpio, time::{delay}};


pub fn main() {
    
    loop {
        gpio::set(0x9);

        delay(2_000_000);
    }
}