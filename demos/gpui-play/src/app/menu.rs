/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/menu
 *
 * Purpose:
 *   Implementation of the top-level application menu.
 */

use {
    crate::constants,
    gpui::{App, Menu, MenuItem, SystemMenuType},
};

// TODO: This is likely to change to a menu that is platform specific.
pub fn build(cx: &mut App) {
    // Register all necessary menu actions
    cx.on_action(about);
    cx.on_action(quit);

    cx.set_menus(vec![Menu {
        name: constants::APP_NAME.into(),
        items: vec![
            MenuItem::action(constants::MENU_ABOUT, About),
            MenuItem::separator(),
            #[cfg(target_os = "macos")]
            MenuItem::os_submenu(constants::MENU_SERVICES, SystemMenuType::Services),
            #[cfg(target_os = "macos")]
            MenuItem::separator(),
            MenuItem::action(constants::MENU_QUIT, Quit),
        ],
    }]);
}

/*+
 *
 * Menu actions + handlers
 *
 **/

gpui::actions!(menu_actions, [About, Quit]);

fn about(_: &About, _: &mut App) {}

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}
