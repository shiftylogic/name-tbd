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

use iced::{
    Color, Length, Subscription, Task, Theme,
    alignment::{Horizontal, Vertical},
    border,
    event::Event,
    widget::{button, column, container, row, rule, space, svg},
};

pub type Element<'a, Message> = iced::Element<'a, Message, iced::Theme, iced::Renderer>;

enum Screen {
    About(about::About),
    Home,
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
}

/**
 *
 * Application landing screen
 *
 **/
pub struct Landing {
    theme: Option<Theme>,
    screen: Screen,
}

impl Landing {
    pub fn new() -> Self {
        Self {
            theme: Some(Theme::Oxocarbon),
            screen: Screen::Home,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(Message::Event)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match (&mut self.screen, message) {
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
                self.screen = Screen::About(about::About::new());
                Task::none()
            }

            (_, Message::ShowStats) => {
                log::info!("Stats clicked");
                self.screen = Screen::Home;
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
            (Screen::About(abt), Message::About(msg)) => abt.update(msg),
            (Screen::Home, _) => todo!(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match &self.screen {
            Screen::Home => container(column![iced::widget::text("Home")])
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
            Screen::About(abt) => abt.view().map(Message::About),
        };

        let sidebar = container(
            column![
                button(svg(Icon::Stats))
                    .width(32)
                    .height(32)
                    .padding(8)
                    .style(button::secondary)
                    .on_press(Message::ShowStats),
                button(svg(Icon::Clips))
                    .width(32)
                    .height(32)
                    .padding(8)
                    .style(button::secondary)
                    .on_press(Message::ShowClips),
                button(svg(Icon::Tagging))
                    .width(32)
                    .height(32)
                    .padding(8)
                    .style(button::secondary)
                    .on_press(Message::ShowTagging),
                space::vertical(),
                button(svg(Icon::Team))
                    .width(32)
                    .height(32)
                    .padding(8)
                    .style(button::secondary)
                    .on_press(Message::ShowTeam),
                button(svg(Icon::Settings))
                    .width(32)
                    .height(32)
                    .padding(8)
                    .style(button::secondary)
                    .on_press(Message::ShowSettings),
                button(svg(Icon::Info))
                    .width(32)
                    .height(32)
                    .padding(8)
                    .style(button::secondary)
                    .on_press(Message::ShowAbout),
            ]
            .spacing(8),
        )
        .align_x(Horizontal::Center)
        .padding(16)
        .height(Length::Fill);

        row![
            sidebar,
            rule::vertical(2),
            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_| {
                    container::Style::default()
                        .background(Color::BLACK.scale_alpha(0.7))
                        .border(border::rounded(16))
                }),
        ]
        .into()
    }

    pub fn theme(&self) -> Option<Theme> {
        self.theme.clone()
    }
}

enum Icon {
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
            Icon::Clips => Self::from_memory(include_bytes!("../../../icons/clips.svg")),
            Icon::Info => Self::from_memory(include_bytes!("../../../icons/info.svg")),
            Icon::Settings => Self::from_memory(include_bytes!("../../../icons/settings.svg")),
            Icon::Stats => Self::from_memory(include_bytes!("../../../icons/stats.svg")),
            Icon::Tagging => Self::from_memory(include_bytes!("../../../icons/tagging.svg")),
            Icon::Team => Self::from_memory(include_bytes!("../../../icons/team.svg")),
        }
    }
}
