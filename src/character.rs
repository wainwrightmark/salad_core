use arrayvec::ArrayVec;

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIs, EnumIter, FromRepr};

use crate::{
    prelude::{Character::Special0, NormalizedCharacterIterator},
    special_characters::SpecialCharacters,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    EnumCount,
    EnumIter,
    EnumIs,
    FromPrimitive,
    FromRepr,
)]
#[repr(u8)]
pub enum Character {
    Blank = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
    I = 9,
    J = 10,
    K = 11,
    L = 12,
    M = 13,
    N = 14,
    O = 15,
    P = 16,
    Q = 17,
    R = 18,
    S = 19,
    T = 20,
    U = 21,
    V = 22,
    W = 23,
    X = 24,
    Y = 25,
    Z = 26,
    Special0 = 27,
    Special1 = 28,
    Special2 = 29,
    Special3 = 30,
    Special4 = 31,
    Special5 = 32,
    Special6 = 33,
    Special7 = 34,

    Special8 = 35,
    Special9 = 36,

    Special10 = 37,
    Special11 = 38,
    Special12 = 39,
    Special13 = 40,
    Special14 = 41,
    Special15 = 42,
    Special16 = 43,

    Special17 = 44,
    Special18 = 45,
    Special19 = 46,

    Special20 = 47,
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl Character {
    pub const ALL_LETTERS: [Self; 26] = [
        Character::A,
        Character::B,
        Character::C,
        Character::D,
        Character::E,
        Character::F,
        Character::G,
        Character::H,
        Character::I,
        Character::J,
        Character::K,
        Character::L,
        Character::M,
        Character::N,
        Character::O,
        Character::P,
        Character::Q,
        Character::R,
        Character::S,
        Character::T,
        Character::U,
        Character::V,
        Character::W,
        Character::X,
        Character::Y,
        Character::Z,
    ];
    pub const fn as_u32(self) -> u32 {
        self as u8 as u32
    }

    pub const fn as_usize(self) -> usize {
        self as u8 as usize
    }

    pub const fn special_index(&self) -> Option<usize> {
        //todo fix

        match (*self as u8).checked_sub(Special0 as u8) {
            Some(x) => Some(x as usize),
            None => None,
        }
    }

    pub const fn try_from_special_index(index: usize) -> Option<Self> {
        if index < 21 {
            Self::from_repr(Special0 as u8 + index as u8)
        } else {
            None
        }
    }

    pub fn to_tile_string<'sc>(&self, special_characters: &'sc SpecialCharacters) -> &'sc str {
        match self {
            Character::Blank => " ",
            Character::A => "A",
            Character::B => "B",
            Character::C => "C",
            Character::D => "D",
            Character::E => "E",
            Character::F => "F",
            Character::G => "G",
            Character::H => "H",
            Character::I => "I",
            Character::J => "J",
            Character::K => "K",
            Character::L => "L",
            Character::M => "M",
            Character::N => "N",
            Character::O => "O",
            Character::P => "P",
            Character::Q => "Q",
            Character::R => "R",
            Character::S => "S",
            Character::T => "T",
            Character::U => "U",
            Character::V => "V",
            Character::W => "W",
            Character::X => "X",
            Character::Y => "Y",
            Character::Z => "Z",

            _ => {
                let Some(special_index) = self.special_index() else {
                    return "?";
                };

                match special_characters.get_from_special_index(special_index) {
                    Some(x) => x.as_str(),
                    None => "?",
                }
            }
        }
    }

    /// Convert to an uppercase character
    /// Blanks map to underscores
    pub fn as_char(&self) -> char {
        match self {
            Character::Blank => '_',
            Character::A => 'A',
            Character::B => 'B',
            Character::C => 'C',
            Character::D => 'D',
            Character::E => 'E',
            Character::F => 'F',
            Character::G => 'G',
            Character::H => 'H',
            Character::I => 'I',
            Character::J => 'J',
            Character::K => 'K',
            Character::L => 'L',
            Character::M => 'M',
            Character::N => 'N',
            Character::O => 'O',
            Character::P => 'P',
            Character::Q => 'Q',
            Character::R => 'R',
            Character::S => 'S',
            Character::T => 'T',
            Character::U => 'U',
            Character::V => 'V',
            Character::W => 'W',
            Character::X => 'X',
            Character::Y => 'Y',
            Character::Z => 'Z',

            Character::Special0 => '0',
            Character::Special1 => '1',
            Character::Special2 => '2',
            Character::Special3 => '3',
            Character::Special4 => '4',
            Character::Special5 => '5',
            Character::Special6 => '6',
            Character::Special7 => '7',
            Character::Special8 => '8',
            Character::Special9 => '9',
            Character::Special10 => '⑩',
            Character::Special11 => '⑪',
            Character::Special12 => '⑫',
            Character::Special13 => '⑬',
            Character::Special14 => '⑭',
            Character::Special15 => '⑮',
            Character::Special16 => '⑯',
            Character::Special17 => '⑰',
            Character::Special18 => '⑱',
            Character::Special19 => '⑲',
            Character::Special20 => '⑳',
        }
    }

    /// Convert to a lowercase character
    /// Blanks map to underscores
    pub fn as_char_lower(&self) -> char {
        match self {
            Character::Blank => '_',
            Character::A => 'a',
            Character::B => 'b',
            Character::C => 'c',
            Character::D => 'd',
            Character::E => 'e',
            Character::F => 'f',
            Character::G => 'g',
            Character::H => 'h',
            Character::I => 'i',
            Character::J => 'j',
            Character::K => 'k',
            Character::L => 'l',
            Character::M => 'm',
            Character::N => 'n',
            Character::O => 'o',
            Character::P => 'p',
            Character::Q => 'q',
            Character::R => 'r',
            Character::S => 's',
            Character::T => 't',
            Character::U => 'u',
            Character::V => 'v',
            Character::W => 'w',
            Character::X => 'x',
            Character::Y => 'y',
            Character::Z => 'z',

            Character::Special0 => '0',
            Character::Special1 => '1',
            Character::Special2 => '2',
            Character::Special3 => '3',
            Character::Special4 => '4',
            Character::Special5 => '5',
            Character::Special6 => '6',
            Character::Special7 => '7',
            Character::Special8 => '8',
            Character::Special9 => '9',
            Character::Special10 => '⑩',
            Character::Special11 => '⑪',
            Character::Special12 => '⑫',
            Character::Special13 => '⑬',
            Character::Special14 => '⑭',
            Character::Special15 => '⑮',
            Character::Special16 => '⑯',
            Character::Special17 => '⑰',
            Character::Special18 => '⑱',
            Character::Special19 => '⑲',
            Character::Special20 => '⑳',
        }
    }
}

