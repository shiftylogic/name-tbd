/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * The Playground!
 *
 * Purpose:
 *   Playing around with the interesting GPUI (by Zed) and GPUI Component library (Longbridge).
 */

mod app;
mod constants;

fn main() {
    env_logger::init();
    app::run();
}
