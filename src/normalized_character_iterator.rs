use crate::prelude::*;
use strum::EnumIs;

pub struct NormalizedCharacterIterator<'s, 'special> {
    special_characters: &'special SpecialCharacters,
    graphemes: unicode_segmentation::Graphemes<'s>,
}

#[derive(Debug, Clone, EnumIs)]
pub enum NormalizedCharacterResult<'s> {
    Error {
        message: &'static str,
        grapheme: &'s str,
    },
    RegularCharacter {
        character: Character,

        grapheme: &'s str,
    },

    SpecialCharacter {
        character: Character,
        first_grapheme: &'s str,
        additional_graphemes: std::iter::Take<unicode_segmentation::Graphemes<'s>>,
    },

    Blank {
        grapheme: &'s str,
    },
}

impl<'s> NormalizedCharacterResult<'s> {
    pub const fn inner_char(&self) -> Result<Character, &'static str> {
        match self {
            NormalizedCharacterResult::Error { message, .. } => Err(message),
            NormalizedCharacterResult::RegularCharacter { character, .. } => Ok(*character),
            NormalizedCharacterResult::SpecialCharacter { character, .. } => Ok(*character),
            NormalizedCharacterResult::Blank { .. } => Ok(Character::Blank),
        }
    }
}

impl<'s, 'special> NormalizedCharacterIterator<'s, 'special> {
    pub fn new(text: &'s str, special_characters: &'special SpecialCharacters) -> Self {
        let graphemes: unicode_segmentation::Graphemes<'_> =
            unicode_segmentation::UnicodeSegmentation::graphemes(text, true);
        Self {
            special_characters,
            graphemes,
        }
    }
}

impl<'s, 'special> Iterator for NormalizedCharacterIterator<'s, 'special> {
    type Item = NormalizedCharacterResult<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        let grapheme = self.graphemes.next()?;

        if !self.special_characters.is_empty() {
            'special: for (special_character_index, sc) in
                self.special_characters.0.iter().enumerate()
            {
                let mut sc_str = sc.as_str();

                if grapheme.len() < sc_str.len()
                    && sc_str[..grapheme.len()].eq_ignore_ascii_case(grapheme)
                {
                    let mut grapheme_count = 1usize;
                    sc_str = &sc_str[grapheme.len()..];
                    let mut graphemes2 = self.graphemes.clone();
                    if !sc_str.is_empty() {
                        'extra_graphemes: loop {
                            let Some(next_grapheme) = graphemes2.next() else {
                                continue 'special;
                            };
                            if !sc_str[0..next_grapheme.len()].eq_ignore_ascii_case(next_grapheme) {
                                continue 'special;
                            }
                            grapheme_count += 1;
                            sc_str = &sc_str[next_grapheme.len()..];
                            if sc_str.is_empty() {
                                break 'extra_graphemes;
                            }
                        }
                        std::mem::swap(&mut self.graphemes, &mut graphemes2);
                    }

                    let character =
                        Character::try_from_special_index(special_character_index).unwrap();

                    return Some(NormalizedCharacterResult::SpecialCharacter {
                        character,
                        first_grapheme: grapheme,
                        additional_graphemes: graphemes2.take(grapheme_count.saturating_sub(1)),
                    });
                }
            }
        }

        let mut normalized = unicode_normalization::UnicodeNormalization::nfd(grapheme);

        let c = normalized.next()?;
        match Character::try_from(c) {
            Ok(character) => {
                if character.is_blank() {
                    Some(NormalizedCharacterResult::Blank { grapheme })
                } else {
                    Some(NormalizedCharacterResult::RegularCharacter {
                        character,

                        grapheme,
                    })
                }
            }
            Err(message) => Some(NormalizedCharacterResult::Error { message, grapheme }),
        }
    }
}
