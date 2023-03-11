#![no_std]
#![no_main]

mod leds;
mod utils;

extern crate panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut state = 0x40;
    loop {
        leds::set_leds(state);

        state >>= 1;
        if state == 0 {
            state = 0x40;
        }

        utils::delay(300000);
    }
}