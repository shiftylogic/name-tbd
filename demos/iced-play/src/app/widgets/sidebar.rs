/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/widgets/sidebar
 *
 * Purpose:
 *   Application or screen sidebar-style / tab-like selector widget.
 */

use {
    crate::app::constants,
    iced::{
        Element, Length,
        alignment::Horizontal,
        border,
        widget::{Column, button, container, space},
    },
};

pub struct Sidebar<'a, Message>(Column<'a, Message>);

impl<'a, Message: Clone + 'a> Sidebar<'a, Message> {
    pub fn with_items(items: impl IntoIterator<Item = impl Into<Element<'a, Message>>>) -> Self {
        Self(
            Column::with_children(items.into_iter().map(|i| i.into()))
                .spacing(constants::SIDEBAR_SPACING),
        )
    }
}

impl<'a, Message: Clone + 'a> From<Sidebar<'a, Message>> for Element<'a, Message> {
    fn from(sidebar: Sidebar<'a, Message>) -> Self {
        Self::new(
            container(sidebar.0)
                .align_x(Horizontal::Center)
                .padding(constants::SIDEBAR_PADDING)
                .height(Length::Fill),
        )
    }
}

pub enum SidebarItem<'a, Message> {
    IconButton(Element<'a, Message>, Message, bool),
    Gap,
}

impl<'a, Message: Clone + 'a> SidebarItem<'a, Message> {
    pub fn icon(
        content: impl Into<Element<'a, Message>>,
        on_press: Message,
        is_active: bool,
    ) -> Self {
        Self::IconButton(content.into(), on_press, is_active)
    }
}

impl<'a, Message: Clone + 'a> From<SidebarItem<'a, Message>> for Element<'a, Message> {
    fn from(item: SidebarItem<'a, Message>) -> Self {
        match item {
            SidebarItem::Gap => space::vertical().into(),
            SidebarItem::IconButton(content, on_press, is_active) => button(content)
                .on_press(on_press)
                .padding(constants::SIDEBAR_BUTTON_PADDING)
                .width(Length::Shrink)
                .style(move |theme, status| {
                    let base = button::Style {
                        border: border::rounded(constants::SIDEBAR_BUTTON_ROUNDING),
                        ..button::background(theme, status)
                    };

                    if is_active {
                        base.with_background(theme.extended_palette().background.weakest.color)
                    } else {
                        base
                    }
                })
                .into(),
        }
    }
}
