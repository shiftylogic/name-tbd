/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/components/icon
 *
 * Purpose:
 *   Implementation of a basic 'icon' component using the GPUI element construct.
 */

use {
    crate::app::styling::Styler,
    gpui::{
        App, Hsla, IntoElement, ParentElement, Rems, RenderOnce, SharedString, Styled, Window, div,
        svg,
    },
};

#[derive(IntoElement)]
pub struct Icon {
    path: SharedString,
    color: Option<Hsla>,
    padding: Option<Rems>,
    size: Option<Rems>,
}

impl Icon {
    pub fn from_asset(path: impl Into<SharedString>) -> Self {
        Self {
            path: path.into(),
            color: None,
            padding: None,
            size: None,
        }
    }

    #[allow(dead_code)]
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    #[allow(dead_code)]
    pub fn padding(mut self, padding: impl Into<Rems>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    #[allow(dead_code)]
    pub fn size(mut self, size: impl Into<Rems>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let styler = cx.global::<Styler>();
        let color = self.color.unwrap_or_else(|| styler.color());
        let padding = self
            .padding
            .unwrap_or_else(|| gpui::rems(styler.icon_padding()));
        let size = self.size.unwrap_or_else(|| gpui::rems(styler.icon_size()));

        div()
            .flex_none()
            .p(padding)
            .child(svg().path(self.path).text_color(color).size(size))
    }
}
