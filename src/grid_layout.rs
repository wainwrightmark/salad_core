use const_sized_bit_set::prelude::{BitSet, BitSet32};
use glam::{U8Vec2, Vec2};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::iter::FusedIterator;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GridLayoutType {
    Square16,
    Hexagon19Fat,
    Hexagon19Thin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TilePositioning{
    Corner,
    Edge,
    Center
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArrowDirection{
    Up, Down, Left, Right
}

pub const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

pub trait GridLayout<const GRID_SIZE: usize>:
    Clone + PartialEq + Send + Sync + Sized + 'static
{
    const ALL_NODES: GridSet = GridSet::from_first_n_const(GRID_SIZE as u32);
    const ADJACENCIES: [GridSet; GRID_SIZE];

    const NAME: &'static str;

    const GRID_LAYOUT_TYPE: GridLayoutType;

    const TILE_SHAPE: TileShape;

    const GAME_URL: &'static str;

    const CONSTRAINT_COUNT_TO_ALLOWED_TILES: [GridSet; 9] = {
        let mut av = [GridSet::EMPTY; 9];
        av[0] = Self::ALL_NODES;

        let mut constraints = 1;
        'c: loop {
            let mut allowed_nodes = GridSet::EMPTY;

            let mut index = 0;
            while index < GRID_SIZE {
                if Self::ADJACENCIES[index].count_const() >= constraints {
                    allowed_nodes.insert_const(index as u32);
                }

                index += 1;
            }

            if allowed_nodes.is_empty_const() {
                break 'c;
            } else {
                av[constraints as usize] = allowed_nodes;
            }

            constraints += 1;
        }

        av
    };

    /// Sets of tiles from this set grouped by the number of adjacencies in descending order
    const TILES_GROUPED_BY_ADJACENCIES: [GridSet; 9] = {
        let mut av = [GridSet::EMPTY; 9];
        let mut tiles_found = 0u32;
        let mut adj_count = 0;
        let mut sets_placed = 0;
        while tiles_found < Self::ADJACENCIES.len() as u32 {
            let mut set = GridSet::EMPTY;
            let mut index = 0;
            while index < Self::ADJACENCIES.len() {
                let count = Self::ADJACENCIES[index].count_const();
                if count == adj_count {
                    set.insert_const(index as u32);
                }
                index += 1;
            }

            if !set.is_empty_const() {
                tiles_found += set.len_const();
                av[sets_placed] = set;
                sets_placed += 1;
            }
            adj_count += 1;
        }
        av.reverse();
        av.rotate_right(sets_placed);

        av
    };

    fn move_tile_direction(tile: GridTile, direction: ArrowDirection)-> Option<GridTile>;

    fn tile_positioning(t: GridTile)-> TilePositioning;

    fn count_tiles_with_positioning(t: TilePositioning)-> usize;

    fn board_dimensions(tile_radius: f32) -> Vec2;

    fn are_tiles_adjacent(t1: GridTile, t2: GridTile) -> bool {
        Self::ADJACENCIES[t1.0 as usize].contains_const(t2.0 as u32)
    }

    fn get_adjacent_tiles(t1: GridTile) -> GridSet {
        Self::ADJACENCIES[t1.0 as usize]
    }

    fn iter_adjacent_tiles(t1: GridTile) -> impl Iterator<Item = GridTile> {
        Self::ADJACENCIES[t1.0 as usize]
            .iter_const()
            .map(|x| GridTile(x as u8))
    }

    fn try_get_nth_adjacent_tile(t1: GridTile, n: u32) -> Option<GridTile> {
        Self::ADJACENCIES[t1.0 as usize]
            .nth(n)
            .map(|x| GridTile(x as u8))
    }

    fn iter_tiles() -> impl ExactSizeIterator<Item = GridTile> + FusedIterator {
        (0..GRID_SIZE).map(|x| GridTile(x as u8))
    }

    fn tile_position_u8(tile: GridTile) -> U8Vec2;

    fn tile_position(tile: GridTile, tile_size: f32, from_centre: bool) -> glam::Vec2 {
        let p = Self::tile_position_u8(tile);
        let p = p.as_vec2() * tile_size * 0.5;
        if from_centre {
            p + Vec2::splat(0.5 * tile_size)
        } else {
            p
        }
    }

    fn get_tile_from_position(position: Vec2, tile_size: f32, sensitivity: f32)
    -> Option<GridTile>;

    // fn try_add_virtual_constraints(
    //     constraints: &mut crate::finder::constraints::Constraints<GRID_SIZE>,
    // ) -> bool;

    fn symmetry_restrictions() -> impl ExactSizeIterator<Item = SymmetryRestriction>;

    fn get_allowed_by_symmetry(used_grid: GridSet) -> GridSet {
        for restriction in Self::symmetry_restrictions() {
            if used_grid.is_subset_const(&restriction.previous_used_superset) {
                return restriction.allowed_nodes;
            }
        }
        Self::ALL_NODES
    }

    fn format_grid(grid: Grid<GRID_SIZE>) -> String;

    fn format_grid_single_line(grid: Grid<GRID_SIZE>) -> String {
        grid.0.iter().map(|x| x.as_char()).join("|")
    }

    fn format_grid_set(grid: GridSet) -> String {
        let grid = Grid(std::array::from_fn(|x| {
            if grid.contains_const(x as u32) {
                crate::prelude::Character::X
            } else {
                crate::prelude::Character::Blank
            }
        }));
        Self::format_grid(grid)
    }

    fn score_solution(solution: &Solution<GRID_SIZE>) -> i32 {
        //look at the first five tiles. Score is accumulated based on letters going left to right, preferably in the same row
        //const FIRST_ROW: Tile<WID> = Tile::new_const::<0, 1>();
        let Some(solution_first) = solution.first() else {
            return 0;
        };
        let solution_first_position = Self::tile_position_u8(*solution_first);
        let tile_0_position = Self::tile_position_u8(GridTile(0));
        let mut total = if solution_first == &GridTile(0) {
            10 //bonus for being top left
        } else if solution_first_position.y <= 2 && solution_first_position.x <= tile_0_position.x {
            8 // bonus for being one below top left
        } else {
            0
        };

        let mut streak = true;

        let windows = solution.iter().tuple_windows();

        for (a, b) in windows {
            let a_position = Self::tile_position_u8(*a);
            let b_position = Self::tile_position_u8(*b);

            match b_position.x.cmp(&a_position.x) {
                std::cmp::Ordering::Less => {
                    return total - 3;
                }
                std::cmp::Ordering::Equal => streak = false,
                std::cmp::Ordering::Greater => {
                    if a_position.y == b_position.y {
                        total += if streak { 4 } else { 2 };
                    } else {
                        total += 1;
                        streak = false;
                    }
                }
            }
        }

        total
    }

    fn possible_next_tiles_in_taboo_word(tile: GridTile) -> GridSet;

    fn symmetries() -> impl ExactSizeIterator<Item = Symmetry<GRID_SIZE>>;

    ///lines that can make a headline word
    fn headline_word_lines() -> impl Iterator<Item = Solution<GRID_SIZE>>;

    const ROTATE_CLOCKWISE: Symmetry<GRID_SIZE>;
    const ROTATE_ANTICLOCKWISE: Symmetry<GRID_SIZE>;
    const REFLECT: Symmetry<GRID_SIZE>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SymmetryRestriction {
    pub previous_used_superset: GridSet,
    pub allowed_nodes: GridSet,
}

impl SymmetryRestriction {
    pub const fn new(previous_used_superset_arr: &[u8], allowed_nodes_arr: &[u8]) -> Self {
        let mut previous_used_superset = GridSet::EMPTY;
        let mut allowed_nodes = GridSet::EMPTY;

        let mut index = 0;
        while index < previous_used_superset_arr.len() {
            previous_used_superset.insert_const(previous_used_superset_arr[index] as u32);
            index += 1;
        }

        let mut index = 0;
        while index < allowed_nodes_arr.len() {
            allowed_nodes.insert_const(allowed_nodes_arr[index] as u32);
            index += 1;
        }

        Self {
            previous_used_superset,
            allowed_nodes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hexagon19Layout;

impl GridLayout<19> for Hexagon19Layout {
    // const TILES_WIDE: usize = 5;
    // const TILES_HIGH: usize = 5;

    const NAME: &'static str = "hexagon19";

    const GRID_LAYOUT_TYPE: GridLayoutType = GridLayoutType::Hexagon19Fat;

    const ADJACENCIES: [GridSet; 19] = HEXAGON_19_ADJACENCIES;

    const TILE_SHAPE: TileShape = TileShape::HexagonPointyTop;

    const GAME_URL: &'static str = "https://hexagon-salad.netlify.app";

    fn move_tile_direction(tile: GridTile, direction: ArrowDirection)-> Option<GridTile> {
        match direction{
            ArrowDirection::Up => {
                let amount_to_sub = match tile.0 {
                    0|1|2=> return None,
                    3|4|5 => 3,
                    6|7|8|9|10  |16|17|18 => 4,
                    11|12|13|14|15 => 5,
                    _=> return None,
                };


                let new_inner  = tile.0.checked_sub(amount_to_sub)?;
                Some(GridTile(new_inner))
            },
            ArrowDirection::Down => {
                let amount_to_add = match tile.0 {
                    0|1|2|11|12|13|14|15 => 4,
                    16|17|18=> {return None;}
                    3|4|5|6|7|8|9|10=> 5,
                    _=>{return None;}
                };

                let new_inner  = tile.0.checked_add(amount_to_add)?;
                
                Some(GridTile(new_inner))
            },
            ArrowDirection::Left => {
                if matches!(tile.0, 0|3|7|12|16){
                    return None;
                }
                let new_inner  = tile.0.checked_sub(1)?;                
                Some(GridTile(new_inner))
            },
            ArrowDirection::Right => {                
                if matches!(tile.0, 2|6|11|15|18){
                    return None;
                }
                let new_inner  = tile.0.checked_add(1)?;                                
                Some(GridTile(new_inner))
            },
        }
    }

    fn count_tiles_with_positioning(t: TilePositioning)-> usize {
        match  t{
            TilePositioning::Corner => 6,
            TilePositioning::Edge => 6,
            TilePositioning::Center => 7,
        }
    }

    fn tile_positioning(t: GridTile)-> TilePositioning {
        match t.0{
            0|2|11|18|16|7=> TilePositioning::Corner,
            1|6|15|17|12|3=> TilePositioning::Edge,
            4|5|8|9|10|13|14 => TilePositioning::Center,
            _=> TilePositioning::Center
        }
    }

    fn tile_position_u8(tile: GridTile) -> U8Vec2 {
        HEXAGON_19_POSITIONS[tile.inner_usize()]
    }

    fn board_dimensions(tile_radius: f32) -> Vec2 {
        Vec2 {
            x: 5.0 * tile_radius * SQRT_3,
            y: 8.0 * tile_radius,
        }
    }

    fn tile_position(tile: GridTile, tile_diameter: f32, from_centre: bool) -> glam::Vec2 {
        let p = Self::tile_position_u8(tile);
        let p = p.as_vec2() * tile_diameter * 0.25 * Vec2 { x: SQRT_3, y: 1.5 };
        if from_centre {
            p + Vec2::new(tile_diameter * 0.25 * SQRT_3, tile_diameter * 0.5)
        } else {
            p
        }
    }

    fn get_tile_from_position(
        position: Vec2,
        tile_diameter: f32,
        sensitivity: f32,
    ) -> Option<GridTile> {
        const RECT_SIZE: Vec2 = Vec2 {
            x: SQRT_3 * 0.5,
            y: 1.0,
        };

        //TODO efficiency

        //log::info!("Get tile from position {position}");
        if position.x < 0.0 || position.y < 0.0 {
            return None;
        }

        let tile_radius = tile_diameter * 0.5;

        let scaled_position = position / (tile_radius * RECT_SIZE);

        let (x, y) = (
            scaled_position.x.floor() as usize,
            scaled_position.y.floor() as usize,
        );

        enum TileOptions {
            E,
            S(u8),
            P(u8, u8),
        }

        use TileOptions::*;

        #[rustfmt::skip]
        const TILES_BY_SQUARE: [[TileOptions; 10];8] = [
            [E,E,S(0),S(0),S(1), S(1), S(2), S(2), E, E ],
            [E,S(3),P(0,3), P(0,4), P(1,4), P(1,5), P(2,5), P(2,6), S(6), E],
            [E, S(3), S(3), S(4), S(4), S(5), S(5), S(6), S(6), E],
            [S(7), P(7,3), P(8,3), P(8,4), P(9,4), P(9,5), P(10,5), P(10,6), P(11,6), S(11)],
            [S(7), P(7,12), P(8,12), P(8,13), P(9,13), P(9,14), P(10,14), P(10,15), P(11,15), S(11)],
            [E, S(12), S(12), S(13), S(13), S(14), S(14), S(15), S(15), E],

            [E,S(12),P(16,12), P(16,13), P(17,13), P(17,14), P(18,14), P(18,15), S(15), E],
            [E,E,S(16),S(16),S(17), S(17), S(18), S(18), E, E ],
        ];

        let tile_options = TILES_BY_SQUARE.get(y).and_then(|arr| arr.get(x))?;

        let (tile, distance) = match tile_options {
            E => return None,
            S(t_index) => {
                let tile = GridTile(*t_index);
                let centre = Self::tile_position(tile, tile_diameter, true);
                let distance = position.distance(centre);

                (tile, distance)
            }
            P(index1, index2) => {
                let t1 = GridTile(*index1);
                let t2 = GridTile(*index2);

                let centre_1 = Self::tile_position(t1, tile_diameter, true);
                let centre_2 = Self::tile_position(t2, tile_diameter, true);

                let d1 = position.distance(centre_1);
                let d2 = position.distance(centre_2);

                if d1 <= d2 { (t1, d1) } else { (t2, d2) }
            }
        };

        if distance / tile_diameter <= sensitivity {
            Some(tile)
        } else {
            None
        }
    }

    fn symmetry_restrictions() -> impl ExactSizeIterator<Item = SymmetryRestriction> {
        const SYMMETRY_RESTRICTIONS: [SymmetryRestriction; 3] = [
            SymmetryRestriction::new(&[], &[0, 1, 4, 9]),
            SymmetryRestriction::new(&[9], &[0, 1, 4]),
            SymmetryRestriction::new(&[0, 1, 4, 9], &[0, 1, 2, 4, 5, 6, 9, 10, 11, 14, 15, 18]),
        ];
        SYMMETRY_RESTRICTIONS.into_iter()
    }

    fn format_grid(grid: Grid<19>) -> String {
        //   0 1 2
        //  3 4 5 6
        // 7 8 9 A B
        //  C D E F
        //   G H I

        let mut s = String::new();

        for (index, c) in grid.iter().enumerate() {
            let (spaces, new_line) = match index {
                0 => (2, false),
                3 => (1, true),
                7 => (0, true),
                12 => (1, true),
                16 => (2, true),
                _ => (1, false),
            };
            if new_line {
                s.push('\n');
            }
            for _ in 0..spaces {
                s.push(' ');
            }

            s.push(c.as_char());
        }
        s
    }

    fn possible_next_tiles_in_taboo_word(tile: GridTile) -> GridSet {
        const SETS: [GridSet; 19] = {
            let mut sets = [GridSet::EMPTY; 19];
            let mut index = 0u32;
            while index < 19 {
                let mut set = BitSet32::EMPTY;
                set.insert_const(index + 1); //east
                set.insert_const(index + 4); //south
                set.insert_const(index + 5); //south east
                set.intersect_with_const(&HEXAGON_19_ADJACENCIES[index as usize]); //remove those that aren't adjacent

                sets[index as usize] = set;
                index += 1;
            }
            sets
        };

        SETS[tile.inner_usize()]
    }

    const REFLECT: Symmetry<19> = Symmetry::new([
        2, 1, 0, 6, 5, 4, 3, 11, 10, 9, 8, 7, 15, 14, 13, 12, 18, 17, 16,
    ]);

    const ROTATE_CLOCKWISE: Symmetry<19> = Symmetry::new([
        2, 6, 11, 1, 5, 10, 15, 0, 4, 9, 14, 18, 3, 8, 13, 17, 7, 12, 16,
    ]);

    const ROTATE_ANTICLOCKWISE: Symmetry<19> = {
        Self::ROTATE_CLOCKWISE
            .combine(Self::ROTATE_CLOCKWISE)
            .combine(Self::ROTATE_CLOCKWISE)
            .combine(Self::ROTATE_CLOCKWISE)
            .combine(Self::ROTATE_CLOCKWISE)
    };

    fn symmetries() -> impl ExactSizeIterator<Item = Symmetry<19>> {
        const SYMMETRIES: [Symmetry<19>; 12] = {
            let rot1 = Symmetry::new([
                2, 6, 11, 1, 5, 10, 15, 0, 4, 9, 14, 18, 3, 8, 13, 17, 7, 12, 16,
            ]);
            let rot2 = rot1.combine(rot1);
            let rot3 = rot2.combine(rot1);
            let rot4 = rot2.combine(rot1);
            let rot5 = rot2.combine(rot1);

            let reflection = Symmetry::new([
                2, 1, 0, 6, 5, 4, 3, 11, 10, 9, 8, 7, 15, 14, 13, 12, 18, 17, 16,
            ]);
            let rot1_ref = rot1.combine(reflection);
            let rot2_ref = rot2.combine(reflection);
            let rot3_ref = rot3.combine(reflection);
            let rot4_ref = rot4.combine(reflection);
            let rot5_ref = rot5.combine(reflection);

            [
                Symmetry::IDENTITY,
                rot1,
                rot2,
                rot3,
                rot4,
                rot5,
                reflection,
                rot1_ref,
                rot2_ref,
                rot3_ref,
                rot4_ref,
                rot5_ref,
            ]
        };

        SYMMETRIES.into_iter()
    }

    fn headline_word_lines() -> impl Iterator<Item = Solution<19>> {
        [
            Solution::from_iter([0, 1, 2].into_iter().map(GridTile)),
            Solution::from_iter([3, 4, 5, 6].into_iter().map(GridTile)),
            Solution::from_iter([7, 8, 9, 10, 11].into_iter().map(GridTile)),
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hexagon19ThinLayout;

impl GridLayout<19> for Hexagon19ThinLayout {
    const NAME: &'static str = "hexagon19thin";

    const GRID_LAYOUT_TYPE: GridLayoutType = GridLayoutType::Hexagon19Thin;

    const ADJACENCIES: [GridSet; 19] = HEXAGON_19_ADJACENCIES;

    const TILE_SHAPE: TileShape = TileShape::HexagonFlatTop;

    const GAME_URL: &'static str = "https://hexagon-salad.netlify.app";


    fn move_tile_direction(tile: GridTile, direction: ArrowDirection)-> Option<GridTile> {
        match direction{
            ArrowDirection::Up => {
                let amount_to_sub = match tile.0 {
                    4|5|6|16|17|18 => 4,
                    8|12|13|9|14|10|11|15=> 5,
                    0|1|2|3|7 => {return None},
                    _=> return None,
                };


                let new_inner  = tile.0.checked_sub(amount_to_sub)?;
                Some(GridTile(new_inner))
            },
            ArrowDirection::Down => {
                let amount_to_add = match tile.0 {
                    0|1|2|12|13|14=> 4,
                    3|7|8|4|9|5|6|10=>5,
                    16|17|18|15|11=> {return None;}
                    _=>{return None;}
                };

                let new_inner  = tile.0.checked_add(amount_to_add)?;
                
                Some(GridTile(new_inner))
            },
            ArrowDirection::Left => {
                let new_inner = match tile.0{
                    0=>3,
                    1=>0,
                    2=>1,
                    3=>7,
                    4=>3,
                    5=>4,
                    6=>5,
                    7=>return None,
                    8=>7,
                    9=>8,
                    10=>9,
                    11=>10,
                    12=>return None,
                    13=>12,
                    14=>13,
                    15=>14,
                    16=>return None,
                    17=>16,
                    18=>17,
                    _=> return None,
                };            
                Some(GridTile(new_inner))
            },
            ArrowDirection::Right => {                              
                let amount_to_add = match tile.0 {
                    0|1|3|4|5|7|8|9|10|12|13|14|16|17=> 1,                    
                    2|6|11|15|18=> {return None;}
                    _=>{return None;}
                };

                let new_inner  = tile.0.checked_add(amount_to_add)?;
                
                Some(GridTile(new_inner))
            },
        }
    }

    fn count_tiles_with_positioning(t: TilePositioning)-> usize {
        match  t{
            TilePositioning::Corner => 6,
            TilePositioning::Edge => 6,
            TilePositioning::Center => 7,
        }
    }

    fn tile_positioning(t: GridTile)-> TilePositioning {
        match t.0{
            0|2|11|18|16|7=> TilePositioning::Corner,
            1|6|15|17|12|3=> TilePositioning::Edge,
            4|5|8|9|10|13|14 => TilePositioning::Center,
            _=> TilePositioning::Center
        }
    }

    fn tile_position_u8(tile: GridTile) -> U8Vec2 {
        HEXAGON_19_ROTATED_POSITIONS[tile.inner_usize()]
    }

    fn board_dimensions(tile_radius: f32) -> Vec2 {
        Vec2 {
            x: 8.0 * tile_radius,
            y: 5.0 * tile_radius * SQRT_3,
        }
    }

    fn tile_position(tile: GridTile, tile_diameter: f32, from_centre: bool) -> glam::Vec2 {
        let p = Self::tile_position_u8(tile);
        let p = p.as_vec2() * tile_diameter * 0.25 * Vec2 { x: 1.0, y: SQRT_3 };
        if from_centre {
            p + Vec2::new(tile_diameter * 0.25 * SQRT_3, tile_diameter * 0.5)
        } else {
            p
        }
    }

    fn get_tile_from_position(
        position: Vec2,
        tile_diameter: f32,
        sensitivity: f32,
    ) -> Option<GridTile> {
        const RECT_SIZE: Vec2 = Vec2 {
            x: 1.0,
            y: SQRT_3 * 0.5,
        };

        //TODO efficiency

        //log::info!("Get tile from position {position}");
        if position.x < 0.0 || position.y < 0.0 {
            return None;
        }

        let tile_radius = tile_diameter * 0.5;

        let scaled_position = position / (tile_radius * RECT_SIZE);

        let (x, y) = (
            scaled_position.x.floor() as usize,
            scaled_position.y.floor() as usize,
        );

        enum TileOptions {
            E,
            S(u8),
            P(u8, u8),
        }

        use TileOptions::*;

        #[rustfmt::skip]
        const TILES_BY_SQUARE: [[TileOptions; 8];10] = [
            [E, E, E, S(0), S(0), E, E, E],
            [E, S(3), S(3), P(3,0), P(0,1), S(1), S(1), E],
            [S(7), P(3,7), S(3), P(3,4), P(4,1), S(1), P(1,2), S(2)],
            [S(7), P(8,7), S(8), P(8,4), P(4,5), S(5), P(5,2), S(2)],
            
            [S(12), P(8,12), S(8), P(8,9), P(9,5), S(5), P(5,6), S(6)],
            [S(12), P(13,12), S(13), P(13,9), P(9,10), S(10), P(10,6), S(6)],
            
            [S(16), P(13,16), S(13), P(13,14), P(14,10), S(10), P(10,11), S(11)],
            [S(16), P(17,16), S(17), P(17,14), P(14,15), S(15), P(15,11), S(11)],
            [E, S(17), S(17), P(17,18), P(18,15), S(15), S(15), E],
            [E, E, E, S(18), S(18), E, E, E],
            
        ];

        let tile_options = TILES_BY_SQUARE.get(y).and_then(|arr| arr.get(x))?;

        let (tile, distance) = match tile_options {
            E => return None,
            S(t_index) => {
                let tile = GridTile(*t_index);
                let centre = Self::tile_position(tile, tile_diameter, true);
                let distance = position.distance(centre);

                (tile, distance)
            }
            P(index1, index2) => {
                let t1 = GridTile(*index1);
                let t2 = GridTile(*index2);

                let centre_1 = Self::tile_position(t1, tile_diameter, true);
                let centre_2 = Self::tile_position(t2, tile_diameter, true);

                let d1 = position.distance(centre_1);
                let d2 = position.distance(centre_2);

                if d1 <= d2 { (t1, d1) } else { (t2, d2) }
            }
        };

        if distance / tile_diameter <= sensitivity {
            Some(tile)
        } else {
            None
        }
    }

    fn symmetry_restrictions() -> impl ExactSizeIterator<Item = SymmetryRestriction> {
        const SYMMETRY_RESTRICTIONS: [SymmetryRestriction; 3] = [
            SymmetryRestriction::new(&[], &[0, 1, 4, 9]),
            SymmetryRestriction::new(&[9], &[0, 1, 4]),
            SymmetryRestriction::new(&[0, 1, 4, 9], &[0, 1, 2, 4, 5, 6, 9, 10, 11, 14, 15, 18]),
        ];
        SYMMETRY_RESTRICTIONS.into_iter()
    }

    fn format_grid(grid: Grid<19>) -> String {
        //   0 1 2
        //  3 4 5 6
        // 7 8 9 A B
        //  C D E F
        //   G H I
        //todo rotate this to
        //  0
        // 3 1
        //7 4 2
        // 8 5
        //C 9 6
        // D A
        //G E B
        // H F
        //  I
        let mut s = String::new();

        for (index, c) in grid.iter().enumerate() {
            let (spaces, new_line) = match index {
                0 => (2, false),
                3 => (1, true),
                7 => (0, true),
                12 => (1, true),
                16 => (2, true),
                _ => (1, false),
            };
            if new_line {
                s.push('\n');
            }
            for _ in 0..spaces {
                s.push(' ');
            }

            s.push(c.as_char());
        }
        s
    }

    fn possible_next_tiles_in_taboo_word(tile: GridTile) -> GridSet {
        const SETS: [GridSet; 19] = {
            let mut sets = [GridSet::EMPTY; 19];
            let mut index = 0u32;
            while index < 19 {
                let mut set = BitSet32::EMPTY;
                set.insert_const(index + 1); //east
                set.insert_const(index + 4); //south
                set.insert_const(index + 5); //south east
                set.intersect_with_const(&HEXAGON_19_ADJACENCIES[index as usize]); //remove those that aren't adjacent

                sets[index as usize] = set;
                index += 1;
            }
            sets
        };

        SETS[tile.inner_usize()]
    }

    const REFLECT: Symmetry<19> = Symmetry::new([
        2, 1, 0, 6, 5, 4, 3, 11, 10, 9, 8, 7, 15, 14, 13, 12, 18, 17, 16,
    ]);

    const ROTATE_CLOCKWISE: Symmetry<19> = Symmetry::new([
        2, 6, 11, 1, 5, 10, 15, 0, 4, 9, 14, 18, 3, 8, 13, 17, 7, 12, 16,
    ]);

    const ROTATE_ANTICLOCKWISE: Symmetry<19> = {
        Self::ROTATE_CLOCKWISE
            .combine(Self::ROTATE_CLOCKWISE)
            .combine(Self::ROTATE_CLOCKWISE)
            .combine(Self::ROTATE_CLOCKWISE)
            .combine(Self::ROTATE_CLOCKWISE)
    };

    fn symmetries() -> impl ExactSizeIterator<Item = Symmetry<19>> {
        const SYMMETRIES: [Symmetry<19>; 12] = {
            let rot1 = Symmetry::new([
                2, 6, 11, 1, 5, 10, 15, 0, 4, 9, 14, 18, 3, 8, 13, 17, 7, 12, 16,
            ]);
            let rot2 = rot1.combine(rot1);
            let rot3 = rot2.combine(rot1);
            let rot4 = rot2.combine(rot1);
            let rot5 = rot2.combine(rot1);

            let reflection = Symmetry::new([
                2, 1, 0, 6, 5, 4, 3, 11, 10, 9, 8, 7, 15, 14, 13, 12, 18, 17, 16,
            ]);
            let rot1_ref = rot1.combine(reflection);
            let rot2_ref = rot2.combine(reflection);
            let rot3_ref = rot3.combine(reflection);
            let rot4_ref = rot4.combine(reflection);
            let rot5_ref = rot5.combine(reflection);

            [
                Symmetry::IDENTITY,
                rot1,
                rot2,
                rot3,
                rot4,
                rot5,
                reflection,
                rot1_ref,
                rot2_ref,
                rot3_ref,
                rot4_ref,
                rot5_ref,
            ]
        };

        SYMMETRIES.into_iter()
    }

    fn headline_word_lines() -> impl Iterator<Item = Solution<19>> {
        [Solution::from_iter(
            [7, 3, 4, 1, 2].into_iter().map(GridTile),
        )]
        .into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Square16Layout;

impl GridLayout<16> for Square16Layout {
    const NAME: &'static str = "square16";

    const GRID_LAYOUT_TYPE: GridLayoutType = GridLayoutType::Square16;

    const ADJACENCIES: [GridSet; 16] = SQUARE_16_ADJACENCIES;

    const TILE_SHAPE: TileShape = TileShape::Square;

    const GAME_URL: &'static str = "https://wordsalad.online";

    fn move_tile_direction(tile: GridTile, direction: ArrowDirection)-> Option<GridTile> {
        match direction{
            ArrowDirection::Up => {
                let new_inner  = tile.0.checked_sub(4)?;
                Some(GridTile(new_inner))
            },
            ArrowDirection::Down => {
                let new_inner  = tile.0.checked_add(4)?;
                if new_inner > 15{return None;}
                Some(GridTile(new_inner))
            },
            ArrowDirection::Left => {
                if tile.0 % 4 == 0{
                    return None;
                }
                let new_inner  = tile.0.checked_sub(1)?;                
                Some(GridTile(new_inner))
            },
            ArrowDirection::Right => {                
                let new_inner  = tile.0.checked_add(1)?;                
                if new_inner % 4 == 0{
                    return None;
                }
                Some(GridTile(new_inner))
            },
        }
    }

    fn count_tiles_with_positioning(t: TilePositioning)-> usize {
        match  t{
            TilePositioning::Corner => 4,
            TilePositioning::Edge => 8,
            TilePositioning::Center => 4,
        }
    }

    fn tile_positioning(t: GridTile)-> TilePositioning {
        match t.0{
            0|3|12|15=> TilePositioning::Corner,
            1|2|7|11|13|14|4|8=> TilePositioning::Edge,
            5|6|9|10 => TilePositioning::Center,
            _=> TilePositioning::Center
        }
    }

    fn tile_position_u8(tile: GridTile) -> U8Vec2 {
        SQUARE_16_POSITIONS[tile.inner_usize()]
    }

    fn board_dimensions(tile_radius: f32) -> Vec2 {
        Vec2::splat(tile_radius * 8.0)
    }

    fn symmetry_restrictions() -> impl ExactSizeIterator<Item = SymmetryRestriction> {
        const SYMMETRY_RESTRICTIONS: [SymmetryRestriction; 2] = [
            SymmetryRestriction::new(&[], &[0, 1, 5]),
            SymmetryRestriction::new(&[0, 5, 10, 15], &[0, 1, 2, 3, 5, 6, 7, 10, 11, 15]),
        ];
        SYMMETRY_RESTRICTIONS.into_iter()
    }

    fn format_grid(grid: Grid<16>) -> String {
        let mut s = String::new();
        for (index, c) in grid.iter().enumerate() {
            if index % 4 == 0 && index > 0 {
                s.push('\n');
            }
            s.push(c.as_char());
        }
        s
    }

    fn possible_next_tiles_in_taboo_word(tile: GridTile) -> GridSet {
        const SETS: [GridSet; 16] = {
            let mut sets = [GridSet::EMPTY; 16];
            let mut index = 0u32;
            while index < 16 {
                let mut set = BitSet32::EMPTY;
                set.insert_const(index + 1); //east
                set.insert_const(index + 4); //south
                set.insert_const(index + 5); //south east
                set.intersect_with_const(&SQUARE_16_ADJACENCIES[index as usize]); //remove those that aren't adjacent

                sets[index as usize] = set;
                index += 1;
            }
            sets
        };

        SETS[tile.inner_usize()]
    }

    const ROTATE_CLOCKWISE: Symmetry<16> =
        Symmetry::new([3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13, 0, 4, 8, 12]);

    const REFLECT: Symmetry<16> =
        Symmetry::new([3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]);

    const ROTATE_ANTICLOCKWISE: Symmetry<16> = {
        Self::ROTATE_CLOCKWISE
            .combine(Self::ROTATE_CLOCKWISE)
            .combine(Self::ROTATE_CLOCKWISE)
    };

    fn symmetries() -> impl ExactSizeIterator<Item = Symmetry<16>> {
        const SYMMETRIES: [Symmetry<16>; 8] = {
            let rot1 = Symmetry::new([3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13, 0, 4, 8, 12]);
            let rot2 = rot1.combine(rot1);
            let rot3 = rot2.combine(rot1);

            let reflection = Symmetry::new([3, 2, 1, 0, 7, 6, 5, 4, 11, 10, 9, 8, 15, 14, 13, 12]);
            let rot1_ref = rot1.combine(reflection);
            let rot2_ref = rot2.combine(reflection);
            let rot3_ref = rot3.combine(reflection);

            [
                Symmetry::IDENTITY,
                rot1,
                rot2,
                rot3,
                reflection,
                rot1_ref,
                rot2_ref,
                rot3_ref,
            ]
        };

        SYMMETRIES.into_iter()
    }

    fn headline_word_lines() -> impl Iterator<Item = Solution<16>> {
        [
            Solution::from_iter([0, 1, 2, 3].into_iter().map(GridTile)),
            Solution::from_iter([4, 5, 6, 7].into_iter().map(GridTile)),
            Solution::from_iter([0, 5, 10, 15].into_iter().map(GridTile)),
        ]
        .into_iter()
    }

    fn get_tile_from_position(
        position: Vec2,
        tile_size: f32,
        sensitivity: f32,
    ) -> Option<GridTile> {
        if position.x < 0.0 || position.y < 0.0 {
            return None;
        }

        //const TILE_SIZE: f32 = BOARD_SIZE / 4.0;

        let x = position.x / tile_size;
        let y = position.y / tile_size;
        let x = x as u8;
        let y = y as u8;

        //println!("Position: {position} x: {x}; y: {y}");

        let tile = GridTile::try_from_usize::<16>(((y * 4) + x) as usize)?;

        let c = Self::tile_position(tile, tile_size, true);
        let distances = ((c - position) / tile_size).abs();

        // log!(
        //     "Position: {position}\n
        // Tile {tile}\n
        //  Sensitivity {sensitivity}\n
        //  Distances {distances}"
        // );

        //println!("Distances: {distances}");

        if distances.x <= sensitivity && distances.y <= sensitivity {
            //leptos::logging::log!("GIC: {gic:?}");
            Some(tile)
        } else {
            None
        }
    }
}

const fn chebyshev_distance(a: U8Vec2, b: U8Vec2) -> u8 {
    let x = a.x.abs_diff(b.x);
    let y = a.y.abs_diff(b.y);
    if x >= y { x } else { y }
}

const HEXAGON_19_POSITIONS: [U8Vec2; 19] = [
    U8Vec2 { x: 2, y: 0 },
    U8Vec2 { x: 4, y: 0 },
    U8Vec2 { x: 6, y: 0 },
    U8Vec2 { x: 1, y: 2 },
    U8Vec2 { x: 3, y: 2 },
    U8Vec2 { x: 5, y: 2 },
    U8Vec2 { x: 7, y: 2 },
    U8Vec2 { x: 0, y: 4 },
    U8Vec2 { x: 2, y: 4 },
    U8Vec2 { x: 4, y: 4 },
    U8Vec2 { x: 6, y: 4 },
    U8Vec2 { x: 8, y: 4 },
    U8Vec2 { x: 1, y: 6 },
    U8Vec2 { x: 3, y: 6 },
    U8Vec2 { x: 5, y: 6 },
    U8Vec2 { x: 7, y: 6 },
    U8Vec2 { x: 2, y: 8 },
    U8Vec2 { x: 4, y: 8 },
    U8Vec2 { x: 6, y: 8 },
];

const HEXAGON_19_ROTATED_POSITIONS: [U8Vec2; 19] = [
    U8Vec2 { x: 7, y: 0 },  //0
    U8Vec2 { x: 10, y: 1 }, //1
    U8Vec2 { x: 13, y: 2 }, //2
    U8Vec2 { x: 4, y: 1 },  //3
    U8Vec2 { x: 7, y: 2 },  //4
    U8Vec2 { x: 10, y: 3 }, //5
    U8Vec2 { x: 13, y: 4 }, //6
    U8Vec2 { x: 1, y: 2 },  //7
    U8Vec2 { x: 4, y: 3 },  //8
    U8Vec2 { x: 7, y: 4 },  //9
    U8Vec2 { x: 10, y: 5 }, //10
    U8Vec2 { x: 13, y: 6 }, //11
    U8Vec2 { x: 1, y: 4 },  //12
    U8Vec2 { x: 4, y: 5 },  //13
    U8Vec2 { x: 7, y: 6 },  //14
    U8Vec2 { x: 10, y: 7 }, //15
    U8Vec2 { x: 1, y: 6 },  //16
    U8Vec2 { x: 4, y: 7 },  //17
    U8Vec2 { x: 7, y: 8 },  //18
];

const SQUARE_16_POSITIONS: [U8Vec2; 16] = [
    U8Vec2 { x: 0, y: 0 },
    U8Vec2 { x: 2, y: 0 },
    U8Vec2 { x: 4, y: 0 },
    U8Vec2 { x: 6, y: 0 },
    U8Vec2 { x: 0, y: 2 },
    U8Vec2 { x: 2, y: 2 },
    U8Vec2 { x: 4, y: 2 },
    U8Vec2 { x: 6, y: 2 },
    U8Vec2 { x: 0, y: 4 },
    U8Vec2 { x: 2, y: 4 },
    U8Vec2 { x: 4, y: 4 },
    U8Vec2 { x: 6, y: 4 },
    U8Vec2 { x: 0, y: 6 },
    U8Vec2 { x: 2, y: 6 },
    U8Vec2 { x: 4, y: 6 },
    U8Vec2 { x: 6, y: 6 },
];

const SQUARE_16_ADJACENCIES: [GridSet; 16] = {
    let mut sets = [GridSet::EMPTY; 16];

    let mut a_index = 0usize;
    while a_index < sets.len() {
        let a = SQUARE_16_POSITIONS[a_index];

        let mut b_index = a_index + 1;
        while b_index < sets.len() {
            let b = SQUARE_16_POSITIONS[b_index];

            if chebyshev_distance(a, b) <= 2 {
                sets[a_index].insert_const(b_index as u32);
                sets[b_index].insert_const(a_index as u32);
            }

            b_index += 1;
        }
        a_index += 1;
    }
    sets
};

const HEXAGON_19_ADJACENCIES: [GridSet; 19] = {
    let mut sets = [GridSet::EMPTY; 19];

    let mut a_index = 0usize;
    while a_index < 19 {
        let a = HEXAGON_19_POSITIONS[a_index];

        let mut b_index = a_index + 1;
        while b_index < 19 {
            let b = HEXAGON_19_POSITIONS[b_index];

            if chebyshev_distance(a, b) <= 2 {
                sets[a_index].insert_const(b_index as u32);
                sets[b_index].insert_const(a_index as u32);
            }

            b_index += 1;
        }
        a_index += 1;
    }
    sets
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Symmetry<const GRID_SIZE: usize>([u8; GRID_SIZE]);

impl<const GRID_SIZE: usize> Symmetry<GRID_SIZE> {
    pub const fn new(x: [u8; GRID_SIZE]) -> Self {
        Self(x)
    }

    pub const IDENTITY: Self = {
        let mut arr = [0; GRID_SIZE];
        let mut i = 0;
        while i < GRID_SIZE {
            arr[i] = i as u8;
            i += 1;
        }
        Self(arr)
    };

    pub const fn combine(self, other: Self) -> Self {
        let mut arr = [0; GRID_SIZE];
        let mut index1 = 0;
        while index1 < GRID_SIZE {
            let index2 = self.0[index1] as usize;
            let index3 = other.0[index2];
            arr[index1] = index3;
            index1 += 1;
        }
        Self(arr)
    }

    pub fn apply_to_grid(self, grid: &Grid<GRID_SIZE>) -> Grid<GRID_SIZE> {
        let mut new_grid = Grid::default();
        for (to, from) in self.0.into_iter().enumerate() {
            let c = grid.0[from as usize];
            new_grid[to] = c;
        }
        new_grid
    }

    pub fn apply_to_set(self, set: BitSet32) -> BitSet32 {
        let mut new_set = BitSet32::EMPTY;

        for index in set.iter_const() {
            let new_index = self.0[index as usize];
            new_set.insert_const(new_index as u32);
        }

        new_set
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileShape {
    Square,
    // Circle, //todo hexagon
    HexagonPointyTop,
    HexagonFlatTop,
}

#[cfg(test)]
mod tests {
    use const_sized_bit_set::prelude::BitSet;

    use crate::{
        grid_layout::{GridLayout, Hexagon19Layout, Square16Layout},
        prelude::GridTile,
    };

    #[test]
    pub fn test_square_layout_tile_positions() {
        const TILE_SIZE: f32 = 100.0;

        for tile in Square16Layout::ALL_NODES.iter() {
            let tile = GridTile(tile as u8);

            let position = Square16Layout::tile_position(tile, TILE_SIZE, true);

            let tile_at_position = Square16Layout::get_tile_from_position(position, TILE_SIZE, 0.0);

            assert_eq!(Some(tile), tile_at_position)
        }
    }

    #[test]
    pub fn test_hexagon_layout_tile_positions() {
        const TILE_SIZE: f32 = 100.0;

        for tile in Hexagon19Layout::ALL_NODES.iter() {
            let tile = GridTile(tile as u8);

            let position = Hexagon19Layout::tile_position(tile, TILE_SIZE, true);

            let tile_at_position =
                Hexagon19Layout::get_tile_from_position(position, TILE_SIZE, 0.0);

            assert_eq!(Some(tile), tile_at_position)
        }
    }
}
