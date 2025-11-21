/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/actions
 *
 * Purpose:
 *   Global application 'actions' used to communicate messages and
 *   instructions back to the application from sub-views.
 */

pub enum Action {
    // Composition 'update' needs nothing else done
    // None,

    // Launch an external hyperlink
    OpenURL(String),
}

impl Action {
    pub fn execute(&self) -> iced::Task<super::Message> {
        match self {
            // Action::None => iced::Task::none(),
            Action::OpenURL(url) => {
                log::info!("Open URL ({url})");
                iced::Task::none()
            }
        }
    }
}
