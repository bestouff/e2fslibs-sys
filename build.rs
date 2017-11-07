extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let lib = "ext2fs";
    let library = pkg_config::probe_library(lib).unwrap();

    println!("cargo:version={}", library.version);
    let includedir = pkg_config::get_variable(lib, "includedir").unwrap();
    println!("cargo:includedir={}", includedir);
    let (clang_version_major, clang_version_minor) = bindgen::clang_version().parsed.unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", includedir))
        .clang_arg(format!(
            "-I/usr/lib/clang/{}.{}/include",
            clang_version_major,
            clang_version_minor
        ))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
