/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/components/label
 *
 * Purpose:
 *   Implementation of a basic 'label' component using the GPUI element construct.
 */

use {
    crate::app::styling::Styler,
    gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div},
};

#[derive(IntoElement)]
pub struct Label(SharedString);

impl Label {
    pub fn new(text: impl Into<SharedString>) -> Self {
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
