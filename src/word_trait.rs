use arrayvec::ArrayVec;
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use ustr::Ustr;

use crate::grid_layout::GridLayout;
use crate::prelude::*;
use crate::{Character, Grid, GridSet, GridTile, Solution};

pub struct WordSolutionIter<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>> {
    characters: ArrayVec<Character, GRID_SIZE>,
    grid: Grid<GRID_SIZE>,

    next_first_tile: Option<GridTile>,
    state: Option<WSIState<GRID_SIZE>>,
    phantom: PhantomData<LAYOUT>,
}

impl<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>> WordSolutionIter<GRID_SIZE, LAYOUT> {
    pub fn new(characters: ArrayVec<Character, GRID_SIZE>, grid: Grid<GRID_SIZE>) -> Self {
        let next_first_tile: Option<GridTile> = if let Some(first_char) = characters.first() {
            grid.enumerate()
                .filter(|x| x.1 == *first_char)
                .map(|x| x.0)
                .next()
        } else {
            None
        };

        Self {
            characters,
            grid,
            next_first_tile,
            state: None,
            phantom: PhantomData,
        }
    }
}

struct WSIState<const GRID_SIZE: usize> {
    path: Solution<GRID_SIZE>,
    used_tiles: GridSet,
    indices: ArrayVec<u8, GRID_SIZE>,
    current_index: u8,
    current_tile: GridTile,
    char_to_find: Character,
}

impl<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>> Iterator
    for WordSolutionIter<GRID_SIZE, LAYOUT>
{
    type Item = Solution<GRID_SIZE>;

    fn next(&mut self) -> Option<Self::Item> {
        //TODO more efficient path if word has no duplicate letters

        loop {
            if let Some(state) = &mut self.state {
                if let Some(adjacent_tile) = LAYOUT::try_get_nth_adjacent_tile(
                    state.current_tile,
                    state.current_index as u32,
                ) {
                    state.current_index += 1;

                    if self.grid[adjacent_tile] == state.char_to_find
                        && !state.used_tiles.contains_const(adjacent_tile.0 as u32)
                    {
                        //we need to go deeper
                        state.path.push(state.current_tile);

                        match self.characters.get(state.path.len() + 1) {
                            Some(c) => {
                                state.used_tiles.insert_const(state.current_tile.0 as u32);
                                state.indices.push(state.current_index);
                                state.current_index = 0;
                                state.current_tile = adjacent_tile;
                                state.char_to_find = *c;
                            }
                            None => {
                                //we have found all the characters we need to find
                                let mut final_path = state.path.clone();
                                final_path.push(adjacent_tile);
                                state.path.pop();
                                return Some(final_path);
                            }
                        };
                    }
                } else {
                    //we have run out of options to try - go up a level
                    let Some(ct) = state.path.pop() else {
                        self.state = None;
                        continue;
                    };

                    state.used_tiles.remove_const(ct.0 as u32);
                    state.current_tile = ct;
                    let Some(ci) = state.indices.pop() else {
                        self.state = None;
                        continue;
                    };
                    state.current_index = ci;

                    state.char_to_find = match self.characters.get(state.path.len() + 1) {
                        Some(c) => *c,
                        None => {
                            self.state = None;
                            continue;
                        }
                    };
                }
            } else if let Some(first_tile) = self.next_first_tile {
                self.next_first_tile = {
                    let mut current: GridTile = first_tile;
                    let first_char = self.grid[first_tile];
                    loop {
                        if let Some(n) = current.try_next::<GRID_SIZE>() {
                            current = n;

                            if self.grid[current] == first_char {
                                break Some(current);
                            }
                        } else {
                            break None;
                        }
                    }
                };

                if let Some(char_to_find) = self.characters.get(1) {
                    self.state = Some(WSIState {
                        path: Default::default(),
                        used_tiles: Default::default(),
                        indices: Default::default(),
                        current_index: 0,
                        current_tile: first_tile,
                        char_to_find: *char_to_find,
                    });
                } else {
                    //Word is a single character
                    let mut path: Solution<GRID_SIZE> = Default::default();
                    path.push(first_tile);
                    return Some(path);
                }
            } else {
                return None;
            }
        }
    }
}

pub trait BasicWordTrait: Clone + PartialEq + PartialOrd {
    //todo move some default methods from WordTrait here
    fn text(&self) -> Ustr;
    fn characters_slice(&self) -> &[Character];

