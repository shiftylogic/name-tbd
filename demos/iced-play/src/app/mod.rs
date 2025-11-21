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

mod constants;

mod actions;
mod messages;
mod root;
mod views;
mod widgets;

/**
 * Re-export the application-level pieces (Actions, Messages, Views, etc).
 **/
use {actions::Action, messages::Message, views::View};

/**
 *
 * Prepare and launch the app.
 *
 **/
pub fn run() {
    iced::application(AppState::new, root::update, root::view)
        .title(root::title)
        .subscription(root::subscription)
        .theme(root::theme)
        .run()
        .expect("failed to launch application");
}

struct AppState {
    active_theme: Option<iced::Theme>,
    active_view: View,
}

impl AppState {
    fn new() -> Self {
        Self {
            active_theme: Some(iced::Theme::TokyoNightStorm),
            active_view: View::default(),
        }
    }
}
