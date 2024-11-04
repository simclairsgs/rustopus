use std::path::PathBuf;
use bindgen;

fn main() {

    println!("cargo:rustc-link-lib=opus");
    println!("cargo:rustc-link-lib=static=opus");
    println!("cargo:rustc-link-search=native=opus/.libs");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .allowlist_function("opus_.*")
        .allowlist_type("Opus.*")
        .allowlist_var("OPUS_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}