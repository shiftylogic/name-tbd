/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/views
 *
 * Purpose:
 *    Re-exports all the views contained within this module.
 */

//mod dashboard;
mod root;

pub(crate) use {
    // dashboard::Dashboard,
    root::Root,
};
