/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Purpose:
 *   Build script for compiling a static SQLite library directly into the
 *   crate. Used for implementation of an abstraction around a SQLite DB.
 */

fn main() {
    cc::Build::new()
        .define("SQLITE_DQS", "0")
        .define("SQLITE_DEFAULT_MEMSTATUS", "0")
        .define("SQLITE_DEFAULT_WAP_SYNCHRONOUS", "1")
        .define("SQLITE_LIKE_DOESNT_MATCH_BLOBS", "1")
        .define("SQLITE_MAX_EXPR_DEPTH", "0")
        .define("SQLITE_OMIT_DECLTYPE", "1")
        .define("SQLITE_OMIT_DEPRECATED", "1")
        .define("SQLITE_OMIT_PROGRESS_CALLBACK", "1")
        .define("SQLITE_OMIT_SHARED_CACHE", " 1")
        .define("SQLITE_USE_ALLOCA", "1")
        .define("SQLITE_OMIT_AUTOINIT", "1")
        .define("SQLITE_THREADSAFE", "0")
        .define("SQLITE_OMIT_LOAD_EXTENSION", "1")
        .file("src/sqlite/sqlite3.c")
        .compile("sqlite");
}
