use std::env;
use std::path::PathBuf;
use bindgen;

fn main() {
    // Specify the path to Opus headers (adjust as necessary)
    let opus_include_path = "opus/include"; // Change this if Opus is installed elsewhere

    // Instruct Cargo to link with Opus library
    println!("cargo:rustc-link-lib=opus");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .allowlist_function("opus_.*")
        .allowlist_type("Opus.*")
        .allowlist_var("OPUS_.*")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to $OUT_DIR/bindings.rs
    let out_path = PathBuf::from("src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}