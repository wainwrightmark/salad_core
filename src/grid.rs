use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub fn try_make_grid<const SIZE: usize>(text: &str) -> Option<Grid<SIZE>> {
    let mut arr = [Character::Blank; SIZE];

    let graphemes: unicode_segmentation::Graphemes<'_> =
        unicode_segmentation::UnicodeSegmentation::graphemes(text, true);
    let mut index = 0usize;

    for grapheme in graphemes {
        let normalized = unicode_normalization::UnicodeNormalization::nfd(grapheme);
        for char in normalized {
            let character = Character::try_from(char).ok()?;
            *arr.get_mut(index)? = character;
            index += 1;
        }
    }

    Some(Grid(arr))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Grid<const GRID_SIZE: usize>(#[serde(with = "serde_arrays")] pub [Character; GRID_SIZE]);

impl<const GRID_SIZE: usize> Index<GridTile> for Grid<GRID_SIZE> {
    type Output = Character;

    fn index(&self, index: GridTile) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl<const GRID_SIZE: usize> IndexMut<GridTile> for Grid<GRID_SIZE> {
    fn index_mut(&mut self, index: GridTile) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

impl<const GRID_SIZE: usize> Index<usize> for Grid<GRID_SIZE> {
    type Output = Character;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const GRID_SIZE: usize> IndexMut<usize> for Grid<GRID_SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const GRID_SIZE: usize> Default for Grid<GRID_SIZE> {
    fn default() -> Self {
        Self([Character::Blank; GRID_SIZE])
    }
}

impl<const GRID_SIZE: usize> Grid<GRID_SIZE> {
    pub fn iter(&self) -> impl ExactSizeIterator<Item = Character> + use<GRID_SIZE> {
        self.0.into_iter()
    }

    pub fn enumerate(
        &self,
    ) -> impl ExactSizeIterator<Item = (GridTile, Character)> + use<GRID_SIZE> {
        self.0
            .into_iter()
            .enumerate()
            .map(|(i, c)| (GridTile(i as u8), c))
    }
}
