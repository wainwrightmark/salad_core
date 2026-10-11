use ustr::Ustr;

use crate::{
    icon_paths::IconPathData,
    prelude::{Character, SpecialCharacterAppearance, SpecialCharacters},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileContents {
    Grapheme {
        text: Ustr,
        flip: bool,
        rotate_degrees: u32,
    },
    Icon {
        unicode_char: char,
        flip: bool,
        rotate_degrees: u32,
    },
}

impl TileContents {
    pub fn get_from_character(
        character: Character,
        special_characters: &SpecialCharacters,
        icon_paths: &IconPathData,
    ) -> TileContents {
        let tile_contents: TileContents;
        if let Some(special_index) = character.special_index()
            && let Some(special_character) =
                special_characters.get_from_special_index(special_index)
        {
            match &special_character.appearance {
                SpecialCharacterAppearance::None => {
                    tile_contents = TileContents::Grapheme {
                        text: Ustr::from(character.to_tile_string(&special_characters)),
                        flip: false,
                        rotate_degrees: 0,
                    };
                }
                SpecialCharacterAppearance::Icon {
                    key_lower,
                    rotate_degrees,
                    flip,
                } => match icon_paths.get_icon_character(&key_lower) {
                    Some(unicode_char) => {
                        tile_contents = TileContents::Icon {
                            unicode_char,
                            flip: *flip,
                            rotate_degrees: *rotate_degrees,
                        };
                    }
                    None => {
                        tile_contents = TileContents::Grapheme {
                            text: Ustr::from(character.to_tile_string(&special_characters)),
                            flip: false,
                            rotate_degrees: 0,
                        };
                        log::warn!(
                            "Could not get icon '{key_lower}' from {} icon paths",
                            icon_paths.len()
                        );
                    }
                },
                SpecialCharacterAppearance::Text {
                    text,
                    rotate_degrees,
                    flip,
                } => {
                    tile_contents = TileContents::Grapheme {
                        text: Ustr::from(&text),
                        flip: *flip,
                        rotate_degrees: *rotate_degrees,
                    };
                }
            }
        } else {
            tile_contents = TileContents::Grapheme {
                text: Ustr::from(character.to_tile_string(&special_characters)),
                flip: false,
                rotate_degrees: 0,
            };
        }

        tile_contents
    }
}
