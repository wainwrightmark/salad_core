use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIs, EnumIter, EnumString};

use crate::prelude::{Grid, GridLayout, GridSet, WordTrait};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumIs,
    Serialize,
    Deserialize,
    Display,
    EnumString,
    AsRefStr,
    EnumIter
)]
pub enum SafetyRestriction {
    #[default]
    None,
    UnambiguousFirstLetters,
    Paper,
    PaperAndArizona,
}

impl SafetyRestriction {
    pub const fn has_paper_restriction(self) -> bool {
        self.is_paper() || self.is_paper_and_arizona()
    }

    pub fn check_is_safe<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
        self,
        word: &impl WordTrait<GRID_SIZE>,
        grid: Grid<GRID_SIZE>,
    ) -> bool {
        match self {
            SafetyRestriction::None => true,
            SafetyRestriction::UnambiguousFirstLetters => {
                let mut current_set = GridSet::EMPTY;
                for solution in word.find_solutions_with_tiles::<LAYOUT>(grid, GridSet::EMPTY) {
                    if let Some(first_tile) = solution.first() {
                        current_set.insert_const(first_tile.inner_u32());

                        if current_set.len_const() > 1 {
                            return false;
                        }
                    }
                }
                true
            }
            SafetyRestriction::Paper | SafetyRestriction::PaperAndArizona => {
                //Check paper safe
                if word
                    .find_solutions::<LAYOUT>(grid)
                    .map(|s| GridSet::from_iter(s.into_iter().map(|x| x.0 as u32)))
                    .dedup()
                    .exactly_one()
                    .ok()
                    .is_none()
                {
                    return false;
                }

                if self.is_paper_and_arizona() {
                    if !word.is_arizona_safe::<LAYOUT>(&grid) {
                        return false;
                    }
                }

                true
            }
        }
    }
}
