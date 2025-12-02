/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  sqlite
 *
 * Purpose:
 *   Wrapper implementation around SQLite C library.
 */

mod binding;
mod database;
mod error;
mod ffi;
mod init;
mod statement;

pub use binding::*;
pub use database::Database;
pub use error::Error;
pub use init::init;
pub use statement::Statement;

/**
 *
 * Helper function that obtains the SQLite version number that is being used
 * by this crate as an integer value.
 *
 * Version number is a single number in the following form:
 *   (major * 1000000) + (minor * 1000) + patch
 *
 *   Ex.
 *      3051001 = "3.51.1"
 *
 **/
pub fn version_number() -> usize {
    unsafe { ffi::sqlite3_libversion_number() as usize }
}

/**
 *
 * Helper function that obtains the SQLite version number that is being used
 * by this crate as a pre-formatted string "major.minor.patch".
 *
 **/
pub fn version_string() -> &'static str {
    unsafe {
        ::core::ffi::CStr::from_ptr(ffi::sqlite3_libversion())
            .to_str()
            .expect("invalid utf-8 version string")
    }
}

/*+
 *
 * Tests
 *
 **/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        assert_eq!(version_number(), 3051001);
        assert_eq!(version_string(), "3.51.1");
    }
}
