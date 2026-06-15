use crate::{prelude::*, special_characters::SpecialCharactersNormalized};
use const_sized_bit_set::prelude::BitSet;
use finite_state_transducer::{FST, LetterJoiner, State, index::FSTIndex};

//use hashbrown::HashSet;
use std::{fmt::Write, iter::Copied};

pub struct FSTHelper;

impl FSTHelper {
    fn find_words_inner<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
        grid: &Grid<GRID_SIZE>,
        wa: &impl FST<Character>,
        push_result: &mut impl FnMut(RawWord<GRID_SIZE>),
        current_index: FSTIndex,
        new_tile: GridTile,
        used_tiles: &GridSet,
        previous_chars: &CharsArray<GRID_SIZE>,
        special_characters: &SpecialCharactersNormalized,
    ) {
        let character = grid[new_tile];

        let slice = match special_characters.try_get_replacement_chars_slice(character) {
            Some(slice) => slice,
            None => &[character],
        };

        //println!("Current_index {current_index:?}. Character {character}. Slice: {slice:?}",);

        if let Some(next_index) = wa.try_accept_slice(current_index, slice) {
            //println!("Next index {next_index:?}");
            let state = wa.get_state(next_index);
            let mut next_chars = previous_chars.to_owned();
            next_chars.push(character);

            let next_used_tiles = used_tiles.with_inserted(new_tile.inner_u32());

            let adjacent_tiles = LAYOUT::iter_adjacent_tiles(new_tile)
                .filter(|t| !used_tiles.contains_const(t.inner_u32()));

            for tile in adjacent_tiles {
                Self::find_words_inner::<GRID_SIZE, LAYOUT>(
                    grid,
                    wa,
                    push_result,
                    next_index,
                    tile,
                    &next_used_tiles,
                    &next_chars,
                    special_characters,
                );
            }
            if state.can_terminate() {
                push_result(RawWord {
                    characters: next_chars,
                });
            }
        } else {
            //println!("Index not accepted");
        }
    }

    pub fn find_words_inner_using_tile<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
        grid: &Grid<GRID_SIZE>,
        must_use_tile: GridTile,
        wa: &impl FST<Character>,
        results: &mut Vec<RawWord<GRID_SIZE>>,
        current_index: FSTIndex,
        new_tile: GridTile,
        used_tiles: &GridSet,
        previous_chars: &CharsArray<GRID_SIZE>,
    ) {
        let character = grid[new_tile];

        if let Some(next_index) = wa.get_state(current_index).try_accept(&character) {
            let state = wa.get_state(next_index);
            let mut next_chars = previous_chars.to_owned();
            next_chars.push(character);

            let next_used_tiles = used_tiles.with_inserted(new_tile.inner_u32());

            let adjacent_tiles = LAYOUT::iter_adjacent_tiles(new_tile)
                .filter(|t| !used_tiles.contains_const(t.inner_u32()));

            for tile in adjacent_tiles {
                Self::find_words_inner_using_tile::<GRID_SIZE, LAYOUT>(
                    grid,
                    must_use_tile,
                    wa,
                    results,
                    next_index,
                    tile,
                    &next_used_tiles,
                    &next_chars,
                );
            }
            if state.can_terminate() && used_tiles.contains_const(must_use_tile.inner_u32()) {
                results.push(RawWord {
                    characters: next_chars,
                });
            }
        }
    }

    pub fn find_all_words<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
        grid: &Grid<GRID_SIZE>,
        wa: &impl FST<Character>,
        special_characters: &SpecialCharacters,
    ) -> Vec<RawWord<GRID_SIZE>> {
        let mut result: Vec<RawWord<GRID_SIZE>> = vec![];
        let empty_used_tiles = GridSet::EMPTY;
        let special_characters_normalized = SpecialCharactersNormalized::new(special_characters);

        //println!("Special characters normalized: {special_characters_normalized:?}");

        for tile in LAYOUT::iter_tiles() {
            Self::find_words_inner::<GRID_SIZE, LAYOUT>(
                grid,
                wa,
                &mut |mut raw_word| {
                    special_characters_normalized
                        .reverse_convert_characters(&mut raw_word.characters);
                    result.push(raw_word)
                },
                FSTIndex(0),
                tile,
                &empty_used_tiles,
                &CharsArray::new_const(),
                &special_characters_normalized,
            )
        }

        result.sort_by(|a, b| a.characters.cmp(&b.characters));
        result.dedup();

        result
    }

    pub fn count_words<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
        grid: &Grid<GRID_SIZE>,
        wa: &impl FST<Character>,
        special_characters: &SpecialCharacters,
    ) -> usize {
        let mut results: hashbrown::HashSet<RawWord<GRID_SIZE>> = Default::default();
        let empty_used_tiles = GridSet::EMPTY;
        let special_characters_normalized = SpecialCharactersNormalized::new(special_characters);
        for tile in LAYOUT::iter_tiles() {
            Self::find_words_inner::<GRID_SIZE, LAYOUT>(
                grid,
                wa,
                &mut |x| {
                    results.insert(x);
                },
                FSTIndex(0),
                tile,
                &empty_used_tiles,
                &CharsArray::new_const(),
                &special_characters_normalized,
            )
        }

        results.len()
    }

    /// Find all words that use a particular tile
    pub fn find_all_words_using_tile<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
        grid: &Grid<GRID_SIZE>,
        required_tile: GridTile,
        wa: &impl FST<Character>,
    ) -> Vec<RawWord<GRID_SIZE>> {
        let mut result: Vec<RawWord<GRID_SIZE>> = vec![];
        let empty_used_tiles = GridSet::EMPTY;
        for tile in LAYOUT::iter_tiles() {
            Self::find_words_inner_using_tile::<GRID_SIZE, LAYOUT>(
                grid,
                required_tile,
                wa,
                &mut result,
                FSTIndex(0),
                tile,
                &empty_used_tiles,
                &CharsArray::new_const(),
            )
        }

        result.sort_by(|a, b| a.characters.cmp(&b.characters));
        result.dedup();

        result
    }
}