pub fn normalize_characters_array<const GRID_SIZE: usize>(
    text: &str,
    special_characters: &SpecialCharacters,
) -> Result<ArrayVec<Character, GRID_SIZE>, &'static str> {
    let mut characters = ArrayVec::<Character, GRID_SIZE>::default();

    let iter: NormalizedCharacterIterator<'_, '_> =
        NormalizedCharacterIterator::new(text, special_characters);

    for result in iter {
        match result {
            crate::prelude::NormalizedCharacterResult::Error(err) => return Err(err),
            crate::prelude::NormalizedCharacterResult::RegularCharacter {
                character: inner,
                ..
            } => {
                characters.try_push(inner).map_err(|_| "Word is too long")?;
            }
            crate::prelude::NormalizedCharacterResult::Blank { .. } => {
                //skip blanks
            }
        }
    }

    Ok(characters)
}

pub fn normalize_characters_vec(
    text: &str,
    special_characters: &SpecialCharacters,
) -> Result<Vec<Character>, &'static str> {
    let mut characters = Vec::<Character>::default();

    let iter: NormalizedCharacterIterator<'_, '_> =
        NormalizedCharacterIterator::new(text, special_characters);

    for result in iter {
        match result {
            crate::prelude::NormalizedCharacterResult::Error(err) => return Err(err),
            crate::prelude::NormalizedCharacterResult::RegularCharacter {
                character: inner,
                ..
            } => {
                characters.push(inner);
            }
            crate::prelude::NormalizedCharacterResult::Blank { .. } => {
                //skip blanks
            }
        }
    }

    Ok(characters)
}

impl TryFrom<char> for Character {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_punctuation() | value.is_ascii_whitespace() {
            return Ok(Character::Blank);
        }

