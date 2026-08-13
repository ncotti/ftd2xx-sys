// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Nicolas Gabriel Cotti

//! Unsafe bindings from the libftd2xx library, automatically created with
//! bindgen.
//!
//! To build and execute the code, the library must have been installed in
//! a "visible path".
//!
//! Supported paths are:
//! * `/usr/lib` and `/usr/include`.
//! * `/usr/local/lib` and `/usr/local/include`
//! * Any path set in the `LD_LIBRARY_PATH` env. variable.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(missing_docs)]

pub mod d2xx {
    include!(concat!(env!("OUT_DIR"), "/d2xx_bindings.rs"));
}

#[cfg(feature = "mpsse")]
pub mod mpsse_i2c {
    include!(concat!(env!("OUT_DIR"), "/mpsse_i2c_bindings.rs"));
}

#[cfg(feature = "mpsse")]
pub mod mpsse_spi {
    include!(concat!(env!("OUT_DIR"), "/mpsse_spi_bindings.rs"));
}
