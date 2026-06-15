use strum::EnumIs;
use crate::prelude::*;

pub struct NormalizedCharacterIterator<'s, 'special> {
    special_characters: &'special SpecialCharacters,
    graphemes: unicode_segmentation::Graphemes<'s>,
}

#[derive(Debug, Clone, PartialEq, EnumIs)]
pub enum NormalizedCharacterResult<'s> {
    Error(&'static str),
    RegularCharacter {
        character: Character,
        grapheme_count: usize,
        grapheme: &'s str,
    },
    Blank {
        grapheme: &'s str,
    },
}

impl<'s> NormalizedCharacterResult<'s> {
    pub const fn inner_char(&self) -> Result<Character, &'static str> {
        match self {
            NormalizedCharacterResult::Error(err) => Err(err),
            NormalizedCharacterResult::RegularCharacter {
                character: inner, ..
            } => Ok(*inner),
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
                    let mut grapheme_count = 1;
                    sc_str = &sc_str[grapheme.len()..];
                    if !sc_str.is_empty() {
                        let mut graphemes2 = self.graphemes.clone();
                        'extra_graphemes: loop {
                            let Some(next_grapheme) = graphemes2.next() else {
                                continue 'special;
                            };
                            if !sc_str.starts_with(next_grapheme) {
                                continue 'special;
                            }
                            grapheme_count += 1;
                            sc_str = &sc_str[next_grapheme.len()..];
                            if sc_str.is_empty() {
                                break 'extra_graphemes;
                            }
                        }
                        self.graphemes = graphemes2;
                    }

                    let inner = Character::try_from_special_index(special_character_index).unwrap();

                    return Some(NormalizedCharacterResult::RegularCharacter {
                        character: inner,
                        grapheme_count,
                        grapheme,
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
                        grapheme_count: 1,
                        grapheme,
                    })
                }
            }
            Err(err) => Some(NormalizedCharacterResult::Error(err)),
        }
    }
}
