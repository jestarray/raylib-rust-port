use std::env;
use std::path::PathBuf;

fn main() {
    let raylib_src = "/mnt/z/raylib/src";
    let external_dir = format!("{}/external", raylib_src);

    // Compile the wrapper containing stb_* and other C libraries
    cc::Build::new()
        .file("src/c_external/wrapper.c")
        .include(&external_dir)
        // Ensure STB libraries are compiled correctly for our use cases
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-missing-field-initializers")
        .compile("external");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("src/c_external/wrapper.c")
        .clang_arg(format!("-I{}", external_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        // Only generate bindings for what we actually included
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("external_bindings.rs"))
        .expect("Couldn't write bindings!");
}
