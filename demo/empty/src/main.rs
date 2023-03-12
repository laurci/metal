#![no_std]
#![no_main]
extern crate panic_halt;

mod app;

use riscv_rt::entry;

#[entry]
fn main() -> ! {    
    metal_std::time::delay(5_000_000);

    app::main();

    loop {}
}