/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/views/about
 *
 * Purpose:
 *   Implementation of the application 'About' view.
 *
 *   NOTE: Uses the 'viewable' pattern, so has no owned state.
 */

use {
    crate::app::Action,
    iced::{
        Element, Length,
        alignment::{Horizontal, Vertical},
        widget::{button, column, container, row, text},
    },
};

#[derive(Clone, Debug)]
pub enum Message {
    OpenIced,
}

pub fn update(message: Message) -> Action {
    log::trace!("Message directed to 'about' ({:?})", message);
    match message {
        Message::OpenIced => Action::OpenURL("https://iced.rs".to_string()),
    }
}

pub fn view<'a>() -> Element<'a, Message> {
    //log::trace!("rendering 'about'");
    container(
        column![
            text("About").size(32),
            iced::widget::space().height(50),
            row![
                text("Build using "),
                button(iced::widget::iced(16)).on_press(Message::OpenIced)
            ]
            .align_y(Vertical::Center),
        ]
        .align_x(Horizontal::Center),
    )
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
