/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  sqlite
 * Module: ffi
 *
 * Purpose:
 *   Contains all the SQLite FFI definitions used by this crate.
 */

/**
 *
 * Constants from SQLite APIs
 *
 **/
pub const SQLITE_OK: ::core::ffi::c_int = 0;
pub const SQLITE_ROW: ::core::ffi::c_int = 100;
pub const SQLITE_DONE: ::core::ffi::c_int = 101;

pub const SQLITE_INTEGER: ::core::ffi::c_int = 1;
pub const SQLITE_FLOAT: ::core::ffi::c_int = 2;
pub const SQLITE_TEXT: ::core::ffi::c_int = 3;
pub const SQLITE_BLOB: ::core::ffi::c_int = 4;
pub const SQLITE_NULL: ::core::ffi::c_int = 5;

/**
 *
 * Opaque SQLite database object as defined in 'C' library.
 *
 **/
#[repr(C)]
pub struct Sqlite3 {
    _unused: [u8; 0],
}

/**
 *
 * Opaque SQLite statement object as defined in 'C' library.
 *
 **/
#[repr(C)]
pub struct Sqlite3Stmt {
    _unused: [u8; 0],
}

/**
 *
 * Definition of the callback function used for one-time statement
 * execution.
 *
 **/
type ExecCallback = unsafe extern "C" fn(
    context: *mut ::core::ffi::c_void,
    column_count: ::core::ffi::c_int,
    column_text: *const *const ::core::ffi::c_char,
    column_names: *const *const ::core::ffi::c_char,
);

/**
 *
 * Definition of the memory de-allocator function that is taken
 * by some API calls.
 *
 **/
type MemDeallocCallback = unsafe extern "C" fn(ptr: *mut ::core::ffi::c_void);

/*+
 *
 * Replacement "return value" for SQLite FFI APIs.
 *
 **/
#[repr(C)]
pub struct SqliteResult(::core::ffi::c_int);

impl SqliteResult {
    pub fn into_result(self) -> Result<(), ::core::ffi::c_int> {
        match self.0 {
            SQLITE_OK => Ok(()),
            _ => Err(self.0),
        }
    }
}

/*+
 *
 * Definitions of the 'unsafe' SQLite C APIs (added as needed).
 *
 */
unsafe extern "C" {
    pub fn sqlite3_libversion() -> *const ::core::ffi::c_char;
    pub fn sqlite3_libversion_number() -> ::core::ffi::c_int;

    pub fn sqlite3_errstr(code: ::core::ffi::c_int) -> *const ::core::ffi::c_char;

    pub fn sqlite3_open_v2(
        db_file: *const ::core::ffi::c_char,
        db_out: *mut *mut Sqlite3,
        flags: ::core::ffi::c_int,
        vfs_module: *const ::core::ffi::c_char,
    ) -> SqliteResult;

    pub fn sqlite3_close_v2(db: *mut Sqlite3) -> SqliteResult;

    pub fn sqlite3_exec(
        db: *mut Sqlite3,
        sql: *const ::core::ffi::c_char,
        callback: ::core::option::Option<ExecCallback>,
        context: *mut ::core::ffi::c_void,
        errmsg_out: *mut *const ::core::ffi::c_char,
    ) -> SqliteResult;

    pub fn sqlite3_prepare(
        db: *mut Sqlite3,
        sql: *const ::core::ffi::c_char,
        sql_bytes: ::core::ffi::c_int,
        stmt_out: *mut *mut Sqlite3Stmt,
        tail_out: *mut *const ::core::ffi::c_char,
    ) -> SqliteResult;

    pub fn sqlite3_clear_bindings(stmt: *mut Sqlite3Stmt) -> SqliteResult;
    pub fn sqlite3_data_count(stmt: *mut Sqlite3Stmt) -> ::core::ffi::c_int;
    pub fn sqlite3_reset(stmt: *mut Sqlite3Stmt) -> SqliteResult;
    pub fn sqlite3_step(stmt: *mut Sqlite3Stmt) -> ::core::ffi::c_int;
    pub fn sqlite3_finalize(stmt: *mut Sqlite3Stmt) -> SqliteResult;

    pub fn sqlite3_bind_parameter_index(
        stmt: *mut Sqlite3Stmt,
        name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;

    pub fn sqlite3_bind_double(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    ) -> SqliteResult;

    pub fn sqlite3_bind_int(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
        value: ::core::ffi::c_int,
    ) -> SqliteResult;

    pub fn sqlite3_bind_int64(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
        value: ::core::ffi::c_longlong,
    ) -> SqliteResult;

    pub fn sqlite3_bind_null(stmt: *mut Sqlite3Stmt, index: ::core::ffi::c_int) -> SqliteResult;

    pub fn sqlite3_bind_text(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
        text: *const ::core::ffi::c_char,
        text_bytes: ::core::ffi::c_int,
        free: ::core::option::Option<MemDeallocCallback>,
    ) -> SqliteResult;

    pub fn sqlite3_column_count(stmt: *mut Sqlite3Stmt) -> ::core::ffi::c_int;
    pub fn sqlite3_column_name(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    pub fn sqlite3_column_type(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    // pub fn sqlite3_column_blob(
    //     stmt: *mut Sqlite3Stmt,
    //     index: ::core::ffi::c_int,
    // ) -> *const ::core::ffi::c_void;
    pub fn sqlite3_column_double(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    pub fn sqlite3_column_int(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn sqlite3_column_int64(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    pub fn sqlite3_column_text(
        stmt: *mut Sqlite3Stmt,
        index: ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
}
