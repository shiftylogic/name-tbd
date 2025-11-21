/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/views
 *
 * Purpose:
 *   Re-export 'view' definitions.
 *   Handle 'view' routing to render the correct active view.
 */

pub mod about;

use {
    super::{Message, State},
    iced::Element,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum View {
    #[default]
    Splash,

    About,
    Clips,
    Settings,
    Stats,
    Tagging,
    Team,
}

/**
 *
 * Implementation details specific to 'iced' UI library (i.e. constructing the user
 * interface + routing / handling of messages).
 *
 **/
impl View {
    pub fn view<'a>(&self, _state: &State) -> iced::Element<'a, Message> {
        match self {
            View::About => about::view().map(Message::About),
            _ => not_implemented(),
        }
    }

    pub fn update(&self, _state: &State, message: Message) -> iced::Task<Message> {
        match (self, message) {
            (View::About, Message::About(msg)) => about::update(msg).execute(),
            _ => iced::Task::none(), // todo!(),
        }
    }
}

/**
 *
 * TODO: Delete placeholder when no longer needed
 *
 **/
fn not_implemented<'a, Message: Clone + 'a>() -> Element<'a, Message> {
    use iced::widget::{column, container};

    container(column![
        iced::widget::svg(super::widgets::Icon::Info)
            .width(iced::Length::Fill)
            .height(32)
            .style(|_, _| {
                iced::widget::svg::Style {
                    color: Some(iced::Color::WHITE),
                }
            }),
        iced::widget::text("Not implemented.")
            .width(iced::Length::Fill)
            .size(32)
            .center()
            .color(iced::Color::WHITE),
    ])
    .height(iced::Length::Fill)
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
    .into()
}
