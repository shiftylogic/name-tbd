/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/widgets/icon
 *
 * Purpose:
 *   Simple widget wrapper around loading an embedded (in-memory) SVG icon
 *   and converting it into an 'iced' renderable element.
 */

use {
    crate::app::constants,
    iced::{Element, widget::svg},
};

pub enum Icon {
    Clips,
    Info,
    Settings,
    Stats,
    Tagging,
    Team,
}

impl From<Icon> for iced::widget::svg::Handle {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Clips => Self::from_memory(constants::SVG_CLIPS_DATA),
            Icon::Info => Self::from_memory(constants::SVG_INFO_DATA),
            Icon::Settings => Self::from_memory(constants::SVG_SETTINGS_DATA),
            Icon::Stats => Self::from_memory(constants::SVG_STATS_DATA),
            Icon::Tagging => Self::from_memory(constants::SVG_TAGGING_DATA),
            Icon::Team => Self::from_memory(constants::SVG_TEAM_DATA),
        }
    }
}

impl<'a, Message> From<Icon> for Element<'a, Message> {
    fn from(icon: Icon) -> Self {
        Self::new(svg(icon))
    }
}
