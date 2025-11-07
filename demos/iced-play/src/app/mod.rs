/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app
 *
 * Purpose:
 *   Implementation of the application initialization logic.
 */

mod screens;

use {
    iced::{Element, Task},
    screens::{Message, Screen},
};

/**
 *
 * Prepare and launch the app.
 *
 **/
pub fn run() {
    iced::application(App::new, App::update, App::view)
        .run()
        .expect("failed to launch application");
}

/**
 *
 * Implements the top-level App structure.
 *
 **/
struct App {
    screen: Screen,
}

impl App {
    fn new() -> Self {
        App {
            screen: Screen::About(screens::about::About::new()),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        screens::update(message, &mut self.screen)
    }

    fn view(&self) -> Element<'_, Message> {
        screens::render(&self.screen)
    }
}
