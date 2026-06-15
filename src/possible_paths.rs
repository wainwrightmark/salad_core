//Utility functions for finding the number of possible ways to arrange the letters in a solution

use const_sized_bit_set::prelude::BitSet;

use crate::{
    Solution,
    grid_layout::GridLayout,
    prelude::{GridSet, GridTile},
};

pub fn count_solution_possible_paths<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
    solution: Solution<GRID_SIZE>,
) -> usize {
    let set: GridSet = GridSet::from_iter(solution.into_iter().map(|x| x.0 as u32));
    count_set_possible_paths::<GRID_SIZE, LAYOUT>(set)
}

fn count_set_possible_paths<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
    set: GridSet,
) -> usize {
    let mut total = 0;
    for tile in set.iter_const() {
        let new_set = set.with_removed(tile);
        total += count_set_possible_paths_inner::<GRID_SIZE, LAYOUT>(GridTile(tile as u8), new_set);
    }

    total
}

fn count_set_possible_paths_inner<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
    first_tile: GridTile,
    set: GridSet,
) -> usize {
    if set.is_empty_const() {
        return 1;
    }
    let adjacent_tiles: GridSet = LAYOUT::ADJACENCIES[first_tile.inner_usize()];

    let next_first_tiles = adjacent_tiles.with_intersect(&set);

    let mut total = 0;
    for next_tile in next_first_tiles.iter() {
        let new_set = set.with_removed(next_tile);
        total +=
            count_set_possible_paths_inner::<GRID_SIZE, LAYOUT>(GridTile(next_tile as u8), new_set);
    }

    total
}
