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
    super::{State, View, views::about},
    iced::{Event, Subscription, Task, Theme},
};

#[derive(Clone, Debug)]
pub enum Message {
    ChangeView(View),
    About(about::Message),

    Event(Event),
    ThemeChanged(Theme),
}

pub fn subscription(_: &State) -> Subscription<Message> {
    iced::event::listen().map(Message::Event)
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ThemeChanged(theme) => {
            state.change_theme(theme);
            Task::none()
        }

        // Handle any system events
        // TODO: Handle keyboard bindings
        Message::Event(_evt) => Task::none(),

        // Handle a view change
        Message::ChangeView(view) => {
            state.change_view(view);
            Task::none()
        }

        //
        // Messages for specific views
        //
        _ => state.root_view().update(state, message),
    }
}
