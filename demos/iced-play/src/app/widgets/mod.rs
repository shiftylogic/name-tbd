/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/widgets
 *
 * Purpose:
 *   Re-export collection of widget definitions.
 */

mod icon;
mod sidebar;

pub use icon::Icon;
pub use sidebar::{Sidebar, SidebarItem};
