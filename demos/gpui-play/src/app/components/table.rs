/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/components/table
 *
 * Purpose:
 *   Implementation of a data table component using GPUI elements.
 */

use {
    crate::app::styling::Styler,
    gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div},
};

#[derive(IntoElement)]
pub struct Table {
    id: ElementId,
    base: S,
}

impl Table {
    pub fn new() -> Self {
        Self(text.into())
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let styler = cx.global::<Styler>();

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .px(gpui::rems(styler.spacing()))
            .line_height(gpui::rems(styler.line_height()))
            .child(self.0)
    }
}
