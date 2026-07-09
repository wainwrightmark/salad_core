use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Write},
    sync::Arc,
};

use crate::prelude::{Character, normalize_characters_array};

pub const SPECIAL_CHARACTER_ICON_DELIMITER: char = ':';
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialCharacter {
    pub text: String,
    pub chars: ArrayVec<Character, 15>,
    pub icon_key: Option<String>,
}

impl Display for SpecialCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(icon_key) = &self.icon_key {
            write!(
                f,
                "{}{SPECIAL_CHARACTER_ICON_DELIMITER}{}",
                self.text, icon_key
            )?;
        } else {
            f.write_str(self.text.as_str())?;
        }

        Ok(())
    }
}

impl SpecialCharacter {
    pub fn try_parse(text: &str) -> Result<Self, &'static str> {
        let sc = if let Some((left, right)) = text.split_once(SPECIAL_CHARACTER_ICON_DELIMITER) {
            SpecialCharacter {
                chars: normalize_characters_array(left, &SpecialCharacters::NONE)?,
                text: left.to_string(),
                icon_key: Some(right.to_string()),
            }
        } else {
            SpecialCharacter {
                chars: normalize_characters_array(text, &SpecialCharacters::NONE)?,
                text: text.to_string(),
                icon_key: None,
            }
        };

        Ok(sc)
    }

    pub fn try_new(text: &str, icon_key: Option<String>) -> Result<Self, &'static str> {
        let chars = normalize_characters_array(text, &SpecialCharacters::NONE)?;
        let text = text.to_string();
        Ok(Self {
            text,
            icon_key,
            chars,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]

pub struct SpecialCharacters(pub ArrayVec<Arc<SpecialCharacter>, 21>);

impl SpecialCharacters {
    pub const NONE: SpecialCharacters = SpecialCharacters(ArrayVec::new_const());

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// impl<'s> FromIterator<&'s str> for SpecialCharacters {
//     fn from_iter<T: IntoIterator<Item = &'s str>>(iter: T) -> Self {
//         let mut sc = SpecialCharacters::NONE;
//         for x in iter {

//             sc.try_push(x).unwrap();
//         }
//         sc
//     }
// }

impl SpecialCharacters {
    pub fn try_from_iter<'a>(
        iter: impl IntoIterator<Item = &'a str>,
    ) -> Result<Self, &'static str> {
        let mut special_characters = SpecialCharacters::NONE;
        for x in iter {
            let sc = SpecialCharacter::try_parse(x)?;
            special_characters.try_push(sc)?;
        }

        Ok(special_characters)
    }

    pub fn try_push(&mut self, special_character: SpecialCharacter) -> Result<(), &'static str> {
        match self.0.try_push(Arc::new(special_character)) {
            Ok(()) => Ok(()),
            Err(_) => Err("Too many special characters"),
        }
    }

    pub fn special_characters_count(&self) -> usize {
        self.0.len()
    }

    pub fn get_from_special_index(&self, index: usize) -> Option<&SpecialCharacter> {
        self.0.get(index).map(|x| x.as_ref())
    }

    pub fn get_from_character(&self, character: Character) -> Option<&SpecialCharacter> {
        let special_index = character.special_index()?;
        self.get_from_special_index(special_index)
    }

    //does this start with the numbers 0 to 9
    pub fn starts_with_numbers(&self) -> bool {
        if self.0.len() < 10 {
            return false;
        }
        for index in 0..10 {
            let replacement = self.0.get(index).unwrap();

            if replacement.text.parse() != Ok(index) {
                return false;
            }
        }
        true
    }

    pub fn try_extract_from_grid_characters(gc: &str) -> Result<(Self, &str), &'static str> {
        if gc.ends_with('}')
            && let Some((prefix, suffix)) = gc.trim_end_matches('}').rsplit_once('{')
        {
            let mut specials = Self::NONE;
            for word in suffix.split(' ') {
                let sc = SpecialCharacter::try_parse(word)?;
                specials.try_push(sc)?;
            }
            return Ok((specials, prefix));
        }
        Ok((Self::NONE, gc))
    }
}

impl SpecialCharacters {
    pub fn try_get_replacement_chars_slice(&self, character: Character) -> Option<&[Character]> {
        match character.special_index() {
            Some(si) => match self.0.get(si) {
                Some(sc) => Some(sc.chars.as_slice()),
                None => None,
            },
            None => None,
        }
    }

