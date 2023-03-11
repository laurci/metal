use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    let destination = Path::new(&out_dir);
    let mut f = File::create(&destination.join("memory.x")).expect("can't create memory.x");

    f.write_all(include_bytes!("memory.x"))
        .expect("can't write memory.x");

    println!("cargo:rustc-link-search={}", destination.display());

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}