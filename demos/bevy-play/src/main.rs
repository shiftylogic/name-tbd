/*
 * Copyright (c) 2022-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Hello world!
 *
 * Purpose:
 *   Just for a baseline check-in.
 */

use bevy::{
    app::{App, Startup, Update},
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query},
    },
    prelude::*,
};

/**
 *
 * Constant values only
 *
 * TODO: Move to a separate file.
 *
 **/
mod constants {
    pub const WINDOW_TITLE: &str = "The Window Title";
    pub const WINDOW_WIDTH: u32 = 1920;
    pub const WINDOW_HEIGHT: u32 = 1080;

    pub const PRESENT_MODE: bevy::window::PresentMode = bevy::window::PresentMode::AutoVsync;
}

/*
 * Entry point
 */
fn main() {
    App::new()
        .add_plugins(core::Plugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

fn hello_world() {
    log::info!("Hello, world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Nate Anderson".into())));
    commands.spawn((Person, Name("Liam Graham".into())));
    commands.spawn((Person, Name("Hugo Dockter".into())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        log::info!(" => hello {}!", name.0);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

/**
 *
 * Custom plugin bundle for our use
 *
 **/
mod core {
    use {
        crate::default,
        bevy::{
            DefaultPlugins,
            app::{PluginGroup, PluginGroupBuilder},
            window::{EnabledButtons, Window, WindowPlugin},
        },
    };

    pub struct Plugins;

    impl PluginGroup for Plugins {
        fn build(self) -> PluginGroupBuilder {
            //
            // TODO: Other changes to default plugins (based on arguments / settings)
            //
            PluginGroupBuilder::start::<Self>().add_group(
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: crate::constants::WINDOW_TITLE.into(),
                        resolution: (
                            crate::constants::WINDOW_WIDTH,
                            crate::constants::WINDOW_HEIGHT,
                        )
                            .into(),
                        present_mode: crate::constants::PRESENT_MODE,
                        enabled_buttons: EnabledButtons {
                            maximize: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
            )
        }
    }
}
