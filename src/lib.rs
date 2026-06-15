pub mod character;
pub mod designed_level;
pub mod display_word;
pub mod draw_paper;
pub mod grid;
pub mod grid_layout;
pub mod grid_tile;
pub mod letter_counts;
pub mod level_trait;
pub mod normalized_character_iterator;
pub mod possible_paths;
pub mod safety_restriction;
pub mod special_characters;
pub mod svg_hexagon;
pub mod tile_usages;
pub mod word_trait;
pub mod complete_solve;
pub use crate::prelude::*;

pub mod prelude {

    pub use glam::Vec2;
    pub use ustr::Ustr;

    pub use crate::character::Character;
    pub use crate::character::*;
    pub use crate::draw_paper::*;
    pub use crate::letter_counts::*;
    pub use crate::normalized_character_iterator::*;
    pub use crate::special_characters::*;
    pub use arrayvec::ArrayVec;
    use const_sized_bit_set::prelude::BitSet32;

    pub use crate::designed_level::DesignedLevel;
    pub use crate::display_word::*;
    pub use crate::grid::*;
    pub use crate::grid_tile::*;
    pub use crate::level_trait::*;
    pub use crate::possible_paths::*;
    pub use crate::safety_restriction::*;
    pub use crate::word_trait::*;
    pub use crate::grid_layout::*;
    pub use crate::complete_solve::*;

    pub type GridSet = BitSet32;
    pub type CharsArray<const GRID_SIZE: usize> = ArrayVec<Character, GRID_SIZE>;
    pub type Solution<const GRID_SIZE: usize> = ArrayVec<GridTile, GRID_SIZE>;
}
