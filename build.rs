use std::env;
use std::path::PathBuf;

fn main() {
    let external_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("src/c_external");
    println!("cargo:rerun-if-changed=src/c_external");

    // Compile the wrapper containing stb_* and other C libraries
    cc::Build::new()
        .file("src/c_external/wrapper.c")
        .include(&external_dir)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-function")
        .flag_if_supported("-Wno-missing-field-initializers")
        .compile("external");

    // Build bindgen
    // bindgen reads BINDGEN_EXTRA_CLANG_ARGS (including the target-specific
    // form) itself. cargo-ndk supplies the NDK sysroot.
    let mut builder = bindgen::Builder::default()
        .header("src/c_external/wrapper.c")
        .clang_arg(format!("-I{}", external_dir.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("android") {
        // Recent NDK headers require an API-versioned clang target. Rust's
        // TARGET has no API suffix, and cargo-ndk 4 supplies only the sysroot
        // in its bindgen flags.
        let api_vars = [
            "ANDROID_PLATFORM",
            "ANDROID_API_LEVEL",
            "CARGO_NDK_ANDROID_PLATFORM",
        ];
        for name in api_vars {
            println!("cargo:rerun-if-env-changed={name}");
        }
        // cargo-ndk 4.1.2 sets CARGO_NDK_ANDROID_PLATFORM to the ABI name,
        // despite its documented meaning. ANDROID_PLATFORM contains the API.
        let api = api_vars
            .into_iter()
            .filter_map(|name| env::var(name).ok())
            .find_map(|value| value.trim_start_matches("android-").parse::<u32>().ok())
            .unwrap_or(25);
        let target = env::var("TARGET").unwrap();
        let clang_target = if target == "armv7-linux-androideabi" {
            "armv7a-linux-androideabi"
        } else {
            &target
        };
        builder = builder.clang_arg(format!("--target={clang_target}{api}"));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("external_bindings.rs"))
        .expect("Couldn't write bindings!");
}