    /// The word lengths of the answer. No brackets
    fn hidden_text(&self) -> String {
        let mut hidden_text: String = Default::default();
        let mut stack: usize = 0;

        let unicode_graphemes =
            unicode_segmentation::UnicodeSegmentation::graphemes(self.text().as_str(), true);

        for grapheme in unicode_graphemes {
            let mut normalized = unicode_normalization::UnicodeNormalization::nfd(grapheme);

            let Some(c) = normalized.next() else {
                continue;
            };

            let Ok(character) = Character::try_from(c) else {
                continue;
            };

            if character.is_blank() {
                if let Some(char_to_push) = {
                    if ['-', '‐', '–', '—'].contains(&c) {
                        Some('-')
                    } else if c.is_ascii_whitespace() {
                        Some(',') //use a comma instead of a space, like a crossword clue
                    } else {
                        None
                    }
                } {
                    if stack > 0 {
                        hidden_text += stack.to_string().as_str();
                        stack = 0;
                    }
                    hidden_text.push(char_to_push);
                }

                // otherwise ignore the character in the hidden text
            } else {
                stack += 1;
            }
        }
        if stack > 0 {
            hidden_text += stack.to_string().as_str();
        }

        hidden_text
    }

    fn hinted_text(
        &self,
        hints: NonZeroUsize,
        special_characters: &SpecialCharacters,
        compact: bool,
    ) -> String {
        let underscore_char = if compact { "_" } else { " _" };

        //todo test and check special characters
        let mut result: String = Default::default();
        let mut hints_left = hints.get();

        for ncr in NormalizedCharacterIterator::new(self.text().as_str(), special_characters) {
            match ncr {
                NormalizedCharacterResult::Error { grapheme, .. } => {
                    //println!("Err '{grapheme}'");
                    result.push_str(grapheme);
                }
                NormalizedCharacterResult::RegularCharacter { grapheme, .. } => {
                    match hints_left.checked_sub(1) {
                        Some(new_hints_left) => {
                            //println!("New hints {new_hints_left} '{grapheme}'");
                            result.push_str(grapheme);
                            hints_left = new_hints_left;
                        }
                        None => {
                            //println!("No hints left '{grapheme}'");
                            result.push_str(underscore_char);
                        }
                    }
                }

                NormalizedCharacterResult::SpecialCharacter {
                    character:_,
                    first_grapheme,
                    additional_graphemes,
                    
                } => match hints_left.checked_sub(1) {
                    Some(new_hints_left) => {
                        result.push_str(first_grapheme);
                        for g in additional_graphemes {
                            result.push_str(g);
                        }
                        
                        hints_left = new_hints_left;
                    }
                    None => {
                        result.push_str(underscore_char);
                    }
                },

                NormalizedCharacterResult::Blank { grapheme } => {
                    //println!("Blank '{grapheme}'");
                    result.push_str(grapheme);
                }
            }
        }

        result
    }
}

pub trait WordWithCounts: BasicWordTrait {
    fn letter_counts_value(&self) -> LetterCounts;
}

pub trait WordTrait<const GRID_SIZE: usize>: BasicWordTrait {
    fn characters(&self) -> &ArrayVec<Character, GRID_SIZE>;

    fn quiz_question(&self) -> Option<Ustr>;

    /// Return the letter counts of this word.
    /// Some implementations will store a cached value, others will recalculate it each time
    /// Returns `None` if the letters don't fit in a bag
    fn letter_counts(&self) -> Option<LetterCounts> {
        LetterCounts::try_from_iter(self.characters_slice().iter().cloned())
    }

    fn find_solutions<LAYOUT: GridLayout<GRID_SIZE>>(
        &self,
        grid: Grid<GRID_SIZE>,
    ) -> impl Iterator<Item = Solution<GRID_SIZE>> {
        WordSolutionIter::<GRID_SIZE, LAYOUT>::new(self.characters().clone(), grid)
    }

    fn find_solution<LAYOUT: GridLayout<GRID_SIZE>>(
        &self,
        grid: Grid<GRID_SIZE>,
    ) -> Option<Solution<GRID_SIZE>> {
        WordSolutionIter::<GRID_SIZE, LAYOUT>::new(self.characters().clone(), grid).next()
    }

    fn find_solutions_with_tiles<LAYOUT: GridLayout<GRID_SIZE>>(
        &self,
        mut grid: Grid<GRID_SIZE>,
        unneeded_tiles: GridSet,
    ) -> impl Iterator<Item = Solution<GRID_SIZE>> {
        for tile in unneeded_tiles.iter_const() {
            grid[GridTile(tile as u8)] = Character::Blank;
        }
        Self::find_solutions::<LAYOUT>(self, grid)
    }

