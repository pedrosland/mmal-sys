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

    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut bindings = bindgen::Builder::default()
        // Pretty bindings make easier reading
        .rustfmt_bindings(true)

        .constified_enum_module(r"MMAL_STATUS_T|MMAL_PARAMETER_CAMERA_CONFIG_TIMESTAMP_MODE_T")

        // Prevent generating bindings including libc!
        .whitelist_type(r"MMAL_.*")
        .whitelist_function(r"(?:mmal_|vcos_|bcm_).*")

        .derive_debug(true)
        .impl_debug(true)
        // Without this, we get `__BindgenUnionField` in
        // places and it isn't very pretty.
        .rust_target(bindgen::RustTarget::Nightly)
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Fix library path to include mmal headers
        .clang_arg("-I/opt/vc/include");

    if target == "armv7-unknown-linux-gnueabihf" && host != target {
        // We're cross-compiling
        bindings = bindings
            .clang_arg("-I/usr/lib/gcc/arm-linux-gnueabihf/4.6/include-fixed/")
            .clang_arg("-I/usr/lib/gcc/arm-linux-gnueabihf/4.6/include/")
            .clang_arg("-I/usr/arm-linux-gnueabihf/include/")
            .clang_arg("-nobuiltininc")
            .clang_arg("-nostdinc++");
    }

    let bindings = bindings
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
