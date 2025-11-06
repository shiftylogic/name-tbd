/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/view/root
 *
 * Purpose:
 *   Implementation of the application root view (top-level user interface).
 */

use {
    crate::app::{
        assets,
        components::{Button /*, Icon*/},
        styling::Styler,
    },
    gpui::{
        App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
    },
};

pub struct Root {}

impl Root {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let styler = cx.global::<Styler>();

        div()
            .flex()
            .flex_col()
            .bg(styler.bg_color())
            .text_color(styler.color())
            .size_full()
            .gap_2()
            .items_center()
            .justify_center()
            .child(
                Button::labelled("btn1", "Click me!")
                    .w(gpui::px(150.0))
                    .px_2(),
            )
            // .child(Button::custom(
            //     "btn2",
            //     div()
            //         .flex()
            //         .flex_row()
            //         .justify_center()
            //         .items_center()
            //         .p_2()
            //         .gap_2()
            //         .child(Icon::from_asset(assets::ICON_SETTINGS))
            //         .child("Hello"),
            // ))
            .child(make_table())
            .child(
                Button::icon("btn-new-2", assets::ICON_SETTINGS), //.padding(gpui::rems(0.25))
            )
    }
}

fn make_table() -> impl IntoElement {
    Button::icon("btn-new", assets::ICON_SETTINGS) //.padding(gpui::rems(0.25))
}
