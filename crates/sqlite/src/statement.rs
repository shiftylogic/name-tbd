/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  sqlite
 * Module: statement
 *
 * Purpose:
 *   Implements a wrapper around SQLite prepared statements attached to a specific
 *   database connection. The database connection will persist as long as the created
 *   statement(s) exist.
 */

use {
    crate::{BindingIndex, BindingValue, Database, Error},
    std::marker::PhantomData,
};

pub struct Statement {
    stmt: *mut crate::ffi::Sqlite3Stmt,
    _db: Database,
    _marker: PhantomData<crate::ffi::Sqlite3Stmt>,
}

/**
 *
 * Statement constructor functions
 *
 * NOTE: These constructors should all be internal to the crate. External usage must
 *       use the "prepare" method on the Database connection since all statements are
 *       tied to a specific Database instance.
 *
 **/
impl Statement {
    #[inline]
    pub(crate) fn new(db: Database, stmt: *mut crate::ffi::Sqlite3Stmt) -> Self {
        Self {
            stmt,
            _db: db,
            _marker: PhantomData,
        }
    }

    /**
     * Internal helper for accessing the underlying opaque 'C' 'handle'.
     **/
    #[inline]
    pub(crate) unsafe fn as_ptr(&self) -> *mut crate::ffi::Sqlite3Stmt {
        self.stmt
    }
}

/**
 *
 * Statement methods for bindings , etc.
 *
 **/
impl Statement {
    #[inline]
    pub fn bind<V: BindingValue>(&self, index: i32, value: V) -> Result<&Self, Error> {
        value.bind(self, index)?;
        Ok(self)
    }

    #[inline]
    pub fn bind_indirect<I, V>(&self, index: I, value: V) -> Result<&Self, Error>
    where
        I: BindingIndex,
        V: BindingValue,
    {
        value.bind(self, index.index(self)?)?;

        Ok(self)
    }

    #[inline]
    pub fn bind_null<I>(&self, index: I) -> Result<&Self, Error>
    where
        I: BindingIndex,
    {
        unsafe { crate::ffi::sqlite3_bind_null(self.stmt, index.index(self)?) }
            .into_result()
            .map_err(Error::StatementBinding)
            .map(|_| self)
    }

    #[inline]
    pub fn bind_index<I: BindingIndex>(&self, index: I) -> Result<i32, Error> {
        index.index(self)
    }
}

/**
 *
 * Statement methods for executing + resetting.
 *
 **/
impl Statement {
    #[inline]
    pub fn execute(&self) -> Result<u32, Error> {
        let mut count = 0;
        loop {
            count += 1;
            match unsafe { crate::ffi::sqlite3_step(self.stmt) } {
                crate::ffi::SQLITE_ROW => continue,
                crate::ffi::SQLITE_DONE => return Ok(count),
                err => return Err(Error::StatementExecute(err)),
            }
        }
    }

    #[inline]
    pub fn reset(&self) -> Result<&Self, Error> {
        unsafe { crate::ffi::sqlite3_reset(self.stmt) }
            .into_result()
            .map_err(Error::StatementReset)
            .map(|_| self)
    }
}

/**
 *
 * We need to ensure the statement is "finalized" (i.e. freed) when
 * this object is dropped.
 *
 **/
