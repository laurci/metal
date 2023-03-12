use metal_std::{gpio, time::{delay}};

#[metal::teleport]
fn mul(a: u16, b: u16) -> u32 {
    a * b
}

pub fn main() {
    let mut state: u8 = 0;
    
    loop {
        if state == 0 {
            gpio::set(mul(5, 2));
            state = 1;
        } else {
            gpio::set(mul(3, 2));
            state = 0;
        }

        delay(2_000_000);
    }
}