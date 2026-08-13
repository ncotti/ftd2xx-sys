// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Nicolas Gabriel Cotti

use ftd2xx_sys::d2xx;

#[test]
fn get_library_version() {
    // 0x010435 stands for v01.04.35
    const EXPECTED_VERSION: u32 = 0x010435;

    let mut version: u32 = 0;
    let status: d2xx::FT_STATUS = unsafe { d2xx::FT_GetLibraryVersion(&mut version) };
    assert!(status == d2xx::FT_OK);
    assert!(version == EXPECTED_VERSION);
}

#[cfg(feature = "mpsse")]

mod mpsse_tests {
    use ftd2xx_sys::mpsse_i2c;
    use ftd2xx_sys::mpsse_spi;

    #[test]
    fn init_and_clean_mpsse_spi() {
        unsafe { mpsse_spi::Init_libMPSSE() };
        unsafe { mpsse_spi::Cleanup_libMPSSE() };
    }

    #[test]
    fn init_and_clean_mpsse_i2c() {
        unsafe { mpsse_i2c::Init_libMPSSE() };
        unsafe { mpsse_i2c::Cleanup_libMPSSE() };
    }
}
