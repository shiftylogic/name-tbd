/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/state
 *
 * Purpose:
 *   Implementation of the application 'state' data.
 */

use {
    super::{Message, View, constants},
    iced::{Task, Theme},
};

/**
 *
 * Contains the entirety of the running application state and
 * can be (at least partially) serialized for resuming sessions
 * after application terminate / restart.
 *
 **/
pub struct State {
    active_theme: Option<Theme>,
    active_view: Option<View>,
}

/**
 *
 * A collection of functions for consuming the current 'state' from within
 * 'iced' user interfaces. All these helpers are 'read-only' and will not
 * manipulate the state in any way. Therefore, they all take an immutable
 * reference to self.
 *
 **/
impl State {
    pub fn is_view_active(&self, view: View) -> bool {
        match &self.active_view {
            Some(v) => *v == view,
            None => false,
        }
    }

    pub fn root_view(&self) -> Option<&View> {
        self.active_view.as_ref()
    }

    pub fn theme(&self) -> Option<Theme> {
        self.active_theme.clone()
    }

    pub fn title(&self) -> String {
        constants::APP_NAME.to_string()
    }
}

/**
 *
 * A collection of functions for transforming the application state into
 * a new state. Each function *should* perform all modifications necessary
 * to leave the application state in a valid configuration once complete.
 *
 * NOTE: Each function should take a reference to 'mut self' and return no
 *       value (or an error). If the transformation is non-trivial, the
 *       operation can be moved into an async task at the callsite.
 *
 **/
impl State {
    pub fn change_theme(&mut self, theme: Theme) {
        log::trace!("Theme change [{:?} => {:?}]", self.active_theme, theme);
        self.active_theme = Some(theme);
    }

    pub fn change_view(&mut self, view: View) {
        log::trace!("View change [{:?} => {:?}]", self.active_view, view);
        self.active_view = Some(view);
    }
}

/**
 *
 * Perform any work necessary to load an application state including
 * loading configuration data, saved state, settings up db connections,
 * etc.
 *
 * Should return a Task if this work is non-trivial.
 *
 **/
pub fn load() -> (State, Task<Message>) {
    (
        State {
            active_theme: Some(constants::DEFAULT_THEME),
            active_view: None,
        },
        Task::perform(load_all_the_things(), |_| Message::ChangeView(View::Stats)),
    )
}

async fn load_all_the_things() -> u32 {
    use iced::futures::channel;

    let (tx, rx) = channel::oneshot::channel();

    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(2000));
        tx.send(42).expect("facepalm");
    });

    rx.await.expect("bad stuff happened")
}
