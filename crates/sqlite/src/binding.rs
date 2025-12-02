/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  sqlite
 * Module: binding
 *
 * Purpose:
 *   Contains various helpers and trait implementations for "nice" binding
 *   of values in SQLite statements.
 */

use crate::{Error, Statement};

/**
 *
 * Implementing this trait allows for different strategies on presenting the
 * binding index for a statement parameter when needed.
 *
 **/
pub trait BindingIndex {
    fn index(self, _: &Statement) -> Result<i32, Error>;
}

/**
 *
 * Direct index binding is the "normal" SQLite way. So, no lookup necessary.
 *
 **/
impl BindingIndex for i32 {
    #[inline]
    fn index(self, _: &Statement) -> Result<i32, Error> {
        Ok(self)
    }
}

/**
 *
 * SQLite APIs exist for converting a string into an binding index.
 *
 **/
impl BindingIndex for &'static str {
    #[inline]
    fn index(self, stmt: &Statement) -> Result<i32, Error> {
        let name = std::ffi::CString::new(self).expect("failed name conversion to cstring");

        match unsafe { crate::ffi::sqlite3_bind_parameter_index(stmt.as_ptr(), name.as_ptr()) } {
            0 => Err(Error::BindingNameUnknown(self.to_string())),
            v => Ok(v),
        }
    }
}

/**
 *
 * Implementing this trait allows options for binding values into SQLite
 * statements. Each datatype in the SQLite API requires a different function
 * that could have different calling patterns.
 *
 **/
pub trait BindingValue {
    fn bind(self, stmt: &Statement, index: i32) -> Result<(), Error>;
}

/**
 *
 * SQLite binding for 'REAL' values.
 *
 **/
impl BindingValue for f64 {
    #[inline]
    fn bind(self, stmt: &Statement, index: i32) -> Result<(), Error> {
        unsafe { crate::ffi::sqlite3_bind_double(stmt.as_ptr(), index, self) }
            .into_result()
            .map_err(Error::StatementBinding)
    }
}

/**
 *
 * SQLite binding for 'INT' & 'INTEGER' values that are specifically 32-bit or less.
 *
 **/
impl BindingValue for i32 {
    #[inline]
    fn bind(self, stmt: &Statement, index: i32) -> Result<(), Error> {
        unsafe { crate::ffi::sqlite3_bind_int(stmt.as_ptr(), index, self) }
            .into_result()
            .map_err(Error::StatementBinding)
    }
}

/**
 *
 * SQLite binding for 'INT' & 'INTEGER' values that are larger than 32-bit.
 *
 **/
impl BindingValue for i64 {
    #[inline]
    fn bind(self, stmt: &Statement, index: i32) -> Result<(), Error> {
        unsafe { crate::ffi::sqlite3_bind_int64(stmt.as_ptr(), index, self) }
            .into_result()
            .map_err(Error::StatementBinding)
    }
}

/**
 *
 * SQLite binding for 'TEXT' values.
 *
 * NOTE: This is specifically for string values that are guaranteed to have
 *       a lifetime that exceeds the statement. Otherwise, we need to use a
 *       different strategy so that memory is not used after-free.
 *
 *       This can use the SQLite (transient) feature to force SQLite to do
 *       an allocate-copy and manage the string. Alternatively, we could do
 *       an allocation from rust and then pass a callback in argument #5 to
 *       release when SQLite says they are done.
 *
 **/
impl BindingValue for &'static str {
    #[inline]
    fn bind(self, stmt: &Statement, index: i32) -> Result<(), Error> {
        unsafe {
            crate::ffi::sqlite3_bind_text(
                stmt.as_ptr(),
                index,
                self.as_ptr() as *const _,
                self.len() as i32,
                None, // "static", so the value must survive until statement finalized
            )
        }
        .into_result()
        .map_err(Error::StatementBinding)
    }
}
