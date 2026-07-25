// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Nicolas Gabriel Cotti

use std::env;
use std::path::PathBuf;

const LIB_NAME: &str = "ftd2xx";
const DYNAMIC_LIB_NAME: &str = "libftd2xx.so";
const STATIC_LIB_NAME: &str = "libftd2xx.a";
const HEADER_NAME: &str = "ftd2xx.h";

/// Contains the path to the library and header
struct LibPaths {
    dynamic_lib: Option<PathBuf>,
    static_lib: Option<PathBuf>,
    header: Option<PathBuf>,
}

/// Checks whether the libftd2xx is already installed in your system.
/// Returns the library and header paths as `Option<PathBuf>`, which may
/// be `None` if they couldn't be found.
/// Library and headers are searched in common directories, plus the env.
/// variables "LD_LIBRARY_PATH"
fn get_system_lib_paths() -> LibPaths {
    let mut possible_lib_paths: Vec<PathBuf> =
        vec![PathBuf::from("/usr/local/lib"), PathBuf::from("/usr/lib")];

    let mut possible_headers: Vec<PathBuf> = vec![
        PathBuf::from("/usr/local/include").join(HEADER_NAME),
        PathBuf::from("/usr/include").join(HEADER_NAME),
    ];

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
                possible_headers.insert(0, absolute_path_from_env.join(HEADER_NAME));
            }
        };
    }

    let possible_dynamic_libs: Vec<PathBuf> = possible_lib_paths
        .clone()
        .into_iter()
        .map(|path| path.join(DYNAMIC_LIB_NAME))
        .collect();
    let possible_static_libs: Vec<PathBuf> = possible_lib_paths
        .into_iter()
        .map(|path| path.join(STATIC_LIB_NAME))
        .collect();

    let dynamic_lib = possible_dynamic_libs.into_iter().find(|path| path.exists());
    let static_lib = possible_static_libs.into_iter().find(|path| path.exists());
    let header = possible_headers.into_iter().find(|path| path.exists());

    LibPaths {
        dynamic_lib: dynamic_lib,
        static_lib: static_lib,
        header: header,
    }
}

fn main() {
    let feature_static = env::var_os("CARGO_FEATURE_STATIC").is_some();

    println!("cargo:rerun-if-env-changed=LD_LIBRARY_PATH");

    // Trying to find the library in the system path
    let lib_paths = get_system_lib_paths();

    if (lib_paths.header.is_none())
        || (feature_static && lib_paths.static_lib.is_none())
        || (!feature_static && lib_paths.dynamic_lib.is_none())
    {
        panic!(
            r#"Couldn't find system library "libftd2xx" installed.
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
        println!("cargo:rustc-link-lib=static={}", LIB_NAME);
    } else {
        println!("cargo:rustc-link-lib=dylib={}", LIB_NAME);
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
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
