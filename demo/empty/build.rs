use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use metal_build::{self, MetalConfig};

fn linker_memory_layout(out_dir: &str) {
    let destination = Path::new(&out_dir);
    let mut f = File::create(&destination.join("memory.x")).expect("can't create memory.x");

    f.write_all(include_bytes!("memory.x"))
        .expect("can't write memory.x");

    println!("cargo:rustc-link-search={}", destination.display());
}

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    
    let src_dir = "./src/";
    let metal_out = "../../core/cpu/hardware/metal/generated/Controller.scala".to_owned();

    metal_build::process_teleports(&src_dir, MetalConfig {
        output: metal_out,
    }).unwrap();

    linker_memory_layout(&out_dir);
}