/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  sqlite
 * Module: error
 *
 * Purpose:
 *   Contains the Error object and defines all errors emitted by this
 *   crate along with some extra logic to gain debug information about
 *   the errors that are specific to the C APIs.
 */

/**
 *
 * Defines all possible 'custom' errors that can return from
 * the public APIs provided by this crate.
 *
 **/
#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    LibraryInitialization(i32),
    DatabaseOpen(i32),
    DatabaseExecute(i32),
    StatementPrepare(i32),
    StatementBinding(i32),
    StatementExecute(i32),
    StatementReset(i32),
    BindingNameUnknown(String),
    ColumnIndexInvalid(u32),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut w = |s, c| write!(f, "sqlite {s} error '{}' (code: {c})", get_error_string(c));

        match self {
            Self::LibraryInitialization(code) => w("initialization", *code),
            Self::DatabaseOpen(code) => w("db open", *code),
            Self::DatabaseExecute(code) => w("db execute", *code),
            Self::StatementPrepare(code) => w("statement prepare", *code),
            Self::StatementBinding(code) => w("statement binding", *code),
            Self::StatementExecute(code) => w("statement execute", *code),
            Self::StatementReset(code) => w("statement reset", *code),
            Self::BindingNameUnknown(name) => {
                write!(f, "sqlite binding lookup failed (name: '{name}')")
            }
            Self::ColumnIndexInvalid(idx) => {
                write!(
                    f,
                    "sqlite statement column name fetch failed (index: {idx})"
                )
            }
        }
    }
}

/**
 *
 * Helper wrapper around an SQLite API for obtaining the error string for a
 * given error code. These strings are managed within the library and should
 * NOT be released in any way.
 *
 **/
#[inline]
fn get_error_string(code: i32) -> &'static str {
    unsafe {
        ::core::ffi::CStr::from_ptr(crate::ffi::sqlite3_errstr(code))
            .to_str()
            .expect("invalid utf-8 error string")
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
    fn error_debug_strings() {
        assert_eq!(
            format!("{:?}", Error::LibraryInitialization(1)),
            "sqlite initialization error 'SQL logic error' (code: 1)"
        );
        assert_eq!(
            format!("{:?}", Error::LibraryInitialization(2)),
            "sqlite initialization error 'unknown error' (code: 2)"
        );
        assert_eq!(
            format!("{:?}", Error::LibraryInitialization(21)),
            "sqlite initialization error 'bad parameter or other API misuse' (code: 21)"
        );

        assert_eq!(
            format!("{:?}", Error::DatabaseOpen(1)),
            "sqlite db open error 'SQL logic error' (code: 1)"
        );
        assert_eq!(
            format!("{:?}", Error::DatabaseOpen(11)),
            "sqlite db open error 'database disk image is malformed' (code: 11)"
        );
        assert_eq!(
            format!("{:?}", Error::DatabaseOpen(14)),
            "sqlite db open error 'unable to open database file' (code: 14)"
        );
    }
}