impl Drop for Statement {
    #[inline]
    fn drop(&mut self) {
        unsafe { crate::ffi::sqlite3_finalize(self.stmt) };
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
    fn basic_select() {
        crate::init().expect("failed sqlite initialization");

        let db = Database::in_memory().expect("failed to create in-memory database");
        db.execute(
            r####"
                CREATE TABLE people (
                    "id"    INTEGER NOT NULL UNIQUE,
                    "name"  TEXT NOT NULL,

                    PRIMARY KEY("id" AUTOINCREMENT)
                ) STRICT;
            "####,
        )
        .expect("failed table create");

        let _stmt = db
            .prepare("SELECT * FROM people;")
            .expect("failed to prepare statement");
    }

    #[test]
    fn statement_reuse() {
        crate::init().expect("failed sqlite initialization");
        let db = Database::in_memory().expect("failed to create in-memory database");
        db.execute(
            r####"
                CREATE TABLE people (
                    "id"    INTEGER NOT NULL UNIQUE,
                    "name"  TEXT NOT NULL,

                    PRIMARY KEY("id" AUTOINCREMENT)
                ) STRICT;
            "####,
        )
        .expect("failed table create");

        let insert = db
            .prepare("INSERT INTO people (name) VALUES (?);")
            .expect("failed to prepare statement");

        for _ in 0..4 {
            assert_eq!(
                1,
                insert
                    .reset()
                    .expect("failed reset")
                    .bind(1, "person")
                    .expect("failed bind")
                    .execute()
                    .expect("failed insert")
            );
        }
    }

    #[test]
    fn bind_lookup() {
        crate::init().expect("failed sqlite initialization");

        let db = Database::in_memory().expect("failed to create in-memory database");
        db.execute(
            r####"
                CREATE TABLE bindy (
                    "a1"    INTEGER,
                    "a2"    INTEGER,
                    "a3"    INTEGER,
                    "a4"    INTEGER,
                    "a5"    INTEGER,
                    "a6"    INTEGER
                ) STRICT;
            "####,
        )
        .expect("failed to create test table");

        let stmt = db
            .prepare(
                "INSERT INTO bindy (a1, a2, a3, a4, a5, a6) VALUES (:a1, :b, :c, :a4, :a5, ?);",
            )
            .expect("failed to create statement");

        assert_eq!(stmt.bind_index(":a1").unwrap(), 1);
        assert_eq!(stmt.bind_index(":b").unwrap(), 2);
        assert_eq!(stmt.bind_index(":c").unwrap(), 3);
        assert_eq!(stmt.bind_index(":a4").unwrap(), 4);
        assert_eq!(stmt.bind_index(":a5").unwrap(), 5);
        assert_eq!(stmt.bind_index(6).unwrap(), 6);
    }

    #[test]
    fn index_binding() {
        crate::init().expect("failed sqlite initialization");

        let db = Database::in_memory().expect("failed to create in-memory database");
        db.execute(
            r####"
                CREATE TABLE things (
                    "int_value"     INTEGER NOT NULL,
                    "text_value"    TEXT NOT NULL,
                    "maybe_null"    TEXT,
                    "int64_value"   INTEGER NOT NULL,
                    "double_value"  REAL NOT NULL
                ) STRICT;
            "####,
        )
        .expect("failed table create");

        assert_eq!(
            1,
            db.prepare(
                r####"
                INSERT INTO things (
                    int_value, text_value, maybe_null, int64_value, double_value
                ) VALUES (
                    ?, ?, ?, ?, ?
                )
            "####,
            )
            .expect("failed to prepare statement")
            .bind(1, 42)
            .expect("failed int binding by index")
            .bind(2, "some string")
            .expect("failed string binding by index")
            .bind_null(3)
            .expect("failed null binding by index")
            .bind(4, 38_i64)
            .expect("failed i64 binding by index")
            .bind(5, 87.897897_f64)
            .expect("failed double binding by index")
            .execute()
            .expect("failed to execute insert")
        );
    }

    #[test]
    fn name_binding() {
        crate::init().expect("failed sqlite initialization");

        let db = Database::in_memory().expect("failed to create in-memory database");
        db.execute(
            r####"
                CREATE TABLE things (
                    "iv"            INTEGER NOT NULL,
                    "tv"            TEXT NOT NULL,
                    "maybe_null"    TEXT,
                    "i64v"          INTEGER NOT NULL,
                    "dv"            REAL NOT NULL
                ) STRICT;
            "####,
        )
        .expect("failed table create");

        assert_eq!(
            1,
            db.prepare(
                r####"
                INSERT INTO things (
                    iv, tv, maybe_null, i64v, dv
                ) VALUES (
                    :iv, :tv, ?3, :i64v, ?5
                )
            "####,
            )
            .expect("failed to prepare statement")
            .bind_indirect(":iv", 42)
            .expect("failed binding by name ('iv')")
            .bind_indirect(":tv", "some string")
            .expect("failed string binding by index")
            .bind_null("?3")
            .expect("failed null binding by index")
            .bind_indirect(":i64v", 38_i64)
            .expect("failed i64 binding by index")
            .bind_indirect(5, 87.897897_f64)
            .expect("failed double binding by index")
            .execute()
            .expect("failed to execute insert")
        );
    }
}
