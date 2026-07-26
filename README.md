# FTD2XX-SYS

This crate generates Rust FFI (Foreign Function Interface) bindings for the [FTDI D2XX library][ftdi_lib] using [bindgen][bindgen].

For its usage, the `libftd2xx` must be installed in one of the following paths:

* `/usr/lib` and `/usr/include`.
* `/usr/local/lib` and `/usr/local/include`
* Any path set in the `LD_LIBRARY_PATH` env. variable.

The `LD_LIBRARY_PATH` variable takes precedence over the system paths.

To install the library into `/usr/local/lib`, a helper script `install_d2xx.sh` is provided.

```bash
./install_d2xx.sh [path_to_decompressed_lib] [version]
```

<!-- External links -->
[ftdi_lib]: https://ftdichip.com/drivers/d2xx-drivers/
[bindgen]: https://rust-lang.github.io/rust-bindgen/
