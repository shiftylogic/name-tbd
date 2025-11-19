/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/screens/about
 *
 * Purpose:
 *   Implementation of the application 'About' screen / overlay.
 */

use iced::{
    Element, Length, Task,
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, text},
};

#[derive(Clone, Debug)]
pub enum Message {
    Other,
}

pub struct About;

impl About {
    pub fn new() -> Self {
        log::trace!("constructing 'about' screen");
        About {}
    }

    pub fn update(&mut self, message: Message) -> Task<super::Message> {
        #[allow(clippy::match_single_binding)]
        match message {
            _ => {
                log::trace!("Message directed to 'about' ({:?})", message);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        //log::trace!("rendering 'about'");
        container(column![
            text("About"),
            button("Close").on_press(Message::Other),
            iced::widget::space().height(50),
            iced::widget::iced(16),
        ])
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
