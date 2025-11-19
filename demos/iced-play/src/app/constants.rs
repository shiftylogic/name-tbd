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
