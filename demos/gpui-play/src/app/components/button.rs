/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/components/button
 *
 * Purpose:
 *   Implementation of a basic 'button' component using the GPUI element construct.
 */

use {
    super::{Icon, Label},
    crate::app::styling::Styler,
    gpui::{
        AnyElement, App, Div, ElementId, InteractiveElement, Interactivity, IntoElement,
        ParentElement, RenderOnce, SharedString, Stateful, StatefulInteractiveElement,
        StyleRefinement, Styled, Window, div,
    },
};

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    base: Stateful<Div>,
    style: StyleRefinement,
    label: Option<AnyElement>,
}

impl Button {
    pub fn labelled(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self::custom(id, Label::new(label))
    }

    pub fn icon(id: impl Into<ElementId>, icon_asset: impl Into<SharedString>) -> Self {
        Self::custom(id, Icon::from_asset(icon_asset))
    }

    pub fn custom(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        let id = id.into();

        Self {
            id: id.clone(),
            base: div().flex_shrink_0().id(id),
            style: StyleRefinement::default(),
            label: Some(content.into_any_element()),
        }
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let focus_handle = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();

        let styler = cx.global::<Styler>();
        let label = self
            .label
            .unwrap_or_else(|| Label::new("Button").into_any_element());

        self.base
            .track_focus(&focus_handle.tab_index(0).tab_stop(true))
            .bg(styler.primary_bg_color())
            .border(gpui::rems(styler.border_width()))
            .border_color(styler.primary_border_color())
            .rounded(gpui::rems(styler.border_radius()))
            .text_color(styler.primary_inverse_color())
            .active(|this| {
                this.bg(styler.primary_active_bg_color())
                    .border_color(styler.primary_active_border_color())
            })
            .refine_style(&self.style)
            .child(label)
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}
