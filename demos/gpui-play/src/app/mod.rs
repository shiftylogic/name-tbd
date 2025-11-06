/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app
 *
 * Purpose:
 *   Implementation of the application initialization logic.
 */

mod assets;
mod components;
mod config;
mod menu;
mod styling;
mod views;

use {
    crate::constants,
    gpui::{Application, TitlebarOptions, WindowOptions},
};

pub fn run() {
    Application::new()
        .with_assets(assets::Assets)
        .run(move |cx| {
            // Load and inject necessary global data (config, styling, etc).
            cx.set_global(config::load());
            cx.set_global(styling::dark());

            // App should quit after the last window is closed.
            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();

            // Add a top-level application menu
            menu::build(cx);

            cx.spawn(async move |cx| {
                cx.open_window(
                    WindowOptions {
                        titlebar: Some(TitlebarOptions {
                            title: Some(constants::APP_NAME.into()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    views::Root::view,
                )
                .expect("failed to spawn the root window");
            })
            .detach();

            // Activate the window so it starts with focus
            cx.activate(true);
        });
}
