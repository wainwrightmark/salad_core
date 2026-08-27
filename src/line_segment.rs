use std::ops::Sub;

use arrayvec::ArrayVec;
use itertools::Itertools;

use crate::prelude::{GridLayout, GridTile, Solution};

pub fn make_line_segments<const GRID_SIZE: usize, LayoutType: GridLayout<GRID_SIZE>> (solution: Solution<GRID_SIZE>) -> ArrayVec<LineSegment, 18> {
    if let Ok(&s) = solution.iter().exactly_one() {
        return ArrayVec::from_iter([LineSegment {
            from: s,
            to: s,
            index: 0,
            color_index: 0,
        }]);
    }

    let mut color_index = 0;
    let mut previous_gradient = None;
    let mut result = ArrayVec::default();
    for (index, (from, to)) in solution.into_iter().tuple_windows().enumerate() {
        let this_gradient = LayoutType::tile_position_u8(to)
            .as_i8vec2()
            .sub(LayoutType::tile_position_u8(from).as_i8vec2());
        if Some(this_gradient) != previous_gradient {
            if !previous_gradient.is_none() {
                color_index += 1;
            }
            previous_gradient = Some(this_gradient);
        }

        result.push(LineSegment {
            from,
            to,
            index,
            color_index,
        });
    }

    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineSegment {
    pub from: GridTile,
    pub to: GridTile,
    pub index: usize,
    pub color_index: usize,
}