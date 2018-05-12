extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(feature = "static") {
        println!("cargo:rustc-link-lib=gpac_static");
    } else {
        println!("cargo:rustc-link-lib=gpac");
    }

    let mut builder = bindgen::Builder::default().header("wrapper.h");
    for blacklist_type in &[
        "u64",
        "u32",
        "u16",
        "u8",
        "FP_NORMAL",
        "FP_SUBNORMAL",
        "FP_ZERO",
        "FP_INFINITE",
        "FP_NAN",
    ] {
        builder = builder.blacklist_type(blacklist_type);
    }
    let bindings = builder.generate().expect("generate gpac bindings");
    let out_path =
        PathBuf::from(env::var("OUT_DIR").expect("environment variable `OUT_DIR' exists"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("write gpac bindings file");
}
