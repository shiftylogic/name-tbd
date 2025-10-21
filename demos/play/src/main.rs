/*
 * Copyright (c) 2022-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Hello world!
 *
 * Purpose:
 *   Just for a baseline check-in.
 */

extern crate env_logger;

/*
 * Entry point
 */
fn main() {
    env_logger::init();
    log::info!("Hello world");
}
