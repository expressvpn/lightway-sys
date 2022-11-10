/*!
 * Contains the build process for Lightway
 */

extern crate bindgen;
extern crate wolfssl_sys;

use std::env;
use std::path::Path;
use std::process::Command;

static LIGHTWAY_VERSION: &str = "lightway-core-1.7.0";

/**
 * Extract Lightway Core
 */
fn extract_lightway(dest: &str) -> std::io::Result<()> {
    Command::new("tar")
        .arg("-zxvf")
        .arg(format!("vendor/{}.tar.gz", LIGHTWAY_VERSION))
        .arg("-C")
        .arg(dest)
        .status()
        .unwrap();

    Ok(())
}

/**
 * Build Lightway Core (libhelium)
 */
fn build_lightway(dest: &str) {
    let include_path = Path::new(dest).join(format!("{}/include", LIGHTWAY_VERSION));
    let src_path = Path::new(dest).join(format!("{}/src/he", LIGHTWAY_VERSION));
    let mut build = cc::Build::new();

    // Specify the Lightway Core source files - ensures minimum code included
    let src_paths = [
        src_path.join("wolf.c"),
        src_path.join("ssl_ctx.c"),
        src_path.join("plugin_chain.c"),
        src_path.join("msg_handlers.c"),
        src_path.join("memory.c"),
        src_path.join("flow.c"),
        src_path.join("core.c"),
        src_path.join("conn.c"),
        src_path.join("config.c"),
        src_path.join("client.c"),
    ];

    // Configure the build
    build
        .include(include_path)
        .files(src_paths)
        .static_flag(true);

    // If the WolfSSL dependency isn't set, then panic - we can't build without it
    if let Some(include) = std::env::var_os("DEP_WOLFSSL_INCLUDE") {
        build.include(format!("{}/include", include.to_str().unwrap()));
    } else {
        panic!("No WolfSSL include!");
    }

    // Build Lightway Core
    build.compile("helium");
}

fn main() -> std::io::Result<()> {
    // Get the build directory
    let outdir_string = env::var("OUT_DIR").unwrap();

    // Extract Lightway
    extract_lightway(&outdir_string)?;

    // Build Lightway
    build_lightway(&outdir_string);

    let dst = Path::new(&outdir_string);

    // Build the Rust binding
    let bindings = bindgen::Builder::default()
        .header(format!("{}/{}/public/he.h", outdir_string, LIGHTWAY_VERSION))
        .clang_arg(format!("-I{}/include/", outdir_string))
        .rustfmt_bindings(true)
        .blocklist_file("/usr/include/features.h")
        .blocklist_file("/usr/include/stdc-predef.h")
        .blocklist_file("/usr/include/stdbool.h")
        .blocklist_file("/usr/include/string.h")
        .blocklist_file("/usr/include/stdlib.h")
        .blocklist_file("/usr/include/stdint.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write out the bindings
    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Tell cargo to tell rustc to link in WolfSSL
    println!("cargo:rustc-link-lib=static=helium");
    println!("cargo:rustc-link-lib=static=wolfssl");

    // Provide the public C header
    println!(
        "cargo:include={}",
        format!("{}/lightway-core-1.1/public/he.h", outdir_string)
    );

    Ok(())
}
