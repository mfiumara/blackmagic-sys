// build.rs

use std::env;
use std::path::PathBuf;

fn main() {
    compile_bmp();
    link_libraries();

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=lib/wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
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

fn link_libraries() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=blackmagic/src");

    // Tell cargo to link blackmagic
    println!("cargo:rustc-link-lib=bmp_remote.o");
    println!("cargo:rustc-link-lib=remote.o");
    println!("cargo:rustc-link-lib=debug.o");
    println!("cargo:rustc-link-lib=hex_utils.o");
    println!("cargo:rustc-link-lib=command.o");
    println!("cargo:rustc-link-lib=platform.o");
    println!("cargo:rustc-link-lib=exception.o");
    // println!("cargo:rustc-link-lib=bmp_serial.o");
    // println!("cargo:rustc-link-lib=cli.o");
    // println!("cargo:rustc-link-lib=adiv5.o");
    // println!("cargo:rustc-link-lib=adiv5_swd.o");
    // println!("cargo:rustc-link-lib=jtag_scan.o");
    // println!("cargo:rustc-link-lib=gdb_if.o");
    // println!("cargo:rustc-link-lib=gdb_packet.o");

    // println!("cargo:rustc-link-lib=rtt.o");

    // Targets
    // println!("cargo:rustc-link-lib=cortex.o");
    // println!("cargo:rustc-link-lib=cortexa.o");
    // println!("cargo:rustc-link-lib=cortexm.o");
    // println!("cargo:rustc-link-lib=stm32f1.o");
    // println!("cargo:rustc-link-lib=stm32f4.o");
    // println!("cargo:rustc-link-lib=stm32g0.o");
    // println!("cargo:rustc-link-lib=stm32h5.o");
    // println!("cargo:rustc-link-lib=stm32h7.o");
    // println!("cargo:rustc-link-lib=stm32l0.o");
    // println!("cargo:rustc-link-lib=stm32l4.o");
    // println!("cargo:rustc-link-lib=renesas.o");
    // println!("cargo:rustc-link-lib=target_flash.o");
    // println!("cargo:rustc-link-lib=target.o");
    // println!("cargo:rustc-link-lib=target_probe.o");

    // Protocols
    println!("cargo:rustc-link-lib=protocol_v0_adiv5.o");
    println!("cargo:rustc-link-lib=protocol_v0_jtag.o");
    println!("cargo:rustc-link-lib=protocol_v0_swd.o");
    println!("cargo:rustc-link-lib=protocol_v0.o");
    println!("cargo:rustc-link-lib=protocol_v1_adiv5.o");
    println!("cargo:rustc-link-lib=protocol_v1.o");
    println!("cargo:rustc-link-lib=protocol_v2.o");
    println!("cargo:rustc-link-lib=protocol_v3_adiv5.o");
    println!("cargo:rustc-link-lib=protocol_v3.o");

    // Conditionally link these files
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=serial_unix.o");
    }
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=serial_win.o");
    }
}

fn compile_bmp() {
    // Environment variables needed by blackmagic build
    // std::env::set_var("PROBE_HOST", "hosted");
    // std::env::set_var("HOSTED_BMP_ONLY", "1");
    // std::env::set_var("ENABLE_RTT", "1");

    // Add logic to invoke the external build system (e.g., `make`)
    // let make_output = std::process::Command::new("blackmagic")
    //     .current_dir("./blackmagic")
    //     .env("PROBE_HOST", "hosted")
    //     .env("HOSTED_BMP_ONLY", 1.to_string())
    //     .env("ENABLE_RTT", 1.to_string())
    //     .status()
    //     .expect("Failed to run 'make'");

    // if !make_output.success() {
    //     panic!("External C project build failed");
    // }
}
