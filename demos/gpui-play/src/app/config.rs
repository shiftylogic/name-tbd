/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/config
 *
 * Purpose:
 *   Implementation of the application-level configuration.
 */

pub struct Config;

/**
 * Necessary to inject into the global state of GPUI application.
 **/
impl gpui::Global for Config {}

impl Config {}

/**
 *
 * Load the configuration data. We will attempt to load from a few different
 * file locations, then fallback to some default values.
 *
 **/
pub fn load() -> Config {
    Config {}
}
