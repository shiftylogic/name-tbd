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

use iced::{
    Length, Task,
    widget::{column, container},
};

pub type Element<'a, Message> = iced::Element<'a, Message, iced::Theme, iced::Renderer>;

pub enum Screen {
    About(about::About),
}

#[derive(Debug)]
pub enum Message {
    About(about::Message),
}

pub fn render(screen: &Screen) -> Element<'_, Message> {
    let content = match screen {
        Screen::About(scr) => scr.view().map(Message::About),
    };

    column![container(content).width(Length::Fill).height(Length::Fill),].into()
}

pub fn update(message: Message, screen: &mut Screen) -> Task<Message> {
    match message {
        Message::About(msg) => {
            let Screen::About(scr) = screen;
            scr.update(msg).map(Message::About)
            // if let Screen::About(scr) = screen {
            //     scr.update(msg).map(Message::About)
            // } else {
            //     return Task::none();
            // }
        }
    }
}

pub mod about {
    use iced::{
        Element, Length, Task,
        alignment::{Horizontal, Vertical},
        widget::{button, column, container, text},
    };

    #[derive(Clone, Debug)]
    pub enum Message {
        OpenRepo,
    }

    pub struct About;

    impl About {
        pub fn new() -> Self {
            About {}
        }

        pub fn update(&mut self, message: Message) -> Task<Message> {
            match message {
                Message::OpenRepo => {
                    log::info!("About::OpenRepo message");
                    Task::none()
                }
            }
        }

        pub fn view(&self) -> Element<'_, Message> {
            container(column![
                text("About"),
                button("Click").on_press(Message::OpenRepo),
            ])
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
    }
}
