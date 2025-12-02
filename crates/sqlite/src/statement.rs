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
 * Statement methods for discovering and binding columns.
 *
 **/
impl Statement {
    #[inline]
    pub fn bind<V: BindingValue>(&mut self, index: i32, value: V) -> Result<&mut Self, Error> {
        value.bind(self, index)?;
        Ok(self)
    }

    #[inline]
    pub fn bind_indirect<I, V>(&mut self, index: I, value: V) -> Result<&mut Self, Error>
    where
        I: BindingIndex,
        V: BindingValue,
    {
        value.bind(self, index.index(self)?)?;

        Ok(self)
    }

    #[inline]
    pub fn bind_null<I>(&mut self, index: I) -> Result<&mut Self, Error>
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

    #[inline]
    pub fn column_count(&self) -> u32 {
        unsafe { crate::ffi::sqlite3_column_count(self.stmt) as u32 }
    }

    #[inline]
    pub fn column_name(&self, index: u32) -> Result<String, Error> {
        // NOTE: 'sqlite3_column_name' always returns
        unsafe {
            let name = crate::ffi::sqlite3_column_name(self.stmt, index as i32);
            if name.is_null() {
                Err(Error::ColumnIndexInvalid(index))
            } else {
                std::ffi::CStr::from_ptr(name)
                    .to_str()
                    .map_err(|err| panic!("failed to convert column name ('{err}')"))
                    .map(|s| s.to_owned())
            }
        }
    }
}

/**
 *
 * Statement methods for executing + resetting.
 *
 **/
impl Statement {
    #[inline]
    pub fn execute(&mut self) -> Result<u32, Error> {
        self.iterate(|_, _| true)
    }

    pub fn iterate<F>(&mut self, mut f: F) -> Result<u32, Error>
    where
        F: FnMut(u32, Row) -> bool,
    {
        let mut count = 0;
        loop {
            count += 1;
            match unsafe { crate::ffi::sqlite3_step(self.stmt) } {
                crate::ffi::SQLITE_ROW => match f(count, Row(self)) {
                    true => continue,
                    false => break,
                },
                crate::ffi::SQLITE_DONE => break,
                err => return Err(Error::StatementExecute(err)),
            }
        }

        Ok(count)
    }

    #[inline]
    pub fn reset(&mut self) -> Result<&mut Self, Error> {
        unsafe { crate::ffi::sqlite3_reset(self.stmt) }
            .into_result()
            .map_err(Error::StatementReset)?;

        // Bindings are cleared separate.
        unsafe { crate::ffi::sqlite3_clear_bindings(self.stmt) }
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

/**
 *
 * Implements a "view" of a result row from a statement execution. The lifetime
 * of this view is valid until the statement moves forward to the next row. One
 * could argue this is a forward-only cursor.
 *
 **/
pub struct Row<'a>(&'a Statement);

impl<'a> Row<'a> {
    pub fn count(&self) -> u32 {
        unsafe { crate::ffi::sqlite3_data_count(self.0.as_ptr()) as u32 }
    }

    pub fn value(&'a self, index: u32) -> Value<'a> {
        Value(self.0, index)
    }
}

/**
 *
 * Implements a "view" of a specific column value within a result row from a
 * statement execution. This view of the value is only valid while the statement
 * object hasn't moved on to the next row of result.
 *
 **/
pub struct Value<'a>(&'a Statement, u32);

impl<'a> Value<'a> {
    pub fn data_type(&self) -> ValueType {
        match unsafe { crate::ffi::sqlite3_column_type(self.0.as_ptr(), self.1 as i32) } {
            crate::ffi::SQLITE_BLOB => ValueType::Blob,
            crate::ffi::SQLITE_INTEGER => ValueType::Integer,
            crate::ffi::SQLITE_NULL => ValueType::Null,
            crate::ffi::SQLITE_TEXT => ValueType::Text,
            crate::ffi::SQLITE_FLOAT => ValueType::Float,
            _ => ValueType::Unknown,
        }
    }