    fn is_arizona_safe<LAYOUT: GridLayout<GRID_SIZE>>(&self, grid: &Grid<GRID_SIZE>) -> bool {
        let characters = self.characters_slice();

        for index1 in 0..characters.len().saturating_sub(3) {
            let character_1 = characters[index1];
            for index2 in (index1 + 3)..characters.len() {
                let character_2 = characters[index2];

                if character_1 == character_2 {
                    for solution in self.find_solutions::<LAYOUT>(*grid) {
                        let tile_1 = solution[index1];
                        let tile_before_2 = solution[index2 - 1];
                        if LAYOUT::are_tiles_adjacent(tile_1, tile_before_2) {
                            if let Some(tile_after_2) = solution.get(index2 + 1) {
                                if LAYOUT::are_tiles_adjacent(tile_1, *tile_after_2) {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Write as _, num::NonZeroUsize};

    use const_sized_bit_set::prelude::BitSet;
    use insta::assert_snapshot;
    use itertools::Itertools;

    use crate::{
        DesignedLevel, grid_layout::Square16Layout, level_trait::LevelTrait, prelude::*,
        word_trait::WordTrait,
    };
    pub type Solution4x4 = Solution<16>;

    #[test]
    pub fn test_hidden_text() {
        let special_characters = SpecialCharacters::try_from_iter(["test"]).unwrap();

        let mut output = String::new();
        for word in ["Singleton", "Two Word", "Three-Word", "Attestation"] {
            let raw_word: DisplayWord<_> =
                DisplayWord::<16>::from_string(word, &special_characters).unwrap();
            let hidden = raw_word.hidden_text();
            writeln!(&mut output, "{word}: '{hidden}'").unwrap();
        }

        assert_snapshot!(output)
    }

    #[test]
    pub fn test_hinted_text() {
        let special_characters = SpecialCharacters::try_from_iter(["test"]).unwrap();

        let mut output = String::new();
        for word in ["Singleton", "Two Word", "Three-Word", "Attestation"] {
            let raw_word: DisplayWord<_> =
                DisplayWord::<16>::from_string(word, &special_characters).unwrap();
            let hinted = raw_word.hinted_text(
                NonZeroUsize::try_from(6).unwrap(),
                &special_characters,
                false,
            );
            let hinted_compact = raw_word.hinted_text(
                NonZeroUsize::try_from(6).unwrap(),
                &special_characters,
                true,
            );
            writeln!(&mut output, "{word}: '{hinted}' / '{hinted_compact}'").unwrap();
        }

        assert_snapshot!(output)
    }

    #[test]
    pub fn test_find_solutions() {
        let level = DesignedLevel::<16, Square16Layout>::from_tsv_line(
            //spellchecker:disable-next-line
            "JNAMLUERNPTSEOIH	5	Earth	Jupiter	Mars	Neptune	Pluto",
            true,
        )
        .expect("Could not parse line");
        let grid = level.grid;

        let mut blank_tiles: GridSet = GridSet::default();

        for tile in grid.enumerate().filter(|x| x.1.is_blank()).map(|x| x.0) {
            blank_tiles = blank_tiles.with_inserted(tile.inner_u32());
        }

        let blank_tiles = blank_tiles;

        #[allow(dead_code)]
        #[derive(Debug)]
        struct SolutionResult {
            word: String,
            missing_words: Vec<String>,
            solutions: Vec<Solution4x4>,
        }

        let mut results: Vec<SolutionResult> = vec![];

        for (index, word) in level.words.iter().enumerate() {
            let solutions = word.find_solutions::<Square16Layout>(grid).collect_vec();

            results.push(SolutionResult {
                word: word.text.to_string(),
                missing_words: vec![],
                solutions,
            });

            let ut = level.calculate_unneeded_tiles(blank_tiles, |i| i == index);

            if !ut.is_empty() {
                for word2 in level
                    .words
                    .iter()
                    .enumerate()
                    .filter(|x| x.0 != index)
                    .map(|x| x.1)
                {
                    let solutions = word2
                        .find_solutions_with_tiles::<Square16Layout>(grid, ut)
                        .collect_vec();

                    results.push(SolutionResult {
                        word: word2.text.to_string(),
                        missing_words: vec![word.text.to_string()],
                        solutions,
                    });
                }
            }
        }

        insta::assert_debug_snapshot!(results);
    }
}
