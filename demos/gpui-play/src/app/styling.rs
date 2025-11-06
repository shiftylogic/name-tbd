/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/styling
 *
 * Purpose:
 *   Implementation of the application-level styling (coloring, spacing, fonts, etc).
 */

use gpui::{FontFallbacks, FontWeight, Global, Hsla, Rems, TextStyle};

/**
 *
 * Some implementation of the the Styler will be loaded into the global state as a means
 * to keep user interface components consistent (color, spacing, etc).
 *
 * NOTE: Currently, styling is a 'feature' that happens at startup and currently no plans
 *       make it changable without a re-compile. This can be converted into a trait and
 *       wrapped as need if we want this functionality later.
 *
 * NOTE: Most sizing is in REM units instead of pixels. If a specific value is *not* in
 *       REM units and it is unclear why, then there should be a short comment inline.
 *
 **/
pub struct Styler;

// Needed to inject into global app state
impl Global for Styler {}

// Coloring
impl Styler {
    pub fn bg_color(&self) -> Hsla {
        gpui::hsla(220.0 / 360.0, 0.24, 0.1, 1.0)
    }
    pub fn color(&self) -> Hsla {
        gpui::hsla(216.0 / 360.0, 0.13, 0.79, 1.0)
    }

    pub fn primary_color(&self) -> Hsla {
        gpui::hsla(200.0 / 360.0, 1.0, 0.5, 1.0)
    }

    pub fn primary_bg_color(&self) -> Hsla {
        gpui::hsla(200.6 / 360.0, 0.989, 0.341, 1.0)
    }

    pub fn primary_border_color(&self) -> Hsla {
        self.primary_bg_color()
    }

    pub fn primary_active_color(&self) -> Hsla {
        gpui::hsla(208.2 / 360.0, 1.0, 0.737, 1.0)
    }

    pub fn primary_active_bg_color(&self) -> Hsla {
        gpui::hsla(200.4 / 360.0, 0.99, 0.378, 1.0)
    }

    pub fn primary_active_border_color(&self) -> Hsla {
        self.primary_active_bg_color()
    }

    pub fn primary_inverse_color(&self) -> Hsla {
        gpui::white()
    }
}

// Borders and outlines
impl Styler {
    pub fn border_radius(&self) -> f32 {
        0.25
    }

    pub fn border_width(&self) -> f32 {
        0.0625
    }

    pub fn outline_width(&self) -> f32 {
        0.125
    }
}

// Spacing
impl Styler {
    pub fn spacing(&self) -> f32 {
        1.0
    }
}

// Text / Font
impl Styler {
    pub fn font_family(&self) -> &'static str {
        "Segoe UI"
    }

    pub fn font_family_fallbacks(&self) -> Vec<String> {
        vec![
            "Roboto".into(),
            "Ubuntu".into(),
            "Cantarell".into(),
            ".SystemUIFont".into(),
        ]
    }

    pub fn font_size(&self) -> f32 {
        1.0
    }

    pub fn font_weight(&self) -> f32 {
        FontWeight::NORMAL.0
    }

    pub fn line_height(&self) -> f32 {
        1.5 // Multiplier of font_size
    }
}

// Component sizing
impl Styler {
    pub fn icon_size(&self) -> f32 {
        1.5
    }

    pub fn icon_padding(&self) -> f32 {
        0.25
    }
}

// Easy conversion of Styler into something that can be injected into
// GPUI elements for styling text
impl From<Styler> for TextStyle {
    fn from(val: Styler) -> Self {
        TextStyle {
            color: val.color(),
            font_family: val.font_family().into(),
            font_fallbacks: Some(FontFallbacks::from_fonts(val.font_family_fallbacks())),
            font_size: Rems(val.font_size()).into(),
            ..TextStyle::default()
        }
    }
}

/**
 *
 * Create and return the "dark" styler (currently the only supported one).
 *
 **/
pub fn dark() -> Styler {
    Styler {}
}

