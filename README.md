# FTD2XX-SYS

This crate generates Rust FFI (Foreign Function Interface) bindings for the [FTDI D2XX library][ftdi_lib] and the [FTDI MPSSE I2C & SPI libraries][ftdi_mpsse_lib] using [bindgen][bindgen].

For its usage, the `libftd2xx` (and optionally `libmpsse`) must be installed in one of the following paths:

* `/usr/lib` and `/usr/include`.
* `/usr/local/lib` and `/usr/local/include`
* Any path set in the `LD_LIBRARY_PATH` env. variable.

The `LD_LIBRARY_PATH` variable takes precedence over the system paths.

To install the libraries into `/usr/local/lib`, a helper script `install_libs.sh` is provided.

```bash
./install_d2xx.sh <path_to_libftd2xx.so> [path_to_libmpsse.so]
```

You may choose to compile the library statically or dynamically by setting the `static` feature in your Cargo.toml. By default, the library is dynamically linked.
Also, you can optionally generate the bindings for the `libmpsse`, provided that it is installed in your system. By default, bindings for this library won't be generated.

```toml
[dependencies]
ftd2xx-sys = { version = "x.x.x", features = ["static", "mpsse"] }
```

<!-- External links -->
<!--Note: Lychee fails to with status code: 403 Forbidden for FTDI links -->
[ftdi_lib]: https://ftdichip.com/drivers/d2xx-drivers/
[ftdi_mpsse_lib]: https://ftdichip.com/software-examples/mpsse-projects/
[bindgen]: https://rust-lang.github.io/rust-bindgen/
