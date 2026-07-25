// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Nicolas Gabriel Cotti

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    struct Version {
        major: u8,
        minor: u8,
        build: u8,
    }

    #[test]
    fn hi() {
        let mut version: u32 = 0;
        let status: FT_STATUS = unsafe {
            FT_GetLibraryVersion(&mut version)
        };
        assert!(status == FT_OK);

        let [build, minor, major, _] = version.to_le_bytes();

        let version  = Version{
            major,
            minor,
            build,
        };
        let version_str = format!("v{}.{}.{}", version.major, version.minor, version.build);

        assert!(version_str == "v1.4.53");
    }
}