        match value {
            '_' | ' ' => Ok(Character::Blank),
            'a' | 'A' => Ok(Character::A),
            'b' | 'B' => Ok(Character::B),
            'c' | 'C' => Ok(Character::C),
            'd' | 'D' => Ok(Character::D),
            'e' | 'E' => Ok(Character::E),
            'f' | 'F' => Ok(Character::F),
            'g' | 'G' => Ok(Character::G),
            'h' | 'H' => Ok(Character::H),
            'i' | 'I' => Ok(Character::I),
            'j' | 'J' => Ok(Character::J),
            'k' | 'K' => Ok(Character::K),
            'l' | 'L' => Ok(Character::L),
            'm' | 'M' => Ok(Character::M),
            'n' | 'N' => Ok(Character::N),
            'o' | 'O' => Ok(Character::O),
            'p' | 'P' => Ok(Character::P),
            'q' | 'Q' => Ok(Character::Q),
            'r' | 'R' => Ok(Character::R),
            's' | 'S' => Ok(Character::S),
            't' | 'T' => Ok(Character::T),
            'u' | 'U' => Ok(Character::U),
            'v' | 'V' => Ok(Character::V),
            'w' | 'W' => Ok(Character::W),
            'x' | 'X' => Ok(Character::X),
            'y' | 'Y' => Ok(Character::Y),
            'z' | 'Z' => Ok(Character::Z),

            '0' | '⓪' => Ok(Character::Special0),
            '1' | '①' => Ok(Character::Special1),
            '2' | '②' => Ok(Character::Special2),
            '3' | '③' => Ok(Character::Special3),
            '4' | '④' => Ok(Character::Special4),
            '5' | '⑤' => Ok(Character::Special5),
            '6' | '⑥' => Ok(Character::Special6),
            '7' | '⑦' => Ok(Character::Special7),
            '8' | '⑧' => Ok(Character::Special8),
            '9' | '⑨' => Ok(Character::Special9),
            '⑩' => Ok(Character::Special10),
            '⑪' => Ok(Character::Special11),
            '⑫' => Ok(Character::Special12),
            '⑬' => Ok(Character::Special13),
            '⑭' => Ok(Character::Special14),
            '⑮' => Ok(Character::Special15),
            '⑯' => Ok(Character::Special16),
            '⑰' => Ok(Character::Special17),
            '⑱' => Ok(Character::Special18),
            '⑲' => Ok(Character::Special19),
            '⑳' => Ok(Character::Special20),

            _ => Err("Invalid character"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct CharacterMap<T>([T; 48]);

impl<T: Default + Copy> Default for CharacterMap<T> {
    fn default() -> Self {
        Self([T::default(); 48])
    }
}

impl<T> CharacterMap<T> {
    pub fn get(&self, c: Character) -> &T {
        &self.0[c.as_usize()]
    }

    pub fn get_mut(&mut self, c: Character) -> &mut T {
        &mut self.0[c.as_usize()]
    }

    pub fn set(&mut self, c: Character, value: T) {
        self.0[c.as_usize()] = value;
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Character, &T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, t)| (Character::from_repr(i as u8).unwrap(), t))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        character::{normalize_characters_array, normalize_characters_vec},
        prelude::*,
    };
    use itertools::Itertools;
    use test_case::test_case;
    use ustr::Ustr;
    /* spellchecker:disable */
    #[test_case("abcd", "ab cd")]
    #[test_case("dali", "Dalí")]
    #[test_case("walle", "Wall-E")]
    fn test_equal(expected: &str, actual: &str) {
        let expected_len = expected.len();
        let expected =
            normalize_characters_array::<16>(expected, &SpecialCharacters::NONE).unwrap();
        let actual = normalize_characters_array(actual, &SpecialCharacters::NONE).unwrap();

        assert_eq!(actual, expected);
        assert_eq!(actual.len(), expected_len);
    }

    #[test]
    fn test_special_characters() {
        let mut sc = SpecialCharacters::NONE;
        sc.try_push(Ustr::from("ant")).unwrap();

        let char_arr = normalize_characters_vec("antielephant", &sc).unwrap();

        let actual = char_arr.iter().map(|x| x.as_char()).join("");

        assert_eq!("0IELEPH0", actual)
    }
}
