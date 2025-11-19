/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/screens
 *
 * Purpose:
 *   Re-export 'screen' definitions.
 *   Handle 'view' routing to render the correct active screen.
 */

mod about;

use {
    crate::app::{
        constants,
        widgets::{Icon, Sidebar, SidebarItem},
    },
    iced::{
        Element, Length, Subscription, Task, Theme,
        event::Event,
        widget::{column, container, row, rule},
    },
};

#[derive(PartialEq)]
enum Screen {
    Stats,
    Clips,
    Tagging,
    Team,
    Settings,
    About,
}

#[derive(Clone, Debug)]
pub enum Message {
    ShowAbout,
    ShowStats,
    ShowClips,
    ShowTagging,
    ShowTeam,
    ShowSettings,

    About(about::Message),
    Event(Event),

    ThemeChanged(Theme),
}

/**
 *
 * Application landing screen
 *
 **/
pub struct Landing {
    theme: Option<Theme>,
    screen: Screen,

    about_screen: Option<about::About>,
}

impl Landing {
    pub fn new() -> Self {
        Self {
            theme: Some(Theme::TokyoNightStorm),
            screen: Screen::Stats,

            about_screen: None,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::Event)
    }

    pub fn title(&self) -> String {
        constants::APP_NAME.to_string()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match (&mut self.screen, message) {
            (_, Message::ThemeChanged(theme)) => {
                self.theme = Some(theme);
                Task::none()
            }

            // Handle any system events
            #[allow(clippy::match_single_binding)]
            (_, Message::Event(evt)) => match evt {
                // Event::Keyboard(keyboard::Event::KeyPressed {
                //     key: keyboard::Key::Named(key::Named::Escape),
                //     ..
                // }) => Task::done(Message::DismissOverlay),
                _ => Task::none(),
            },

            //
            // Global messages don't care about overlay or screen
            //
            (_, Message::ShowAbout) => {
                self.about_screen = Some(about::About::new());
                self.screen = Screen::About;
                Task::none()
            }

            (_, Message::ShowStats) => {
                log::info!("Stats clicked");
                self.screen = Screen::Stats;
                Task::none()
            }

            (_, Message::ShowClips) => {
                log::info!("Clips clicked");
                Task::none()
            }

            (_, Message::ShowTagging) => {
                log::info!("Tagging clicked");
                Task::none()
            }

            (_, Message::ShowTeam) => {
                log::info!("Team clicked");
                Task::none()
            }

            (_, Message::ShowSettings) => {
                log::info!("Settings clicked");
                Task::none()
            }

            //
            // Messages for specific screens
            //
            (Screen::About, Message::About(msg)) => self.about_mut().update(msg),
            (Screen::Clips, _) => todo!(),
            (Screen::Settings, _) => todo!(),
            (Screen::Stats, _) => todo!(),
            (Screen::Tagging, _) => todo!(),
            (Screen::Team, _) => todo!(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        row![
            self.sidebar(),
            rule::vertical(1).style(rule::weak),
            self.content(),
        ]
        .into()
    }

    pub fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }
}

/**
 *
 * Helper methods for unwrapping 'screen' structures (as needed).
 *
 **/
impl Landing {
    fn about(&self) -> &about::About {
        self.about_screen
            .as_ref()
            .expect("invalid state: 'about' not constructed")
    }

    fn about_mut(&mut self) -> &mut about::About {
        self.about_screen
            .as_mut()
            .expect("invalid state: 'about' not constructed")
    }
}

/**
 *
 * Build core parts of landing UI
 *
 **/
impl Landing {
    fn content(&self) -> Element<'_, Message> {
        container(column![
            match &self.screen {
                Screen::About => self.about().view().map(Message::About),
                Screen::Clips => not_implemented(),
                Screen::Settings => not_implemented(),
                Screen::Stats => not_implemented(),
                Screen::Tagging => not_implemented(),
                Screen::Team => not_implemented(),
            },
            iced::widget::pick_list(Theme::ALL, self.theme.as_ref(), Message::ThemeChanged)
                .width(Length::Fill),
        ])
        .width(Length::Fill)
        .into()
    }

    fn sidebar(&self) -> Element<'_, Message> {
        let is_active = |scr| self.screen == scr;

        Sidebar::with_items([
            SidebarItem::icon(Icon::Stats, Message::ShowStats, is_active(Screen::Stats)),
            SidebarItem::icon(Icon::Clips, Message::ShowClips, is_active(Screen::Clips)),
            SidebarItem::icon(
                Icon::Tagging,
                Message::ShowTagging,
                is_active(Screen::Tagging),
            ),
            SidebarItem::Gap,
            SidebarItem::icon(Icon::Team, Message::ShowTeam, is_active(Screen::Team)),
            SidebarItem::icon(
                Icon::Settings,
                Message::ShowSettings,
                is_active(Screen::Settings),
            ),
            SidebarItem::icon(Icon::Info, Message::ShowAbout, is_active(Screen::About)),
        ])
        .into()
    }
}

/**
 *
 * TODO: Delete placeholder when no longer needed
 *
 **/
fn not_implemented<'a, Message: Clone + 'a>() -> Element<'a, Message> {
    container(column![
        iced::widget::svg(Icon::Info)
            .width(Length::Fill)
            .height(32)
            .style(|_, _| {
                iced::widget::svg::Style {
                    color: Some(iced::Color::WHITE),
                }
            }),
        iced::widget::text("Not implemented.")
            .width(Length::Fill)
            .size(32)
            .center()
            .color(iced::Color::WHITE),
    ])
    .height(Length::Fill)
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
    .into()
}
