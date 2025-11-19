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

mod screens;

use screens::Landing;

/**
 *
 * Prepare and launch the app.
 *
 **/
pub fn run() {
    iced::application(Landing::new, Landing::update, Landing::view)
        .subscription(Landing::subscription)
        .theme(Landing::theme)
        .run()
        .expect("failed to launch application");
}