pub struct CharacterJoiner<const GRID_SIZE: usize>;

impl<const GRID_SIZE: usize> LetterJoiner<Character> for CharacterJoiner<GRID_SIZE> {
    type String = RawWord<GRID_SIZE>;

    fn join<'a>(items: impl Iterator<Item = &'a Character>) -> Self::String
    where
        Self: 'a,
    {
        RawWord {
            characters: items.copied().collect(),
        }
    }
}

impl finite_state_transducer::Letter for Character {
    fn try_from_u32(key: u32) -> Option<Self> {
        num_traits::FromPrimitive::from_usize(key as usize)
    }

    fn to_u32(&self) -> u32 {
        *self as u32
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawWord<const GRID_SIZE: usize> {
    pub characters: CharsArray<GRID_SIZE>,
}

impl<const GRID_SIZE: usize> std::fmt::Display for RawWord<GRID_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.characters.iter() {
            f.write_char(x.as_char())?;
        }

        Ok(())
    }
}

impl<'a, const GRID_SIZE: usize> IntoIterator for &'a RawWord<GRID_SIZE> {
    type Item = Character;

    type IntoIter = Copied<<&'a CharsArray<16> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.characters).into_iter().copied()
    }
}

// impl<const GRID_SIZE: usize> BasicWordTrait for RawWord<GRID_SIZE> {
//     fn text(&self) -> ustr::Ustr {
//         crate::chars_array_text_lower(&self.characters)
//     }

//     fn characters_slice(&self) -> &[Character] {
//         &self.characters
//     }
// }

// impl<const GRID_SIZE: usize> WordTrait<GRID_SIZE> for RawWord<GRID_SIZE> {
//     fn characters(&self) -> &CharsArray<GRID_SIZE> {
//         &self.characters
//     }

//     fn quiz_question(&self) -> Option<ustr::Ustr> {
//         None
//     }
// }

impl<const GRID_SIZE: usize> RawWord<GRID_SIZE> {
    pub fn from_string(
        s: &str,
        special_characters: &SpecialCharacters,
    ) -> Result<Self, &'static str> {
        let characters = crate::normalize_characters_array(s, special_characters)?;

