/*
 * Copyright (c) 2025-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:  <undecided>
 * Module: app/assets
 *
 * Purpose:
 *   Implementation of the application asset loading.
 */

/**
 *
 * Macro that expands a list of "key" => "path" or just "key" into
 * machinery used to register an asset source with GPUI for loading
 * assets within components.
 *
 **/
macro_rules! embedded_assets {
    (
        $name:ident
        $( $an:ident $ad:ident $key:literal ),*
        $(,)?
    ) => {
        $(
        pub const $an: &'static str = $key;
        )*

        pub struct Assets;

        impl $name {
            const ASSETS: &'static [&'static str] = &[
                $(
                    $key,
                )*
            ];

            $(
            const $ad: std::borrow::Cow<'static, [u8]> = std::borrow::Cow::Borrowed(
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $key)),
            );
            )*
        }

        impl gpui::AssetSource for $name {
            fn load(
                &self,
                path: &str,
            ) -> anyhow::Result<std::option::Option<std::borrow::Cow<'static, [u8]>>> {
                if path.is_empty() {
                    log::info!("Asset not loaded - path empty");
                    return Ok(None);
                }

                log::info!("Loading asset '{}'", path);

                match path {
                    $(
                    $key => Ok(Some(Self::$ad)),
                    )*
                    _ => Err(anyhow::anyhow!("could not find asset at path '{path}'")),
                }
            }

            fn list(&self, path: &str) -> anyhow::Result<Vec<gpui::SharedString>> {
                Ok(Self::ASSETS
                    .iter()
                    .filter_map(|p| p.starts_with(path).then(|| (*p).into()))
                    .collect())
            }
        }
    };
}

/*+
 *
 * Collection of all the icons we want included / embedded into
 * the built binary.
 *
 **/
embedded_assets!(Assets
    // ICON_CLIPS     LIPS        "icons/clips.svg",
    // ICON_HOME      HOME        "icons/home.svg",
    // ICON_INFO      INFO        "icons/info.svg",
    ICON_SETTINGS  SETTINGS    "icons/settings.svg",
    // ICON_STATS     STATS       "icons/stats.svg",
    // ICON_TAGGING   TAGGING     "icons/tagging.svg",
    // ICON_TEAM      TEAM        "icons/team.svg",
);
