// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Nicolas Gabriel Cotti

use ftd2xx_sys::*;


#[test]
fn get_library_version() {
    // 0x010435 stands for v01.04.35
    const EXPECTED_VERSION: u32 = 0x010435;

    let mut version: u32 = 0;
    let status: FT_STATUS = unsafe { FT_GetLibraryVersion(&mut version) };
    assert!(status == FT_OK);
    assert!(version == EXPECTED_VERSION);
}
