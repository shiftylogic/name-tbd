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
mod state;
mod views;
mod widgets;

/**
 * Re-export the application-level pieces (Actions, Messages, Views, etc).
 **/
use {actions::Action, messages::Message, state::State, views::View};

/**
 *
 * Prepare and launch the app.
 *
 **/
pub fn run() {
    iced::application(state::load, messages::update, root::view)
        .title(State::title)
        .subscription(messages::subscription)
        .theme(State::theme)
        .run()
        .expect("failed to launch application");
}
