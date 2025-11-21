/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: constants
 *
 * Purpose:
 *    Define and / or fetch all application constants.
 */

/**
 *
 * Concatenating constants at run-time does not work in Rust. So, we have to
 * perform some sorcery.
 *
 * Original author: Jef <jackefransham@gmail.com>
 * Original code: https://github.com/eira-fransham/const-concat/blob/master/src/lib.rs
 * License: Unlicensed
 *
 **/
#[allow(unused_macros)]
macro_rules! const_concat {
    () => {
        ""
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {{
        let bytes: &'static [u8] = unsafe {
            &concat::<
                [u8; $a.len()],
                [u8; $b.len()],
                [u8; $a.len() + $b.len()],
            >($a.as_bytes(), $b.as_bytes())
        };

        unsafe { transmute::<_, &'static str>(bytes) }
    }};
    ($a:expr, $($rest:expr),*) => {{
        const TAIL: &str = const_concat!($($rest),*);
        const_concat!($a, TAIL)
    }};
    ($a:expr, $($rest:expr),*,) => {
        const_concat!($a, $($rest),*)
    };
}

#[allow(unused)]
const unsafe fn transmute<From, To>(from: From) -> To {
    unsafe {
        union Transmute<From, To> {
            from: std::mem::ManuallyDrop<From>,
            to: std::mem::ManuallyDrop<To>,
        }

        std::mem::ManuallyDrop::into_inner(
            Transmute {
                from: std::mem::ManuallyDrop::new(from),
            }
            .to,
        )
    }
}

#[allow(unused)]
const unsafe fn concat<First, Second, Out>(a: &[u8], b: &[u8]) -> Out
where
    First: Copy,
    Second: Copy,
    Out: Copy,
{
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Both<A, B>(A, B);

    unsafe {
        let arr: Both<First, Second> = Both(
            *transmute::<_, *const First>(a.as_ptr()),
            *transmute::<_, *const Second>(b.as_ptr()),
        );

        transmute(arr)
    }
}

/*+
 *
 * All application constants should be below here so they can use the magic macro above.
 *
 **/

pub const APP_NAME: &str = "Iced Playground";
pub const DEFAULT_THEME: iced::Theme = iced::Theme::TokyoNightStorm;

//*******************
// Sidebar styling
//
pub const SIDEBAR_PADDING: f32 = 16.;
pub const SIDEBAR_SPACING: f32 = 16.;
pub const SIDEBAR_BUTTON_ROUNDING: f32 = 5.;
pub const SIDEBAR_BUTTON_PADDING: f32 = 8.;

//*********************
// Embedded Icon Paths
//
pub const SVG_CLIPS_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/clips.svg"));
pub const SVG_INFO_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/info.svg"));
pub const SVG_SETTINGS_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/settings.svg"));
pub const SVG_STATS_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/stats.svg"));
pub const SVG_TAGGING_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/tagging.svg"));
pub const SVG_TEAM_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/team.svg"));
