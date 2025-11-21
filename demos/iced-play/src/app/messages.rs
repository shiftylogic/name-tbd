/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/messages
 *
 * Purpose:
 *   Global application 'messages' used at the "root" view. The messages
 *   at this level are either directed to subviews or handled by global
 *   subsystems (keybindings, etc) to manipulate app state.
 */

use {
    super::views::about,
    iced::{Event, Theme},
};

#[derive(Clone, Debug)]
pub enum Message {
    ChangeView(super::View),
    About(about::Message),

    Event(Event),
    ThemeChanged(Theme),
}
