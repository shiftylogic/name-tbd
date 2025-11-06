/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/components
 *
 * Purpose:
 *    Re-exports all the components contained within this module.
 */

mod button;
mod icon;
mod label;

pub(crate) use {button::Button, icon::Icon, label::Label};
