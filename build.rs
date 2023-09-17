// build.rs

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=lib/wrapper.h");

    // Compile blackmagic with the cc crate
    compile_with_cc();

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("lib/wrapper.h")
        // preprocessor definitions
        .clang_args(&[
            "-DPROBE_HOST=hosted",
            "-DHOSTED_BMP_ONLY=1",
            "-DENABLE_RTT=1",
            "-DPC_HOSTED=1",
        ])
        // include directories
        .clang_args(&[
            "-Iblackmagic/src/target",
            "-Iblackmagic/src/include",
            "-Iblackmagic/src/platforms/hosted",
        ])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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

fn compile_with_cc() {
    let binding = cc::Build::new();
    let mut build = binding;

    build
        .files([
            "blackmagic/src/command.c",
            "blackmagic/src/remote.c",
            "blackmagic/src/hex_utils.c",
            "blackmagic/src/exception.c",
            "blackmagic/src/platforms/hosted/bmp_remote.c",
            "blackmagic/src/platforms/hosted/debug.c",
            "blackmagic/src/platforms/hosted/platform.c",
            "blackmagic/src/platforms/hosted/utils.c",
            // Protocols
            "blackmagic/src/platforms/hosted/remote/protocol_v0_adiv5.c",
            "blackmagic/src/platforms/hosted/remote/protocol_v0_jtag.c",
            "blackmagic/src/platforms/hosted/remote/protocol_v0_swd.c",
            "blackmagic/src/platforms/hosted/remote/protocol_v0.c",
            "blackmagic/src/platforms/hosted/remote/protocol_v1_adiv5.c",
            "blackmagic/src/platforms/hosted/remote/protocol_v1.c",
            "blackmagic/src/platforms/hosted/remote/protocol_v2.c",
            "blackmagic/src/platforms/hosted/remote/protocol_v3_adiv5.c",
            "blackmagic/src/platforms/hosted/remote/protocol_v3.c",
        ])
        .includes([
            "lib/include",
            "blackmagic/src",
            "blackmagic/src/target",
            "blackmagic/src/include",
            "blackmagic/src/platforms/hosted",
        ])
        .define("PROBE_HOST", "hosted")
        .define("HOSTED_BMP_ONLY", "1")
        .define("ENABLE_RTT", "1")
        .define("PC_HOSTED", "1");

    // Conditionally link these files
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        build.file("blackmagic/src/platforms/hosted/serial_unix.c");
    }
    if cfg!(target_os = "windows") {
        build.file("blackmagic/src/platforms/hosted/serial_win.c");
    }

    // Compile into lib and link
    build.compile("blackmagic-lib");
    println!("cargo:rustc-link-lib=blackmagic-lib");
}
