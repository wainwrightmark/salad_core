use anyhow::bail;
use itertools::Itertools;
use ustr::Ustr;

use crate::{
    GridSet,
    grid_layout::GridLayout,
    level_trait::LevelTrait,
    word_trait::{BasicWordTrait, WordTrait},
};

#[derive(Debug, PartialEq)]
pub struct TileUsages<const GRID_SIZE: usize>(pub [usize; GRID_SIZE]);

impl<const GRID_SIZE: usize> TileUsages<GRID_SIZE> {
    pub fn try_from_level_or_error<LAYOUT: GridLayout<GRID_SIZE>>(
        level: &impl LevelTrait<GRID_SIZE>,
    ) -> Result<Self, anyhow::Error> {
        let mut map: [usize; GRID_SIZE] = [0; GRID_SIZE];

        let mut error_words: Vec<Ustr> = vec![];

        for word in level.words().iter() {
            let tiles: GridSet = match word
                .find_solutions::<LAYOUT>(level.grid())
                .map(|s| GridSet::from_iter(s.into_iter().map(|x| x.0 as u32)))
                .dedup()
                .exactly_one()
                .ok()
            {
                Some(x) => x,
                None => {
                    error_words.push(word.text());
                    continue;
                }
            };

            for t in tiles.iter_const() {
                map[t as usize] += 1;
            }
        }

        if error_words.is_empty() {
            Ok(Self(map))
        } else {
            bail!(
                "Some words have multiple paths: {}",
                error_words.iter().map(|x| x.as_str()).join(", ")
            )
        }
    }

    pub fn try_from_level<LAYOUT: GridLayout<GRID_SIZE>>(
        level: &impl LevelTrait<GRID_SIZE>,
    ) -> Option<Self> {
        let mut map: [usize; GRID_SIZE] = [0; GRID_SIZE];

        for word in level.words().iter() {
            let tiles: GridSet = match word
                .find_solutions::<LAYOUT>(level.grid())
                .map(|s| GridSet::from_iter(s.into_iter().map(|x| x.0 as u32)))
                .dedup()
                .exactly_one()
                .ok()
            {
                Some(x) => x,
                None => {
                    return None;
                }
            };

            for t in tiles.iter_const() {
                map[t as usize] += 1;
            }
        }

        Some(Self(map))
    }
}
