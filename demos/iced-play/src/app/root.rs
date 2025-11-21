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
 */

use {
    super::{
        Message, State, View,
        views::splash,
        widgets::{Icon, Sidebar, SidebarItem},
    },
    iced::{
        Element, Length, Theme,
        widget::{column, container, row, rule},
    },
};

pub fn view(state: &State) -> Element<'_, Message> {
    state.root_view().map_or_else(splash::view, |view| {
        row![
            // Application sidebar (navigate around views)
            Sidebar::with_items([
                sidebaritem_from_view(state, View::Stats),
                sidebaritem_from_view(state, View::Clips),
                sidebaritem_from_view(state, View::Tagging),
                SidebarItem::Gap,
                sidebaritem_from_view(state, View::Team),
                sidebaritem_from_view(state, View::Settings),
                sidebaritem_from_view(state, View::About),
            ]),
            // Simple visible separator between sidebar and main view
            rule::vertical(1).style(rule::weak),
            // Render the main view
            container(column![
                view.view(state),
                iced::widget::pick_list(Theme::ALL, state.theme(), Message::ThemeChanged)
                    .width(Length::Fill),
            ])
            .width(Length::Fill),
        ]
        .into()
    })
}

/**
 *
 * Convert a specified View value into a SidebarItem including selecting
 * an appropriate button icon and determining whether the view is the
 * current active one.
 *
 **/
fn sidebaritem_from_view<'a>(state: &State, view: View) -> SidebarItem<'a, Message> {
    SidebarItem::icon(
        match view {
            View::About => Icon::Info,
            View::Clips => Icon::Clips,
            View::Settings => Icon::Settings,
            View::Stats => Icon::Stats,
            View::Tagging => Icon::Tagging,
            View::Team => Icon::Team,
        },
        Message::ChangeView(view.clone()),
        state.is_view_active(view),
    )
}
