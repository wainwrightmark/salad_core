use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use ustr::Ustr;

use crate::{character::normalize_characters_array, prelude::Character};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]

pub struct SpecialCharacters(pub ArrayVec<Ustr, 21>);

impl SpecialCharacters {
    pub const NONE: SpecialCharacters = SpecialCharacters(ArrayVec::new_const());

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'s> FromIterator<&'s str> for SpecialCharacters {
    fn from_iter<T: IntoIterator<Item = &'s str>>(iter: T) -> Self {
        let mut sc = SpecialCharacters::NONE;
        for x in iter {
            let u = Ustr::from_str(x).unwrap();
            sc.try_push(u).unwrap();
        }
        sc
    }
}

impl SpecialCharacters {
    pub fn try_push(&mut self, sc: Ustr) -> Result<(), &'static str> {
        match self.0.try_push(sc) {
            Ok(()) => Ok(()),
            Err(_) => Err("Too many special characters"),
        }
    }

    pub fn special_characters_count(&self) -> usize {
        self.0.len()
    }

    pub fn get_from_special_index(&self, index: usize) -> Option<Ustr> {
        self.0.get(index).copied()
    }

    pub fn get_from_character(&self, character: Character) -> Option<Ustr> {
        let special_index = character.special_index()?;
        self.0.get(special_index).copied()
    }

    //does this start with the numbers 0 to 9
    pub fn starts_with_numbers(&self) -> bool {
        if self.0.len() < 10 {
            return false;
        }
        for index in 0..10 {
            let replacement = self.0.get(index).unwrap();

            if replacement.parse() != Ok(index) {
                return false;
            }
        }
        true
    }

    pub fn extract_from_grid_characters(gc: &str) -> (Self, &str) {
        if gc.ends_with('}')
            && let Some((prefix, suffix)) = gc.trim_end_matches('}').rsplit_once('{')
        {
            let mut specials = Self::NONE;
            for word in suffix.split(' ') {
                let _ = specials.try_push(Ustr::from(word));
            }
            return (specials, prefix);
        }
        (Self::NONE, gc)
    }

    pub fn format_for_url(&self) -> String {
        let mut s = String::new();
        s.push('{');
        let mut first = true;
        for word in self.0.iter() {
            if first {
                first = false;
            } else {
                s.push(' ');
            }
            s.push_str(word.as_str());
        }

        s.push('}');
        s
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpecialCharactersNormalized(pub Vec<Option<ArrayVec<Character, 15>>>);

impl SpecialCharactersNormalized {
    pub fn new(sc: &SpecialCharacters) -> Self {
        let mut v = vec![];
        for regular_text in sc.0.iter() {
            v.push(normalize_characters_array(regular_text, &SpecialCharacters::NONE).ok());
        }

        Self(v)
    }

    pub fn try_get_replacement_chars_slice(&self, character: Character) -> Option<&[Character]> {
        match character.special_index() {
            Some(si) => match self.0.get(si) {
                Some(Some(av)) => Some(av.as_slice()),
                Some(None) | None => None,
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
                if let Some(replacement) = replacement {
                    if replacement.len() + index > chars.len() {
                        continue 'replacements;
                    }
                    let mut len = 1usize;
                    while len <= replacement.len() {
                        if !replacement.starts_with(&chars[index..(index + len)]) {
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

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{
        character::normalize_characters_array,
        prelude::{NormalizedCharacterIterator, SpecialCharacters},
        special_characters::SpecialCharactersNormalized,
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
        let special_characters = SpecialCharacters::from_iter([special]);

        let chars: Vec<_> = NormalizedCharacterIterator::new(input, &special_characters)
            .map(|x| x.inner_char().unwrap())
            .collect();

        assert_eq!(expected, chars.iter().map(|x| x.as_char()).join(""));
    }

    #[test]
    fn test_forward_convert_character() {
        let special_characters = SpecialCharacters::from_iter(["ant", "any"]);

        let nsc = SpecialCharactersNormalized::new(&special_characters);

        let mut anteater: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("anteater", &SpecialCharacters::NONE).unwrap();
        let mut elephant: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("elephant", &SpecialCharacters::NONE).unwrap();
        let mut spiral: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("spiral", &SpecialCharacters::NONE).unwrap();

        nsc.forward_convert_characters(&mut spiral);
        nsc.forward_convert_characters(&mut anteater);
        nsc.forward_convert_characters(&mut elephant);

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
        let special_characters = SpecialCharacters::from_iter(["north", "east", "south", "west"]);

        let nsc = SpecialCharactersNormalized::new(&special_characters);

        let mut awestricken: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("awestricken", &SpecialCharacters::NONE).unwrap();

        nsc.forward_convert_characters(&mut awestricken);

        assert_eq!(
            awestricken.into_iter().map(|x| x.as_char_lower()).join(""),
            "a3ricken"
        );
    }

    #[test]
    fn test_reverse_convert_character() {
        let special_characters = SpecialCharacters::from_iter(["ant", "any"]);

        let nsc = SpecialCharactersNormalized::new(&special_characters);

        let mut anteater: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("anteater", &special_characters).unwrap();
        let mut elephant: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("elephant", &special_characters).unwrap();
        let mut spiral: arrayvec::ArrayVec<crate::prelude::Character, 16> =
            normalize_characters_array("spiral", &special_characters).unwrap();

        nsc.reverse_convert_characters(&mut spiral);
        nsc.reverse_convert_characters(&mut anteater);
        nsc.reverse_convert_characters(&mut elephant);

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
