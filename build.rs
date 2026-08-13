// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Nicolas Gabriel Cotti

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Contains the path to the library and header
#[derive(Debug)]
struct LibPaths {
    dynamic_lib: Option<PathBuf>,
    static_lib: Option<PathBuf>,
    header: Option<PathBuf>,
}

/// Creates a `wrapper.h` header file that includes all header files
/// listed in the "headers" input
fn merge_headers(lib_name: &str, headers: Vec<&str>) -> PathBuf {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let wrapper = lib_name.to_string() + "_wrapper.h";
    let wrapper_path = out_path.join(wrapper);

    let mut wrapper = File::create(&wrapper_path).unwrap();

    for header in headers {
        wrapper
            .write_all(format!("#include \"{}\"\n", header).as_bytes())
            .unwrap();
    }

    wrapper_path
}

/// Checks whether the library is already installed in your system.
///
/// `lib_name` is the name of the library without the "lib" prefix, like
/// `lib<lib_name>.so`.
/// `headers` is a vector which contains all headers required by the library.
/// To generate the bindgen, a single "wrapper.h" header will be created
/// which will contain `#include <header.h>` statements to all files listed.
///
/// Returns the library and wrapper header paths as `Option<PathBuf>`,
/// which may be `None` if they couldn't be found.
///
/// The library is searched in common directories, plus the env.
/// variable "LD_LIBRARY_PATH"
fn get_system_lib_paths(lib_name: &str, headers: Vec<&str>) -> LibPaths {
    let mut possible_lib_paths: Vec<PathBuf> =
        vec![PathBuf::from("/usr/local/lib"), PathBuf::from("/usr/lib")];

    let header_name = merge_headers(lib_name, headers);

    // The user may provide these env. variable to search for the library
    let env_vars = ["LD_LIBRARY_PATH"];

    for env_var in env_vars {
        if let Some(dirs) = env::var_os(env_var) {
            // The env. variable may have multiple dirs separated by semicolons
            for dir in dirs.to_string_lossy().split(":") {
                let absolute_path_from_env =
                    PathBuf::from(&dir).canonicalize().unwrap_or_else(|e| {
                        panic!("Path in {env_var}={:?} does not exists. Error: {e}", dir);
                    });
                possible_lib_paths.insert(0, absolute_path_from_env.clone());
            }
        };
    }

    let dyn_lib: String = format!("lib{}.so", lib_name);
    let static_lib: String = format!("lib{}.a", lib_name);

    let possible_dynamic_libs: Vec<PathBuf> = possible_lib_paths
        .clone()
        .into_iter()
        .map(|path| path.join(&dyn_lib))
        .collect();
    let possible_static_libs: Vec<PathBuf> = possible_lib_paths
        .into_iter()
        .map(|path| path.join(&static_lib))
        .collect();

    let dynamic_lib = possible_dynamic_libs.into_iter().find(|path| path.exists());
    let static_lib = possible_static_libs.into_iter().find(|path| path.exists());

    LibPaths {
        dynamic_lib: dynamic_lib,
        static_lib: static_lib,
        header: Some(header_name),
    }
}

fn generate_bindings(
    lib_name: &str,
    headers: Vec<&str>,
    feature_static: bool,
    bindings_file: &str,
) {
    let lib_paths = get_system_lib_paths(lib_name, headers);

    if (feature_static && lib_paths.static_lib.is_none())
        || (!feature_static && lib_paths.dynamic_lib.is_none())
    {
        panic!(
            r#"Couldn't find system library {lib_name} installed.
Please, do one of the following:
- Install the libftd2xx library in "/usr/local/lib".
- Set the "LD_LIBRARY_PATH" environment variable to the path where the library is installed.
See the crate documentation for details.
"#
        );
    }

    let lib_dir = match feature_static {
        true => lib_paths.static_lib.as_ref().unwrap().parent().unwrap(),
        false => lib_paths.dynamic_lib.as_ref().unwrap().parent().unwrap(),
    }
    .to_string_lossy();

    // Tell cargo to look for shared libraries in the specified directory
    // Similar to "-L" flag
    println!("cargo:rustc-link-search={}", lib_dir);

    // Add the library dir to the run-time search-path (only useful for this crate)
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir);

    // Tell cargo to tell rustc to link the system ftd2xx shared library.
    // Similar to "-l" flag
    if feature_static {
        println!("cargo:rustc-link-lib=static={}", lib_name);
    } else {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(lib_paths.header.unwrap().to_string_lossy())
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
        .write_to_file(out_path.join(bindings_file))
        .expect("Couldn't write bindings!");
}

fn main() {
    let feature_static = env::var_os("CARGO_FEATURE_STATIC").is_some();
    let feature_mpsse = env::var_os("CARGO_FEATURE_MPSSE").is_some();

    let d2xx_lib_name: &str = "ftd2xx";
    let d2xx_headers: Vec<&str> = vec!["ftd2xx.h"];
    let d2xx_bindings: &str = "d2xx_bindings.rs";

    let mpsse_i2c_lib_name: &str = "mpsse";
    let mpsse_i2c_headers: Vec<&str> = vec!["ftd2xx.h", "libmpsse_i2c.h"];
    let mpsse_i2c_bindings: &str = "mpsse_i2c_bindings.rs";

    let mpsse_spi_lib_name: &str = "mpsse";
    let mpsse_spi_headers: Vec<&str> = vec!["ftd2xx.h", "libmpsse_spi.h"];
    let mpsse_spi_bindings: &str = "mpsse_spi_bindings.rs";

    println!("cargo:rerun-if-env-changed=LD_LIBRARY_PATH");

    generate_bindings(d2xx_lib_name, d2xx_headers, feature_static, d2xx_bindings);

    if feature_mpsse {
        generate_bindings(
            mpsse_i2c_lib_name,
            mpsse_i2c_headers,
            feature_static,
            mpsse_i2c_bindings,
        );
        generate_bindings(
            mpsse_spi_lib_name,
            mpsse_spi_headers,
            feature_static,
            mpsse_spi_bindings,
        );
    }
}
