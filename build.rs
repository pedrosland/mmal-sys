extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system shared libraries.
    println!("cargo:rustc-link-lib=mmal_core");
    println!("cargo:rustc-link-lib=mmal_util");
    println!("cargo:rustc-link-lib=mmal_vc_client");
    println!("cargo:rustc-link-lib=vcos");
    println!("cargo:rustc-link-lib=bcm_host");
    println!("cargo:rustc-link-search=native=/opt/vc/lib");
//    println!("cargo:include=/opt/vc/include/interface");
//    println!("cargo:libdir=/opt/vc/lib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
       .constified_enum_module(r"MMAL_STATUS_T|MMAL_PARAMETER_CAMERA_CONFIG_TIMESTAMP_MODE_T")

       .derive_debug(true)
       .impl_debug(true)
        // Without this, we get `__BindgenUnionField` in
        // places and it isn't very pretty.
       .rust_target(bindgen::RustTarget::Nightly)
//      // Fix library path to include mmal headers
        .clang_arg("-I/opt/vc/include")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