        Ok(Self { characters })
    }
}

#[cfg(test)]
mod tests {

    use finite_state_transducer::mutable::MutableFST;
    use itertools::Itertools;

    use super::*;
    #[test]
    pub fn test_word_automata() {
        let special_characters = SpecialCharacters::NONE;
        let mark = RawWord::<16>::from_string("Mark", &special_characters).unwrap();
        let mar = RawWord::<16>::from_string("Mar", &special_characters).unwrap();

        let mut wa = MutableFST::<Character>::default();

        assert!(!wa.contains(&mar));
        assert!(!wa.contains(&mark));

        assert!(wa.add_word(&mar));

        assert!(wa.contains(&mar));
        assert!(!wa.contains(&mark));

        assert!(wa.add_word(&mark));
        assert!(
            !wa.add_word(&mark),
            "Should return false when adding duplicate word"
        );

        assert!(wa.contains(&mar));
        assert!(wa.contains(&mark));
    }

    #[test]
    pub fn test_iter() {
        let special_characters = SpecialCharacters::NONE;
        let mut wa: MutableFST<Character> = MutableFST::default();

        for word in [
            "Earth", "Mars", "Neptune", "Pluto", "Saturn", "Uranus", "Venus", "Some", "Random",
            "Word",
        ] {
            let word = RawWord::<16>::from_string(word, &special_characters).unwrap();
            wa.add_word(&word);
        }

        let wa = wa.compress();

        let v: Vec<RawWord<16>> = wa.iter::<CharacterJoiner<16>>().collect_vec();

        let joined = Itertools::join(&mut v.into_iter(), ", ");

        assert_eq!(
            joined,
            "EARTH, MARS, NEPTUNE, PLUTO, RANDOM, SATURN, SOME, URANUS, VENUS, WORD"
        );
    }

    #[test]
    pub fn test_on_grid() {
        let special_characters = SpecialCharacters::NONE;
        let mut wa = MutableFST::default();

        for word in [
            "Earth", "Mars", "Neptune", "Pluto", "Saturn", "Uranus", "Venus", "Some", "Random",
            "Word",
        ] {
            let word = RawWord::<16>::from_string(word, &special_characters).unwrap();
            wa.add_word(&word);
        }

        //assert_eq!(wa.slab.len(), 52);
        //println!("Uncompressed - {} states", wa.slab.len());
        let wa = wa.compress();
        //println!("Compressed - {} states", wa.slab.len());
        //assert_eq!(wa.slab.len(), 38);

        let grid = try_make_grid::<16>("VENMOUAULTRSHPEN").unwrap();

        let grid_words =
            FSTHelper::find_all_words::<16, Square16Layout>(&grid, &wa, &SpecialCharacters::NONE);

        let found_words = grid_words
            .iter()
            .map(|x| x.characters.iter().map(|x| x.as_char_lower()).join(""))
            .join(", ");

        assert_eq!(
            found_words,
            "earth, mars, neptune, pluto, saturn, uranus, venus", //Should be in alphabetical order
        )
    }

    #[test]
    pub fn test_special_characters() {
        let special_characters = SpecialCharacters::from_iter(["ant"]);
        let mut wa = MutableFST::default();

        for word in ["anteater", "elephant", "peat", "pant", "teat"] {
            let word = RawWord::<16>::from_string(word, &SpecialCharacters::NONE).unwrap();
            wa.add_word(&word);
        }

        //assert_eq!(wa.slab.len(), 52);
        //println!("Uncompressed - {} states", wa.slab.len());
        let wa = wa.compress();
        //println!("Compressed - {} states", wa.slab.len());
        //assert_eq!(wa.slab.len(), 38);

        let grid = try_make_grid::<16>("0EATH__EP__RELE_").unwrap();

        let grid_words =
            FSTHelper::find_all_words::<16, Square16Layout>(&grid, &wa, &special_characters);

        let found_words = grid_words
            .iter()
            .map(|x| x.characters.iter().map(|x| x.as_char_lower()).join(""))
            .join(", ");

        assert_eq!(
            found_words,
            "anteater, elephant", //Should be in alphabetical order
        )
    }
}