    ///Turns terms back into special characters
    pub fn forward_convert_characters<const L: usize>(&self, chars: &mut ArrayVec<Character, L>) {
        if self.0.is_empty() {
            return;
        }
        //log::info!("Forward Convert Characters {}", chars.iter().map(|x|x.as_char_lower()).join(""));

        let mut index = 0;
        while index < chars.len() {
            //let current = chars[index];
            'replacements: for (sc_index, replacement) in self.0.iter().enumerate() {
                if replacement.chars.len() + index > chars.len() {
                    continue 'replacements;
                }
                let mut len = 1usize;
                while len <= replacement.chars.len() {
                    if !replacement.chars.starts_with(&chars[index..(index + len)]) {
                        continue 'replacements;
                    }
                    len += 1;
                }
                chars[index] = Character::try_from_special_index(sc_index).unwrap();

                for _ in 2..len {
                    chars.remove(index + 1);
                }

                break 'replacements;
            }

            index += 1;
        }
    }

    ///Turns special characters back into letter
    pub fn reverse_convert_characters<const L: usize>(&self, chars: &mut ArrayVec<Character, L>) {
        if self.0.is_empty() {
            return;
        }

        let mut index = 0;
        while index < chars.len() {
            let c = chars[index];
            if let Some(new_slice) = self.try_get_replacement_chars_slice(c) {
                chars[index] = new_slice[0];
                let mut new_slice_index = 1;
                index += 1;
                while new_slice_index < new_slice.len() && index < L {
                    chars.insert(index, new_slice[new_slice_index]);
                    new_slice_index += 1;
                    index += 1;
                }
            } else {
                index += 1;
            }
        }
    }
}

impl Display for SpecialCharacters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('{')?;

        let mut first = true;
        for word in self.0.iter() {
            if first {
                first = false;
            } else {
                f.write_char(' ')?;
            }
            word.fmt(f)?;
        }

        f.write_char('}')?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{
        character::normalize_characters_array,
        prelude::{NormalizedCharacterIterator, SpecialCharacters},
    };
    use test_case::test_case;

    #[test_case("", "abcde", "ABCDE")]
    #[test_case("z", "abcde", "ABCDE")]
    #[test_case("abc", "abcde", "0DE")]
    //spellchecker:disable-next-line
    #[test_case("ab", "abacuses abandon", "0ACUSES_0ANDON")]
    #[test_case("HAND", "chandelier", "C0ELIER")]
    #[test_case("hand", "SHANDY", "S0Y")]
    fn test_normalized_character_iterator(special: &str, input: &str, expected: &str) {
        let special_characters = SpecialCharacters::try_from_iter([special]).unwrap();

        let chars: Vec<_> = NormalizedCharacterIterator::new(input, &special_characters)
            .map(|x| x.inner_char().unwrap())
            .collect();

        assert_eq!(expected, chars.iter().map(|x| x.as_char()).join(""));
    }

    #[test]
    fn test_forward_convert_character() {
        let special_characters = SpecialCharacters::try_from_iter(["ant", "any"]).unwrap();

        let mut anteater: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("anteater", &SpecialCharacters::NONE).unwrap();
        let mut elephant: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("elephant", &SpecialCharacters::NONE).unwrap();
        let mut spiral: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("spiral", &SpecialCharacters::NONE).unwrap();

        special_characters.forward_convert_characters(&mut spiral);
        special_characters.forward_convert_characters(&mut anteater);
        special_characters.forward_convert_characters(&mut elephant);

        assert_eq!(
            spiral.into_iter().map(|x| x.as_char_lower()).join(""),
            "spiral"
        );
        assert_eq!(
            anteater.into_iter().map(|x| x.as_char_lower()).join(""),
            "0eater"
        );
        assert_eq!(
            elephant.into_iter().map(|x| x.as_char_lower()).join(""),
            "eleph0"
        );
    }

    #[test]
    fn test_forward_convert_character2() {
        let special_characters =
            SpecialCharacters::try_from_iter(["north", "east", "south", "west"]).unwrap();

        let mut awestricken: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("awestricken", &SpecialCharacters::NONE).unwrap();

        special_characters.forward_convert_characters(&mut awestricken);

        assert_eq!(
            awestricken.into_iter().map(|x| x.as_char_lower()).join(""),
            "a3ricken"
        );
    }

    #[test]
    fn test_reverse_convert_character() {
        let special_characters = SpecialCharacters::try_from_iter(["ant", "any"]).unwrap();

        let mut anteater: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("anteater", &special_characters).unwrap();
        let mut elephant: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("elephant", &special_characters).unwrap();
        let mut spiral: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("spiral", &special_characters).unwrap();

        special_characters.reverse_convert_characters(&mut spiral);
        special_characters.reverse_convert_characters(&mut anteater);
        special_characters.reverse_convert_characters(&mut elephant);

        assert_eq!(
            spiral.into_iter().map(|x| x.as_char_lower()).join(""),
            "spiral"
        );
        assert_eq!(
            anteater.into_iter().map(|x| x.as_char_lower()).join(""),
            "anteater"
        );
        assert_eq!(
            elephant.into_iter().map(|x| x.as_char_lower()).join(""),
            "elephant"
        );
    }
}
