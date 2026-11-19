use serde::{Deserialize, Serialize};

use crate::{
    color_scheme::ColorSchemeChoice::{Kale, Lettuce},
    colors::*,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize, Serialize, Eq, PartialOrd, Ord)]
pub enum ColorSchemeChoice {
    #[default]
    Lettuce,
    Kale,
}

impl ColorSchemeChoice {
    pub const fn accent_color_name(&self) -> &'static str {
        match self {
            // ColorSchemeChoice::Light1 => "blue",
            // ColorSchemeChoice::Light2 => "orange",
            // ColorSchemeChoice::Dark1 => "purple",
            Kale => "green",
            Lettuce => "green",
        }
    }

    pub const fn hint_accent_color_name(&self) -> &'static str {
        match self {
            // ColorSchemeChoice::Light1 => "orange",
            // ColorSchemeChoice::Light2 => "purple",
            // ColorSchemeChoice::Dark1 => "orange",
            Kale => "green",
            Lettuce => "green",
        }
    }

    pub const fn color_scheme(&self) -> &'static ColorScheme {
        match self {
            // ColorSchemeChoice::Light1 => &COLOR_SCHEME1A,
            // ColorSchemeChoice::Light2 => &COLOR_SCHEME2,
            // ColorSchemeChoice::Dark1 => &COLOR_SCHEME_DARK,
            Kale => &COLOR_SCHEME_KALE,
            Lettuce => &COLOR_SCHEME_LETTUCE,
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            // ColorSchemeChoice::Light1 => "Light Colours 1",
            // ColorSchemeChoice::Light2 => "Light Colours 2",
            // ColorSchemeChoice::Dark1 => "Dark Colours 1",
            Kale => "Kale",
            Lettuce => "Lettuce",
        }
    }

    pub const fn next(&self) -> Self {
        match self {
            // ColorSchemeChoice::Light1 => Self::Light2,
            // ColorSchemeChoice::Light2 => Self::Dark1,
            // ColorSchemeChoice::Dark1 => Self::Light1,
            ColorSchemeChoice::Kale => Lettuce,
            ColorSchemeChoice::Lettuce => Kale,
        }
    }
}
