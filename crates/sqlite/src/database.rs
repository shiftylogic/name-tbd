/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  sqlite
 * Module: database
 *
 * Purpose:
 *   Contains a wrapper implementation around opening a SQLite database
 *   file and managing the lifetime.
 */

use {
    crate::{Error, Statement},
    std::{marker::PhantomData, path::Path},
};

/**
 *
 * Manages a connection to a SQLite database.
 *
 **/
#[derive(Clone)]
pub struct Database(std::rc::Rc<Connector>);

/**
 *
 * Database constructor functions
 *
 **/
impl Database {
    /**
     * Creates a new in-memory database instance.
     **/
    #[inline]
    pub fn in_memory() -> Result<Self, Error> {
        Self::open_internal(":memory:", Flags::memory())
    }

    /**
     * Attempts to open the specified SQLite database as read-only.
     **/
    #[inline]
    pub fn open_read<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Self::open_internal(path, Flags::readonly())
    }

    /**
     * Attempts to open the specified SQLite database for reading and writing.
     **/
    #[inline]
    pub fn open_readwrite<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Self::open_internal(path, Flags::readwrite())
    }

    /**
     * Attempts to open the specified SQLite database for reading and writing.
     * If the database does not exist, it will be created instead (also still
     * in reading and writing mode).
     **/
    #[inline]
    pub fn open_create<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Self::open_internal(path, Flags::create())
    }

    /**
     * Attempts to open the specified Sqlite database via the supplied URI.
     * This will set all the flags based on the structure of the URI.
     **/
    #[inline]
    pub fn open_uri<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Self::open_internal(path, Flags::uri())
    }

    /**
     * Internal helper for opening the database given a set of flags.
     **/
    #[inline]
    fn open_internal<P: AsRef<Path>>(path: P, flags: Flags) -> Result<Self, Error> {
        let db_path = std::ffi::CString::new(
            path.as_ref()
                .to_str()
                .expect("failed path conversion to string"),
        )
        .expect("failed path conversion to cstring");

        let mut db = std::ptr::null_mut();
        unsafe {
            crate::ffi::sqlite3_open_v2(
                db_path.as_ptr(),
                &mut db,
                flags.0 as ::core::ffi::c_int,
                std::ptr::null(),
            )
        }
        .into_result()
        .map_err(Error::DatabaseOpen)
        .map(|_| {
            Self(std::rc::Rc::new(Connector {
                ptr: db,
                _marker: PhantomData,
            }))
        })
    }
}

/**
 *
 * SQLite statement functions
 *
 **/
impl Database {
    /**
     * Execute SQL statement(s) without care for the results.
     *
     * NOTE: This returns success or fail and results (if any) are ignored.
     **/
    #[inline]
    pub fn execute<S: AsRef<str>>(&self, statement: S) -> Result<(), Error> {
        let sql = std::ffi::CString::new(statement.as_ref())
            .expect("failed statement conversion to cstring");

        unsafe {
            crate::ffi::sqlite3_exec(
                self.0.ptr,
                sql.as_ptr(),
                None,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        }
        .into_result()
        .map_err(Error::DatabaseExecute)
    }

    /**
     *
     * Constructs a new 'prepared' statement that can be executed indefinitely
     * while the database is still active.
     *
     **/
    #[inline]
    pub fn prepare<S: AsRef<str>>(&self, statement: S) -> Result<Statement, Error> {
        let sql = std::ffi::CString::new(statement.as_ref())
            .expect("failed statement conversion to cstring");

        let mut stmt = std::ptr::null_mut();
        unsafe {
            crate::ffi::sqlite3_prepare(
                self.0.ptr,
                sql.as_ptr(),
                -1,
                &mut stmt,
                std::ptr::null_mut(),
            )
        }
        .into_result()
        .map_err(Error::StatementPrepare)
        .map(|_| Statement::new(self.clone(), stmt))
    }
}

/**
 *
 * Internal structure to manage the 'C' connector object to the
 * SQLite object managing the database connection.
 *
 **/
struct Connector {
    ptr: *mut crate::ffi::Sqlite3,
    _marker: PhantomData<crate::ffi::Sqlite3>,
}

/**
 *
 * We need to ensure the database is closed when it is dropped.
 *
 **/
impl Drop for Connector {
    #[inline]
    fn drop(&mut self) {
        unsafe { crate::ffi::sqlite3_close_v2(self.ptr) };
    }
}

/**
 *
 * SQLite "open" flag builder
 *
 **/
struct Flags(u32);

impl Flags {
    fn readonly() -> Self {
        Self(1) /* SQLITE_OPEN_READONLY */
    }

    fn readwrite() -> Self {
        Self(2) /* SQLITE_OPEN_READWRITE */
    }

    fn create() -> Self {
        Self(4 | 2) /* SQLITE_OPEN_CREATE | SQLITE_OPEN_READWRITE */
    }

    fn memory() -> Self {
        Self(0x80 | 4 | 2) /* SQLITE_OPEN_MEMORY | SQLITE_OPEN_CREATE | SQLITE_OPEN_READWRITE */
    }

    fn uri() -> Self {
        Self(0x40) /* SQLITE_OPEN_URI */
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
    fn in_memory() {
        crate::init().expect("failed sqlite initialization");
        Database::in_memory().expect("failed to create in-memory database");
    }

    #[test]
    #[should_panic(expected = "sqlite db open error 'unable to open database file' (code: 14)")]
    fn open() {
        crate::init().expect("failed sqlite initialization");
        match Database::open_read("some-file-that-does-not-exist.db") {
            Ok(_) => panic!("should have failed"),
            Err(err) => panic!("{err:?}"),
        }
    }

    #[test]
    fn basic_create_insert() {
        crate::init().expect("failed sqlite initialization");
        let db = Database::in_memory().expect("failed to create in-memory database");

        db.execute(
            r####"
                CREATE TABLE things (
                    "id"    INTEGER NOT NULL UNIQUE,
                    "name"  TEXT NOT NULL,

                    PRIMARY KEY("id" AUTOINCREMENT)
                ) STRICT;
            "####,
        )
        .expect("failed table create");

        db.execute("INSERT INTO things (name) VALUES ('item 1');")
            .expect("failed table insert #1");
        db.execute("INSERT INTO things (name) VALUES ('item 2');")
            .expect("failed table insert #2");
        db.execute("INSERT INTO things (name) VALUES ('item 3');")
            .expect("failed table insert #3");
    }

    #[test]
    #[should_panic(expected = "sqlite db execute error 'SQL logic error' (code: 1)")]
    fn basic_query_fail() {
        crate::init().expect("failed sqlite initialization");
        let db = Database::in_memory().expect("failed to create in-memory database");

        match db.execute("SELECT * FROM things;") {
            Ok(_) => panic!("should have failed"),
            Err(err) => panic!("{err:?}"),
        }
    }
}
