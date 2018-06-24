use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system porcupine
    // shared library.
    println!("cargo:rustc-link-search=./lib/");
    println!("cargo:rustc-link-lib=pv_porcupine");
}
