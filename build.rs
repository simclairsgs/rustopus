use std::env;
use std::path::PathBuf;
use bindgen;

fn main() {
    // Specify the path to Opus headers (adjust as necessary)
    let opus_include_path = "/Users/george-17657/Documents/Codebase/rustopus/opus/include"; // Change this if Opus is installed elsewhere

    // Instruct Cargo to link with Opus library
    println!("cargo:rustc-link-lib=opus");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h") // Adjust for specific path if necessary - format!("{}/opus.h", opus_include_path)
        .allowlist_function("opus_.*") // Only generate bindings for functions that start with "opus_"
        .allowlist_type("Opus.*")      // Allowlist types that start with "Opus"
        .allowlist_var("OPUS_.*")      // Allowlist Opus constants/macros
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to $OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}