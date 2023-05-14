#[cfg(feature = "generate_bindings")]
extern crate bindgen;

use std::env;
use std::path::Path;

#[cfg(feature = "generate_bindings")]
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system shared libraries.
    println!("cargo:rustc-link-lib=mmal_util");
    println!("cargo:rustc-link-lib=mmal_core");
    println!("cargo:rustc-link-lib=mmal_vc_client");
    println!("cargo:rustc-link-lib=vcos");
    println!("cargo:rustc-link-lib=bcm_host");
    println!("cargo:rustc-link-lib=vchiq_arm");
    println!("cargo:rustc-link-lib=vcsm");
    println!("cargo:rerun-if-env-changed=HOST");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=MMAL_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=MMAL_LIB_DIR");
    println!("cargo:rustc-link-search=native={}", locate_mmal_lib_dir());

    #[cfg(feature = "generate_bindings")]
    generate_bindings();
}

#[cfg(feature = "generate_bindings")]
fn generate_bindings() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    let mut mmal_lib_arg = "-I".to_owned();
    mmal_lib_arg.push_str(&locate_mmal_headers());

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
        .whitelist_var(r"MMAL_.*")

        .derive_debug(true)
        .impl_debug(true)
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h");

    // Include mmal library headers
    bindings = bindings.clang_arg(mmal_lib_arg);

    if target == "armv7-unknown-linux-gnueabihf" && host != target {
        // We're cross-compiling
        bindings = bindings
            .clang_arg("-I/usr/lib/gcc-cross/arm-linux-gnueabihf/4.8/include-fixed/")
            .clang_arg("-I/usr/lib/gcc-cross/arm-linux-gnueabihf/4.8/include/")
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


#[cfg(feature = "generate_bindings")]
fn locate_mmal_headers() -> String {
    let default_path = "/opt/vc/include";
    let path = if let Ok(env_path) = env::var("MMAL_INCLUDE_DIR") {
        env_path
    } else {
        default_path.to_owned()
    };

    if !Path::new(&path).exists() {
        panic!(format!("Could not locate mmal headers.
path: {}
default: {}
env MMAL_INCLUDE_DIR: {:?}
",
            path, default_path, env::var("MMAL_INCLUDE_DIR")
	));
    }

    path
}

fn locate_mmal_lib_dir() -> String {
    let default_path = "/opt/vc/lib";
    let path = if let Ok(env_path) = env::var("MMAL_LIB_DIR") {
        env_path
    } else {
        default_path.to_owned()
    };

    if !Path::new(&path).exists() {
        panic!("Could not locate libary.
path: {}
default: {}
env MMAL_LIB_DIR: {:?}
",
            path, default_path, env::var("MMAL_LIB_DIR")
        );
    }

    path
}
