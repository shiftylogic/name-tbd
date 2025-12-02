/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  sqlite
 * Module: init
 *
 * Purpose:
 *   The SQLite C library requires a call to 'sqlite3_initialize' followed
 *   by a call to sqlite3_shutdown when done. Compiling the library with the
 *   SQLITE_OMIT_AUTOINIT gains a bit of performance per API call but at the
 *   cost of having to initialize the library.
 *
 *   Since we initialize the first time but never actually do a shutdown, the
 *   shutdown is unnecessary. The library will live (once used) throughout the
 *   duration of the application. Given the use case for this library, this
 *   shouldn't be a problem and we are only talking about an allocation of a
 *   few small objects (locks, etc) by the library.
 */

use {super::Error, std::sync::LazyLock};

/**
 *
 * The public interface to this initialization is a single 'init' function
 * that returns a standard Rust Result<(), Error>. Internally, the function
 * manages a single instance of the initializer.
 *
 **/
pub fn init() -> Result<(), Error> {
    static INSTANCE: LazyLock<i32> = LazyLock::new(|| unsafe { sqlite3_initialize() });

    match *INSTANCE {
        0 => Ok(()),
        err => Err(Error::LibraryInitialization(err)),
    }
}

/*+
 *
 * Definitions of the 'unsafe' SQLite C APIs that are used in this module.
 *
 */
unsafe extern "C" {
    fn sqlite3_initialize() -> ::core::ffi::c_int;
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
    fn check_lib_init() {
        // Extra inits are benign
        init().expect("failed initialization #1");
        init().expect("failed initialization #2");
        init().expect("failed initialization #3");
    }
}
