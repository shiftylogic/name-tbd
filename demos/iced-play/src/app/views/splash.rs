/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/views/splash
 *
 * Purpose:
 *   Implementation of a simple 'Splash' loading view.
 *
 *   NOTE: Uses the 'view-helper' pattern, so it has no owned state
 *         and does not send any messages (other than possibly from
 *         messages defined at the callsite).
 *
 */

use iced::{
    Element, Length,
    alignment::{Horizontal, Vertical},
    widget::{container, text},
};

pub fn view<'a, Message: Clone + 'a>() -> Element<'a, Message> {
    container(text("Initializing...").size(48))
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