    pub fn as_str(&self) -> &'a str {
        unsafe {
            let val = crate::ffi::sqlite3_column_text(self.0.as_ptr(), self.1 as i32);
            if val.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(val)
                    .to_str()
                    .expect("failed to convert column name ('{err}')")
            }
        }
    }

    pub fn as_i32(&self) -> i32 {
        unsafe { crate::ffi::sqlite3_column_int(self.0.as_ptr(), self.1 as i32) }
    }

    pub fn as_i64(&self) -> i64 {
        unsafe { crate::ffi::sqlite3_column_int64(self.0.as_ptr(), self.1 as i32) }
    }

    pub fn as_f64(&self) -> f64 {
        unsafe { crate::ffi::sqlite3_column_double(self.0.as_ptr(), self.1 as i32) }
    }
}

/**
 *
 * Mapping a SQLite data type to a better representation.
 *
 **/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValueType {
    Blob,
    Float,
    Integer,
    Null,
    Text,
    Unknown,
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

        let mut insert = db
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
    fn column_names() {
        crate::init().expect("failed sqlite initialization");

        let db = Database::in_memory().expect("failed to create in-memory database");
        db.execute(
            r####"
                CREATE TABLE columns (
                    "a1" INT,
                    "a2" TEXT,
                    "a3" BLOB,
                    "a4" REAL
                ) STRICT;
            "####,
        )
        .expect("failed to create test table");

        let stmt = db
            .prepare("SELECT * FROM columns;")
            .expect("failed to create statement");

        assert_eq!(stmt.column_count(), 4);
        assert_eq!(stmt.column_name(0).unwrap(), "a1");
        assert_eq!(stmt.column_name(1).unwrap(), "a2");
        assert_eq!(stmt.column_name(2).unwrap(), "a3");
        assert_eq!(stmt.column_name(3).unwrap(), "a4");
        assert!(stmt.column_name(4).is_err());
    }

    #[test]
    fn column_types_and_values() {
        crate::init().expect("failed sqlite initialization");

        let db = Database::in_memory().expect("failed to create in-memory database");
        db.execute(
            r####"
                CREATE TABLE columns (
                    "a1" INT,
                    "a2" TEXT,
                    "a3" BLOB,
                    "a4" REAL,
                    "a5" TEXT
                ) STRICT;

                INSERT INTO columns (a1, a2, a3, a4, a5) VALUES (1, '1', X'deadbeef', 1.01, NULL);
                INSERT INTO columns (a1, a2, a3, a4, a5) VALUES (2, '2', X'baadf00d', 2.02, NULL);
                INSERT INTO columns (a1, a2, a3, a4, a5) VALUES (3, '3', X'cccccccc', 3.03, NULL);
                INSERT INTO columns (a1, a2, a3, a4, a5) VALUES (4, '4', X'90909090', 4.04, NULL);
            "####,
        )
        .expect("failed to create / populate test table");

        let mut count = 0;
        db.prepare("SELECT * FROM columns;")
            .expect("failed to create statement")
            .iterate(|index, row| {
                assert_eq!(row.count(), 5);
                assert_eq!(row.value(0).data_type(), ValueType::Integer);
                assert_eq!(row.value(0).as_i32(), index as i32);
                assert_eq!(row.value(1).data_type(), ValueType::Text);
                assert_eq!(row.value(1).as_str(), format!("{index}"));
                assert_eq!(row.value(2).data_type(), ValueType::Blob);
                assert_eq!(row.value(3).data_type(), ValueType::Float);
                assert_float_eq(row.value(3).as_f64(), 1.01 * index as f64, 0.001);
                assert_eq!(row.value(4).data_type(), ValueType::Null);

                count += 1;
                true
            })
            .expect("failed to iterate results");

        assert_eq!(count, 4);
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

    fn assert_float_eq(a: f64, b: f64, epsilon: f64) {
        assert!((a - b).abs() < epsilon);
    }
}
