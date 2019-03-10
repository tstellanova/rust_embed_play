extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::{ RustTarget};


//TODO WIP -- not yet working, using bindgen cli instead

fn main() {

    let src_dir = env::current_dir().unwrap();
    println!("src_dir: {:?}", src_dir);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("out_path: {:?}", out_path);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let _builder = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        .rust_target(RustTarget::Stable_1_33)
        // The input header we would like to generate
        // bindings for.
        .header("inc/export_rusts.h")
        // no layout tests required
        .layout_tests(false)
        // use core instead of std
        .use_core()
        // use cty types instead of pulling in std types
        .ctypes_prefix("cty");
    println!("init builder");

    // Finish the builder and generate the bindings.
//    let bindings = builder.generate()
//        // Unwrap the Result and panic on failure.
//        .expect("Unable to generate bindings");
//    println!("generated bindings");



    // Write the bindings to the $OUT_DIR/bindings.rs file.
//    bindings
//        .write_to_file(out_path.join("uorb_bindings.rs"))
//        .expect("Couldn't write bindings!");

}