use ustr::Ustr;

use crate::{icon_paths::IconPaths, prelude::{Character, SpecialCharacterAppearance, SpecialCharacters}};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileContents {
    Grapheme {
        text: Ustr,
        flip: bool,
        rotate_degrees: u32,
    },
    Path {
        d: Ustr,
        flip: bool,
        rotate_degrees: u32,
    },
}

impl TileContents {
    pub fn get_from_character(
        character: Character,
        special_characters: &SpecialCharacters,
        icon_paths: &IconPaths,
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
                    key,
                    rotate_degrees,
                    flip,
                } => match icon_paths.map.get(key) {
                    Some(d) => {
                        tile_contents = TileContents::Path {
                            d: Ustr::from(&d),
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
                        if !icon_paths.map.is_empty() {
                            log::warn!("Could not get icon '{key}'");
                        }
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
