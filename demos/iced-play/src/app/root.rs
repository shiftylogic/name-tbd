/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/root
 *
 * Purpose:
 *   Implementation of the 'root' view.
 *
 *   Handles routing the view creattion and message handling to all subviews.
 */

use {
    super::{
        AppState, Message, View, constants,
        widgets::{Icon, Sidebar, SidebarItem},
    },
    iced::{
        Element, Length, Subscription, Task, Theme,
        widget::{column, container, row, rule},
    },
};

pub fn subscription(_: &AppState) -> Subscription<super::Message> {
    iced::event::listen().map(super::Message::Event)
}

pub fn title(_: &AppState) -> String {
    constants::APP_NAME.to_string()
}

pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::ThemeChanged(theme) => {
            state.active_theme = Some(theme);
            Task::none()
        }

        // Handle any system events
        // TODO: Handle keyboard bindings
        Message::Event(_evt) => Task::none(),

        // Handle a view change
        Message::ChangeView(view) => {
            log::info!(
                "View change requested {:?} => {:?}",
                state.active_view,
                view
            );
            state.active_view = view;
            Task::none()
        }

        //
        // Messages for specific views
        //
        _ => state.active_view.update(state, message),
    }
}

pub fn view(state: &AppState) -> Element<'_, Message> {
    let is_active = |scr| state.active_view == scr;

    row![
        // Application sidebar (navigate around views)
        Sidebar::with_items([
            SidebarItem::icon(
                Icon::Stats,
                Message::ChangeView(View::Stats),
                is_active(View::Stats)
            ),
            SidebarItem::icon(
                Icon::Clips,
                Message::ChangeView(View::Clips),
                is_active(View::Clips)
            ),
            SidebarItem::icon(
                Icon::Tagging,
                Message::ChangeView(View::Tagging),
                is_active(View::Tagging),
            ),
            SidebarItem::Gap,
            SidebarItem::icon(
                Icon::Team,
                Message::ChangeView(View::Team),
                is_active(View::Team)
            ),
            SidebarItem::icon(
                Icon::Settings,
                Message::ChangeView(View::Settings),
                is_active(View::Settings),
            ),
            SidebarItem::icon(
                Icon::Info,
                Message::ChangeView(View::About),
                is_active(View::About)
            ),
        ]),
        // Simple visible separator between sidebar and main view
        rule::vertical(1).style(rule::weak),
        // Render the main view
        container(column![
            state.active_view.view(state),
            iced::widget::pick_list(
                Theme::ALL,
                state.active_theme.as_ref(),
                Message::ThemeChanged
            )
            .width(Length::Fill),
        ])
        .width(Length::Fill),
    ]
    .into()
}

pub fn theme(state: &AppState) -> Option<Theme> {
    state.active_theme.clone()
}