/***
 *
 * Theming
 * -------
 *
 * Default dark colors:
 *   background-color: rgb(19, 22.5, 30.5);
 *   color: #c2c7d0;
 *   text-selection-color: rgba(1, 170, 255, 0.1875);
 *   muted-color: #7b8495;
 *   muted-border-color: #202632;
 *   primary: #01aaff;
 *   primary-background: #0172ad;
 *   primary-border: var(--pico-primary-background);
 *   primary-underline: rgba(1, 170, 255, 0.5);
 *   primary-hover: #79c0ff;
 *   primary-hover-background: #017fc0;
 *   primary-hover-border: var(--pico-primary-hover-background);
 *   primary-hover-underline: var(--pico-primary-hover);
 *   primary-focus: rgba(1, 170, 255, 0.375);
 *   primary-inverse: #fff;
 *   secondary: #969eaf;
 *   secondary-background: #525f7a;
 *   secondary-border: var(--pico-secondary-background);
 *   secondary-underline: rgba(150, 158, 175, 0.5);
 *   secondary-hover: #b3b9c5;
 *   secondary-hover-background: #5d6b89;
 *   secondary-hover-border: var(--pico-secondary-hover-background);
 *   secondary-hover-underline: var(--pico-secondary-hover);
 *   secondary-focus: rgba(144, 158, 190, 0.25);
 *   secondary-inverse: #fff;
 *   contrast: #dfe3eb;
 *   contrast-background: #eff1f4;
 *   contrast-border: var(--pico-contrast-background);
 *   contrast-underline: rgba(223, 227, 235, 0.5);
 *   contrast-hover: #fff;
 *   contrast-hover-background: #fff;
 *   contrast-hover-border: var(--pico-contrast-hover-background);
 *   contrast-hover-underline: var(--pico-contrast-hover);
 *   contrast-focus: rgba(207, 213, 226, 0.25);
 *   contrast-inverse: #000;
 *   box-shadow: 0.0145rem 0.029rem 0.174rem rgba(7, 8.5, 12, 0.01698),
 *               0.0335rem 0.067rem 0.402rem rgba(7, 8.5, 12, 0.024),
 *               0.0625rem 0.125rem 0.75rem rgba(7, 8.5, 12, 0.03),
 *               0.1125rem 0.225rem 1.35rem rgba(7, 8.5, 12, 0.036),
 *               0.2085rem 0.417rem 2.502rem rgba(7, 8.5, 12, 0.04302),
 *               0.5rem 1rem 6rem rgba(7, 8.5, 12, 0.06), 0 0 0 0.0625rem rgba(7, 8.5, 12, 0.015);
 *   h1-color: #f0f1f3;
 *   h2-color: #e0e3e7;
 *   h3-color: #c2c7d0;
 *   h4-color: #b3b9c5;
 *   h5-color: #a4acba;
 *   h6-color: #8891a4;
 *   mark-background-color: #014063;
 *   mark-color: #fff;
 *   ins-color: #62af9a;
 *   del-color: rgb(205.5, 126, 123);
 *   blockquote-border-color: var(--pico-muted-border-color);
 *   blockquote-footer-color: var(--pico-muted-color);
 *   button-box-shadow: 0 0 0 rgba(0, 0, 0, 0);
 *   button-hover-box-shadow: 0 0 0 rgba(0, 0, 0, 0);
 *   table-border-color: var(--pico-muted-border-color);
 *   table-row-stripped-background-color: rgba(111, 120, 135, 0.0375);
 *   code-background-color: rgb(26, 30.5, 40.25);
 *   code-color: #8891a4;
 *   code-kbd-background-color: var(--pico-color);
 *   code-kbd-color: var(--pico-background-color);
 *   form-element-background-color: rgb(28, 33, 43.5);
 *   form-element-selected-background-color: #2a3140;
 *   form-element-border-color: #2a3140;
 *   form-element-color: #e0e3e7;
 *   form-element-placeholder-color: #8891a4;
 *   form-element-active-background-color: rgb(26, 30.5, 40.25);
 *   form-element-active-border-color: var(--pico-primary-border);
 *   form-element-focus-color: var(--pico-primary-border);
 *   form-element-disabled-opacity: 0.5;
 *   form-element-invalid-border-color: rgb(149.5, 74, 80);
 *   form-element-invalid-active-border-color: rgb(183.25, 63.5, 59);
 *   form-element-invalid-focus-color: var(--pico-form-element-invalid-active-border-color);
 *   form-element-valid-border-color: #2a7b6f;
 *   form-element-valid-active-border-color: rgb(22, 137, 105.5);
 *   form-element-valid-focus-color: var(--pico-form-element-valid-active-border-color);
 *   switch-background-color: #333c4e;
 *   switch-checked-background-color: var(--pico-primary-background);
 *   switch-color: #fff;
 *   switch-thumb-box-shadow: 0 0 0 rgba(0, 0, 0, 0);
 *   range-border-color: #202632;
 *   range-active-border-color: #2a3140;
 *   range-thumb-border-color: var(--pico-background-color);
 *   range-thumb-color: var(--pico-secondary-background);
 *   range-thumb-active-color: var(--pico-primary-background);
 *   accordion-border-color: var(--pico-muted-border-color);
 *   accordion-active-summary-color: var(--pico-primary-hover);
 *   accordion-close-summary-color: var(--pico-color);
 *   accordion-open-summary-color: var(--pico-muted-color);
 *   card-background-color: #181c25;
 *   card-border-color: var(--pico-card-background-color);
 *   card-box-shadow: var(--pico-box-shadow);
 *   card-sectioning-background-color: rgb(26, 30.5, 40.25);
 *   dropdown-background-color: #181c25;
 *   dropdown-border-color: #202632;
 *   dropdown-box-shadow: var(--pico-box-shadow);
 *   dropdown-color: var(--pico-color);
 *   dropdown-hover-background-color: #202632;
 *   loading-spinner-opacity: 0.5;
 *   modal-overlay-background-color: rgba(7.5, 8.5, 10, 0.75);
 *   progress-background-color: #202632;
 *   progress-color: var(--pico-primary-background);
 *   tooltip-background-color: var(--pico-contrast-background);
 *   tooltip-color: var(--pico-contrast-inverse);
 *
 * Default light colors:
 *   bg:  #fff;
 *   default: #373c44;
 *   text-selection: rgba(2, 154, 232, 0.25);
 *   muted-color: #646b79;
 *   muted-border-color: rgb(231, 234, 239.5);
 *   primary: #0172ad;
 *   primary-background: #0172ad;
 *   primary-border: #0172ad;
 *   primary-underline: rgba(1, 114, 173, 0.5);
 *   primary-hover: #015887;
 *   primary-hover-background: #02659a;
 *   primary-hover-border: #02659a;
 *   primary-hover-underline: #015887;
 *   primary-focus: rgba(2, 154, 232, 0.5);
 *   primary-inverse: #fff;
 *   secondary: #5d6b89;
 *   secondary-background: #525f7a;
 *   secondary-border: #525f7a;
 *   secondary-underline: rgba(93, 107, 137, 0.5);
 *   secondary-hover: #48536b;
 *   secondary-hover-background: #48536b;
 *   secondary-hover-border: #48536b;
 *   secondary-hover-underline: #48536b;
 *   secondary-focus: rgba(93, 107, 137, 0.25);
 *   secondary-inverse: #fff;
 *   contrast: #181c25;
 *   contrast-background: #181c25;
 *   contrast-border: #181c25;
 *   contrast-underline: rgba(24, 28, 37, 0.5);
 *   contrast-hover: #000;
 *   contrast-hover-background: #000;
 *   contrast-hover-border: #000;
 *   contrast-hover-underline: #48536b;
 *   contrast-focus: rgba(93, 107, 137, 0.25);
 *   contrast-inverse: #fff;
 *   box-shadow: 0.0145rem 0.029rem 0.174rem rgba(129, 145, 181, 0.01698),
 *               0.0335rem 0.067rem 0.402rem rgba(129, 145, 181, 0.024),
 *               0.0625rem 0.125rem 0.75rem rgba(129, 145, 181, 0.03),
 *               0.1125rem 0.225rem 1.35rem rgba(129, 145, 181, 0.036),
 *               0.2085rem 0.417rem 2.502rem rgba(129, 145, 181, 0.04302),
 *               0.5rem 1rem 6rem rgba(129, 145, 181, 0.06),
 *               0 0 0 0.0625rem rgba(129, 145, 181, 0.015);
 *   h1-color: #2d3138;
 *   h2-color: #373c44;
 *   h3-color: #424751;
 *   h4-color: #4d535e;
 *   h5-color: #5c6370;
 *   h6-color: #646b79;
 *   mark-background-color: rgb(252.5, 230.5, 191.5);
 *   mark-color: #0f1114;
 *   ins-color: rgb(28.5, 105.5, 84);
 *   del-color: rgb(136, 56.5, 53);
 *   button-box-shadow: 0 0 0 rgba(0, 0, 0, 0);
 *   button-hover-box-shadow: 0 0 0 rgba(0, 0, 0, 0);
 *   table-border-color: rgb(231, 234, 239.5);
 *   table-row-stripped-background-color: rgba(111, 120, 135, 0.0375);
 *   form-element-background-color: rgb(251, 251.5, 252.25);
 *   form-element-selected-background-color: #dfe3eb;
 *   form-element-border-color: #cfd5e2;
 *   form-element-color: #23262c;
 *   form-element-placeholder-color: var(--pico-muted-color);
 *   form-element-active-background-color: #fff;
 *   form-element-active-border-color: var(--pico-primary-border);
 *   form-element-focus-color: var(--pico-primary-border);
 *   form-element-disabled-opacity: 0.5;
 *   form-element-invalid-border-color: rgb(183.5, 105.5, 106.5);
 *   form-element-invalid-active-border-color: rgb(200.25, 79.25, 72.25);
 *   form-element-invalid-focus-color: var(--pico-form-element-invalid-active-border-color);
 *   form-element-valid-border-color: rgb(76, 154.5, 137.5);
 *   form-element-valid-active-border-color: rgb(39, 152.75, 118.75);
 *   form-element-valid-focus-color: var(--pico-form-element-valid-active-border-color);
 *   switch-background-color: #bfc7d9;
 *   switch-checked-background-color: var(--pico-primary-background);
 *   switch-color: #fff;
 *   switch-thumb-box-shadow: 0 0 0 rgba(0, 0, 0, 0);
 *   range-border-color: #dfe3eb;
 *   range-active-border-color: #bfc7d9;
 *   range-thumb-border-color: var(--pico-background-color);
 *   range-thumb-color: var(--pico-secondary-background);
 *   range-thumb-active-color: var(--pico-primary-background);
 *   card-background-color: var(--pico-background-color);
 *   card-border-color: var(--pico-muted-border-color);
 *   card-box-shadow: var(--pico-box-shadow);
 *   card-sectioning-background-color: rgb(251, 251.5, 252.25);
 *   dropdown-background-color: #fff;
 *   dropdown-border-color: #eff1f4;
 *   dropdown-box-shadow: var(--pico-box-shadow);
 *   dropdown-color: var(--pico-color);
 *   dropdown-hover-background-color: #eff1f4;
 *   modal-overlay-background-color: rgba(232, 234, 237, 0.75);
 *   progress-background-color: #dfe3eb;
 *   progress-color: var(--pico-primary-background);
 *   tooltip-background-color: var(--pico-contrast-background);
 *   tooltip-color: var(--pico-contrast-inverse);
 *
 * Sizing / Styling
 *   text-underline-offset: 0.1rem;
 *   transition: 0.2s ease-in-out;
 *   spacing: 1rem;
 *   typography-spacing-vertical: 1rem;
 *   block-spacing-vertical: var(--pico-spacing);
 *   block-spacing-horizontal: var(--pico-spacing);
 *   grid-column-gap: var(--pico-spacing);
 *   grid-row-gap: var(--pico-spacing);
 *   form-element-spacing-vertical: 0.75rem;
 *   form-element-spacing-horizontal: 1rem;
 *   group-box-shadow: 0 0 0 rgba(0, 0, 0, 0);
 *   group-box-shadow-focus-with-button: 0 0 0 var(--pico-outline-width) var(--pico-primary-focus);
 *   group-box-shadow-focus-with-input: 0 0 0 0.0625rem var(--pico-form-element-border-color);
 *   modal-overlay-backdrop-filter: blur(0.375rem);
 *   nav-element-spacing-vertical: 1rem;
 *   nav-element-spacing-horizontal: 0.5rem;
 *   nav-link-spacing-vertical: 0.5rem;
 *   nav-link-spacing-horizontal: 0.5rem;
 *
 ***/
