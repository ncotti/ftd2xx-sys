// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Nicolas Gabriel Cotti

use cotti_build_support as bsup;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use tempdir::TempDir;

struct OsInfo {
    os: String,
    arch: String,
    lib_version: String,
    lib_src_path: String,
}

fn check_os() -> OsInfo {
    let os = match std::env::var("CARGO_CFG_TARGET_OS").as_deref() {
        Ok("linux") => String::from("linux"),
        Ok(e) => {
            panic!("Unsupported platform: {e}. Only Linux is supported")
        }
        Err(e) => {
            panic!("Error: {e}")
        }
    };

    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH").as_deref() {
        Ok("x86_64") => String::from("x86_64"),
        _ => {
            panic!("Unsupported architecture. Only x86_64 is supported")
        }
    };

    OsInfo {
        os: os,
        arch: arch,
        lib_version: std::env::var("LIBFTD2XX_VERSION").unwrap(),
        lib_src_path: std::env::var("LIBFTD2XX_SRC_DIR").unwrap(),
    }
}

fn check_lib_installed() -> bool {
    let possible_lib_paths: [&Path; 2] = [
        Path::new("/usr/local/lib/libftd2xx.so"),
        Path::new("/usr/lib/libftd2xx.so"),
    ];

    let possible_header_paths = [
        Path::new("/usr/local/include/ftd2xx.h"),
        Path::new("/usr/include/ftd2xx.h"),
    ];

    let lib_exists = (possible_lib_paths.iter().any(|path| path.exists()))
        && (possible_header_paths.iter().any(|path| path.exists()));

    println!("Lib exists? {:?}", lib_exists);

    lib_exists
}

/// Installs the libtfd2xx library in OUT_DIR, i.e., in a temporal location
/// relative to this package build.
fn install_lib(os_info: OsInfo) -> PathBuf {
    let tar = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(&os_info.lib_src_path)
        .join(format!(
            "libftd2xx-{}-{}-{}.tgz",
            os_info.os, os_info.arch, os_info.lib_version
        ));

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    bsup::untar(tar, &out_dir).expect("Untar library file should succeed");

    let untared_folder_name = format!("{}-{}", os_info.os, os_info.arch);

    let output = out_dir.join(untared_folder_name);
    output

    // let lib_glob = out_dir
    //     .join(&untared_folder_name)
    //     .join("libftd2xx.*");
    // let header_glob = out_dir
    //     .join(&untared_folder_name)
    //     .join("*.h");

    // TODO, put files in OUT_DIR for build
    //let lib_install_path = Path::new("/usr/local/lib");
    //let header_install_path = Path::new("/usr/local/include");

    //bsup::install(lib_glob, &lib_install_path).expect("Ok");
    //bsup::install(header_glob, &header_install_path).expect("Ok");
}

fn main() {
    let os_info = check_os();

    let lib_path = install_lib(os_info);

    // if ! check_lib_installed() {
    //     println!("Installing lib");
    //     install_lib(os_info);
    // }

    let link_search_path: String =
        format!("cargo:rustc-link-search={}", lib_path.to_string_lossy());

    // Tell cargo to look for shared libraries in the specified directory
    // Similar to "-L" flag
    println!("{}", link_search_path);

    // Tell cargo to tell rustc to link the system ftd2xx shared library.
    // Similar to "-l" flag
    println!("cargo:rustc-link-lib=ftd2xx");

    let header = PathBuf::from(lib_path).join("ftd2xx.h");
    let header = header.to_string_lossy();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
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
