use bit_bag::prelude::BitBag;
use const_sized_bit_set::prelude::*;
use ustr::Ustr;

use super::word_trait::WordTrait;
use crate::{
    Character, Grid, GridSet, GridTile, grid_layout::GridLayout, prelude::*,
    special_characters::SpecialCharacters, tile_usages::TileUsages, word_trait::BasicWordTrait,
};

pub trait LevelTrait<const GRID_SIZE: usize>: Clone {
    type Word: WordTrait<GRID_SIZE>;
    type Layout: GridLayout<GRID_SIZE>;

    fn grid(&self) -> Grid<GRID_SIZE>;

    fn grid_mut(&mut self) -> &mut Grid<GRID_SIZE>;

    fn words(&self) -> &[Self::Word];

    ///Usually the attribution
    fn extra_info(&self) -> Option<Ustr>;

    fn special_colors(&self) -> Option<&[bevy_color::prelude::Srgba]>;

    ///human readable data string
    fn tsv_line(&self, title: &str, special_characters: &SpecialCharacters) -> String {
        // format!("{grid}{special_characters}\t{title}{extra}{colors}\t{words}")
        use std::fmt::Write;
        let mut s = String::new();
        for character in self.grid().iter() {
            write!(&mut s, "{}", character.as_char()).unwrap();
        }

        write!(&mut s, "{special_characters}\t{title}").unwrap();

        if let Some(extra) = self.extra_info() {
            s.push('[');
            write!(&mut s, "{extra}").unwrap();
            s.push(']');
        }

        if let Some(colors) = self.special_colors() {
            s.push('{');
            for (index, color) in colors.iter().enumerate() {
                if index > 0 {
                    s.push(',');
                }
                s.push_str(color.to_hex().as_str());
            }
            s.push('}');
        }
        s.push('\t');

        for (index, word) in self.words().iter().enumerate() {
            if index > 0 {
                s.push('\t');
            }
            s.push_str(&word.text());

            if let Some(clue) = word.clue() {
                s.push('[');
                s.push_str(clue.as_str());
                s.push(']');
            }
        }

        return s;
    }

    ///Full Url String
    fn url_string(&self, title: &str, special_characters: &SpecialCharacters) -> String {
        let unencoded = self.tsv_line(title, special_characters);

        use base64::Engine;
        let encoded = base64::engine::general_purpose::URL_SAFE.encode(unencoded);

        let game_url = Self::Layout::GAME_URL;

        format!("{game_url}/game/{encoded}")
    }

    #[must_use]
    fn meets_safety_restrictions(&self, restriction: SafetyRestriction) -> bool {
        match restriction {
            SafetyRestriction::None => true,
            SafetyRestriction::UnambiguousFirstLetters => self
                .try_get_first_letters(GridSet::EMPTY, |_| false)
                .is_some(),
            SafetyRestriction::Paper => self.is_paper_suitable(),
            SafetyRestriction::PaperAndArizona => {
                self.is_paper_suitable() && self.is_arizona_safe()
            }
        }
    }

    ///Returns the tiles which are first letter tiles if possible
    #[must_use]
    fn try_get_first_letters(
        &self,
        unneeded_tiles: GridSet,
        is_word_found: impl Fn(usize) -> bool,
    ) -> Option<GridSet> {
        let mut final_set = GridSet::EMPTY;
        let grid = self.grid();
        for (index, word) in self.words().iter().enumerate() {
            if !is_word_found(index) {
                let mut current_set = GridSet::EMPTY;
                for solution in word.find_solutions_with_tiles::<Self::Layout>(grid, unneeded_tiles)
                {
                    if let Some(first_tile) = solution.first() {
                        current_set.insert_const(first_tile.inner_u32());

                        if current_set.len_const() > 1 {
                            return None;
                        }
                    }
                }

                final_set.union_with_const(&current_set);
            }
        }
        Some(final_set)
    }

    #[must_use]
    fn is_paper_suitable(&self) -> bool {
        TileUsages::try_from_level_or_error::<Self::Layout>(self).is_ok()
    }

    #[must_use]
    fn is_arizona_safe(&self) -> bool {
        //we want to avoid characters that are duplicates where either the duplicate is the last the letters after the duplicate
        for word in self.words() {
            if !word.is_arizona_safe::<Self::Layout>(&self.grid()) {
                return false;
            }
        }

        true
    }

    fn draw_grid_svg(&self, special_characters: &SpecialCharacters) -> String {
        crate::draw_grid::draw::<GRID_SIZE, Self::Layout>(self, special_characters)
    }

    fn draw_paper_svg(
        &self,
        title: &str,
        subtitle: &str,
        modifiers: &crate::draw_paper::DrawPaperModifiers,
        special_characters: &SpecialCharacters,
    ) -> String {
        crate::draw_paper::draw::<GRID_SIZE, Self::Layout>(
            self,
            title,
            subtitle,
            modifiers,
            special_characters,
        )
    }

    fn is_falloff_100_inner(level: &Self, capacity: usize, slice: &mut [u8]) -> bool {
        for s in 0..capacity {
            let set = BitSet64::from_inner_const(s as u64);

            let unneeded =
                level.calculate_unneeded_tiles(GridSet::EMPTY, |w| !set.contains_const(w as u32));
            let unneeded_count = unneeded.count() as u8;

            slice[s] = unneeded_count;

            for x in set {
                let set2 = set.with_removed(x);
                let set2_inner = set2.into_inner_const();

                let set2_count = slice[set2_inner as usize];

                if set2_count <= unneeded_count {
                    return false;
                }
            }
        }

        true
    }

    fn is_falloff_100(&self) -> bool {
        let n = self.words().len() as u32;
        let capacity = 2usize.pow(n);
        match n {
            0 => true,
            1 => true,
            2..=7 => Self::is_falloff_100_inner(self, capacity, &mut [0; 128]),
            8 => Self::is_falloff_100_inner(self, capacity, &mut [0; 256]),
            _ => {
                let mut unneeded_counts: Vec<u8> = vec![0; capacity];

                Self::is_falloff_100_inner(self, capacity, &mut unneeded_counts)
            }
        }
    }

    fn final_word_score(&self, word: &impl WordTrait<GRID_SIZE>) -> Option<f32> {
        word.find_solutions::<Self::Layout>(self.grid())
            .flat_map(|solution| {
                let path_count = crate::possible_paths::count_solution_possible_paths::<
                    GRID_SIZE,
                    Self::Layout,
                >(solution);
                //the score is 200x the inverse of the number of paths through the tiles
                if path_count == 0 {
                    return Option::<f32>::None;
                }

                Some(200.0 / (path_count as f32))
            })
            .max_by(num_traits::float::TotalOrder::total_cmp)
    }

    // fn count_crossings(solution: Solution< GRID_SIZE>) -> usize {
    //     let mut set = GridSet::default();
    //     let mut count = 0usize;
    //     for (l, r) in solution.iter().tuple_windows() {
    //         if !l.is_contiguous_with(r) {
    //             if let Some(corner) =
    //                 GridTile::try_new(l.x().min(r.x()), l.y().min(r.y()))
    //             {
    //                 //this is the south east corner of the north west tile.
    //                 if set.get_bit(&corner) {
    //                     count += 1;
    //                 } else {
    //                     set.set_bit(&corner, true);
    //                 }
    //             }
    //         }
    //     }
    //     count
    // }

    // ///Count the number of diagonal crossings
    // fn count_diagonal_crossings(&self) -> usize {
    //     let mut total = 0usize;

    //     'words: for word in self.words() {
    //         let mut min = usize::MAX;
    //         for solution in word.find_solutions(self.grid()) {
    //             let c = Self::count_crossings(solution);
    //             if c == 0 {
    //                 continue 'words;
    //             } else {
    //                 min = min.min(c);
    //             }
    //         }

    //         total = total.saturating_add(min);
    //     }

    //     total
    // }

    fn calculate_falloff_probability(&self) -> f32 {
        let n = self.words().len() as u32;
        let capacity = 2usize.pow(n);
        let mut unneeded_counts: Vec<u8> = vec![0; capacity];
        let mut numerator = 0u32;

        // https://oeis.org/A001787
        let denominator: u32 = n * (2u32.pow(n.saturating_sub(1)));

        for s in 0..capacity {
            let set = BitSet64::from_inner(s as u64);

            let unneeded =
                self.calculate_unneeded_tiles(GridSet::EMPTY, |w| !set.contains(w as u32));
            let unneeded_count = unneeded.count() as u8;

            unneeded_counts[s] = unneeded_count;

            for x in set {
                let set2 = set.with_removed(x);
                let set2_inner = set2.into_inner_const();

                let set2_count = unneeded_counts[set2_inner as usize];

                if set2_count > unneeded_count {
                    numerator += 1;
                }
            }
        }

        numerator as f32 / denominator as f32
    }

    /// Normalized difficulty score based on what proportion of letters use the more connected tiles
    fn difficulty_score(&self) -> f32 {
        let mut max_score: f32 = 0.0;

        const CENTRAL_TILE_SCORE: f32 = 8.0;
        const EDGE_TILE_SCORE: f32 = 5.0;
        const CORNER_TILE_SCORE: f32 = 3.0;

        for word in self.words() {
            let chars = word.characters().len();

            let layout_central_tiles =
                Self::Layout::count_tiles_with_positioning(TilePositioning::Center);
            let layout_edge_tiles =
                Self::Layout::count_tiles_with_positioning(TilePositioning::Edge);

            let central_tiles: usize;
            let edge_tiles: usize;
            let corner_tiles: usize;

            if chars <= layout_central_tiles {
                central_tiles = chars;
                edge_tiles = 0;
                corner_tiles = 0
            } else if chars <= (layout_central_tiles + layout_edge_tiles) {
                central_tiles = layout_central_tiles;
                edge_tiles = chars - layout_central_tiles;
                corner_tiles = 0;
            } else {
                central_tiles = layout_central_tiles;
                edge_tiles = layout_edge_tiles;
                corner_tiles = chars - (layout_central_tiles + layout_edge_tiles);
            }

            let word_score = ((central_tiles as f32) * CENTRAL_TILE_SCORE)
                + ((edge_tiles as f32) * EDGE_TILE_SCORE)
                + ((corner_tiles as f32) * CORNER_TILE_SCORE);

            max_score += word_score;
        }

        let max_score = max_score;
        let grid_prime_bag = LetterCounts::try_from_iter(self.grid().iter())
            .expect("Should be able to make prime bag for grid");

        let mut score: f32 = 0.0;

        for (tile, character) in self.grid().enumerate() {
            let mut words_using = 0usize;
            let mut words_maybe_using = 0usize;

            let char_instances = grid_prime_bag.count_instances(character);

            for word in self.words() {
                let word_char_instances =
                    word.characters().iter().filter(|c| character.eq(c)).count();

                if word_char_instances == 0 {
                } else if word_char_instances == char_instances
                    || word
                        .find_solutions::<Self::Layout>(self.grid())
                        .all(|arr| arr.contains(&tile))
                {
                    words_using += 1;
                } else {
                    words_maybe_using += 1;
                }
            }

            let tile_score = match Self::Layout::tile_positioning(tile) {
                TilePositioning::Corner => CORNER_TILE_SCORE,
                TilePositioning::Edge => EDGE_TILE_SCORE,
                TilePositioning::Center => CENTRAL_TILE_SCORE,
            };

            score += (tile_score) * ((words_using as f32) + ((words_maybe_using as f32) * 0.5));
        }

        score / max_score
    }

    /// Returns whether the every non-blank tile in the grid is needed
    fn are_all_tiles_needed(&self) -> bool {
        let mut blank_tiles: GridSet = GridSet::default();

        for tile in self
            .grid()
            .enumerate()
            .filter(|x| x.1.is_blank())
            .map(|x| x.0)
        {
            blank_tiles = blank_tiles.with_inserted(tile.inner_u32());
        }

        let unneeded_tiles = self.calculate_unneeded_tiles(blank_tiles, |_| false);

        unneeded_tiles == blank_tiles
    }

    fn replace_unneeded_tiles_with_blanks(&mut self) {
        let unneeded_tiles = self.calculate_unneeded_tiles(BitSet32::EMPTY, |_| false);

        let grid = self.grid_mut();
        for tile_index in unneeded_tiles {
            grid[tile_index as usize] = Character::Blank;
        }
    }

    fn calculate_unneeded_tiles<F: Fn(usize) -> bool>(
        &self,
        mut unneeded_tiles: GridSet,
        is_word_found: F,
    ) -> GridSet {
        let mut needed_characters: LetterCounts = LetterCounts::default();
        for word in self
            .words()
            .iter()
            .enumerate()
            .filter(|x| !is_word_found(x.0))
            .map(|x| x.1)
        {
            let Some(characters) = word.letter_counts() else {
                //warn!("Could not get letter counts for word");
                return unneeded_tiles;
            };

            needed_characters.0.union_with(&characters.0);
        }

        let grid = self.grid();

        let remaining_characters = grid
            .enumerate()
            .filter(|(tile, _)| !unneeded_tiles.contains_const(tile.inner_u32()))
            .map(|x| x.1)
            .filter(|x| !x.is_blank());
        let Some(remaining_characters) = LetterCounts::try_from_iter(remaining_characters) else {
            //warn!("Could not get letter counts of remaining tiles");
            return unneeded_tiles;
        };

        let Some(potentially_redundant_characters) = remaining_characters
            .0
            .with_checked_difference(&needed_characters.0)
        else {
            //warn!("Remaining characters was not a superset of needed characters"); //todo add log to this crate
            return unneeded_tiles;
        };

        'character_groups: for (character, mut remaining_copies) in potentially_redundant_characters
            .iter_element_groups()
            .map(|x| {
                (
                    Character::from_repr(x.element as u8).unwrap(),
                    x.count.get(),
                )
            })
        {
            let character_tiles = grid.enumerate().filter(|x| x.1 == character).map(|x| x.0);
            if needed_characters.0.element_count(character.as_u32()) > 0 {
                //we have additional copies of this character - try removing them
                'tiles_to_check: for tile in character_tiles {
                    if unneeded_tiles.contains_const(tile.inner_u32()) {
                        //we've already excluded this tile
                        continue 'tiles_to_check;
                    }

                    let mut remaining_grid = self.grid();
                    for t in unneeded_tiles.iter_const() {
                        remaining_grid[GridTile(t as u8)] = Character::Blank;
                    }
                    remaining_grid[tile] = Character::Blank;

                    for word in self
                        .words()
                        .iter()
                        .enumerate()
                        .filter(|x| !is_word_found(x.0))
                        .map(|x| x.1)
                    {
                        if word.find_solution::<Self::Layout>(remaining_grid).is_none() {
                            continue 'tiles_to_check;
                        }
                    }
                    //this tile is not needed for any solutions
                    unneeded_tiles.insert_const(tile.inner_u32());
                    remaining_copies -= 1;
                    if remaining_copies == 0 {
                        continue 'character_groups;
                    }
                }
            } else {
                //remove this character completely
                for tile in character_tiles {
                    unneeded_tiles.insert_const(tile.inner_u32());
                }
            }
        }

        unneeded_tiles
    }

    fn first_clue_index(&self) -> Option<usize> {
        self.words()
            .iter()
            .enumerate()
            .filter(|(_, word)| word.clue().is_some())
            .map(|x| x.0)
            .next()
    }

    fn next_clue_index(
        &self,
        current_index: usize,
        is_word_found: impl Fn(usize) -> bool,
    ) -> Option<usize> {
        let len = self.words().len();
        (0..len)
            .cycle()
            .skip(current_index + 1)
            .take(len)
            .find(|x| !is_word_found(*x))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        draw_paper::AllDrawPaperModifiers,
        grid_layout::{Hexagon19FatLayout, Square16Layout},
        prelude::*,
    };

    #[test]
    pub fn test_falloff_probability() {
        let level = DesignedLevel::<16, Square16Layout>::from_tsv_line(
            // spellchecker:disable-next-line
            "ASHPKILOEUIOGNDT\tSports\tPOLO\tSHOOTING\tKENDO\tSAILING\tLUGE\tSKIING",
            true,
        )
        .unwrap();

        let p = LevelTrait::calculate_falloff_probability(&level);

        assert_eq!(0.9583333, p);
    }

    #[test]
    pub fn test_arizona_safe() {
        let level1 = DesignedLevel::<16, Square16Layout>::from_tsv_line(
            // spellchecker:disable-next-line
            "GVIOIERZNGOADIAN\tUS States\tArizona",
            true,
        )
        .unwrap();

        let is_arizona_safe = LevelTrait::is_arizona_safe(&level1);
        assert!(!is_arizona_safe, "Should not be safe");

        let level2 = DesignedLevel::<16, Square16Layout>::from_tsv_line(
            // spellchecker:disable-next-line
            "GVIOIERZNGOADIAN\tUS States\tArizon",
            true,
        )
        .unwrap();

        let is_arizona_safe = LevelTrait::is_arizona_safe(&level2);
        assert!(is_arizona_safe, "Should be safe")
    }

    #[test]
    pub fn test_draw_grid() {
        let level1 = crate::designed_level::DesignedLevel::<16, Square16Layout>::from_tsv_line(
            // spellchecker:disable-next-line
            "CHSTWELADABFRROS\tFurniture\tchest\tshelf\tsofa\ttable\twardrobe",
            true,
        )
        .unwrap();

        let svg = level1.draw_grid_svg(&crate::special_characters::SpecialCharacters::NONE);

        let svg = svg.replace(
            r#"xmlns="http://www.w3.org/2000/svg""#,
            r#"xmlns="http://www.w3.org/2000/svg" style="background:white""#,
        );

        let path = "furniture_grid.svg";

        match std::panic::catch_unwind(|| {
            insta::assert_snapshot!(svg.clone());
        }) {
            Ok(()) => {
                if !std::fs::exists("path").unwrap() {
                    std::fs::write(path, svg.clone()).unwrap();
                }
            }
            Err(_err) => {
                std::fs::write(path, svg.clone()).unwrap();
                insta::assert_snapshot!(svg);
            }
        }
    }

    #[test]
    pub fn test_draw_grid_hexagon() {
        let level1 = crate::designed_level::DesignedLevel::<19, Hexagon19ThinLayout>::from_tsv_line(
            // spellchecker:disable-next-line
            r#"CREHAUSORLADIOMESTR		aroma[You might pick it up at a coffee shop]	choir[Ones who agree with you metaphorically]	Christmas[A famous father]	Carol[Number by a door]	crusade[Campaign religiously]	Treasure[Something found at "X"]	measure[Piano Bar]	medal[Come third or better]	salome[Dancer Of The Seven Veils]	tremor[It's a fault's fault]"#,
            true
        )
        .unwrap();

        let svg = level1.draw_grid_svg(&crate::special_characters::SpecialCharacters::NONE);

        let svg = svg.replace(
            r#"xmlns="http://www.w3.org/2000/svg""#,
            r#"xmlns="http://www.w3.org/2000/svg" style="background:white""#,
        );

        let path = "furniture_grid_hexagon.svg";

        match std::panic::catch_unwind(|| {
            insta::assert_snapshot!(svg.clone());
        }) {
            Ok(()) => {
                if !std::fs::exists("path").unwrap() {
                    std::fs::write(path, svg.clone()).unwrap();
                }
            }
            Err(_err) => {
                std::fs::write(path, svg.clone()).unwrap();
                insta::assert_snapshot!(svg);
            }
        }
    }

    #[test]
    pub fn test_paper_svg() {
        let level1 = crate::designed_level::DesignedLevel::<16, Square16Layout>::from_tsv_line(
            // spellchecker:disable-next-line
            "CHSTWELADABFRROS\tFurniture\tchest\tshelf\tsofa\ttable\twardrobe",
            true,
        )
        .unwrap();

        let svg = level1.draw_paper_svg(
            "Furniture",
            "",
            &Default::default(),
            &crate::special_characters::SpecialCharacters::NONE,
        );

        let svg = svg.replace(
            r#"xmlns="http://www.w3.org/2000/svg""#,
            r#"xmlns="http://www.w3.org/2000/svg" style="background:white""#,
        );

        let path = "furniture.svg";

        match std::panic::catch_unwind(|| {
            insta::assert_snapshot!(svg.clone());
        }) {
            Ok(()) => {
                if !std::fs::exists("path").unwrap() {
                    std::fs::write(path, svg.clone()).unwrap();
                }
            }
            Err(_err) => {
                std::fs::write(path, svg.clone()).unwrap();
                insta::assert_snapshot!(svg);
            }
        }
    }

    #[test]
    pub fn test_paper_svg_with_modifiers() {
        let level1 = crate::designed_level::DesignedLevel::<16, Square16Layout>::from_tsv_line(
            // spellchecker:disable-next-line
            "CHSTWELADABFRROS\tFurniture\tchest\tshelf\tsofa\ttable\twardrobe",
            true,
        )
        .unwrap();

        let ant_path = "m60.9 145.5c-4.2-1-8.9-4.6-12.6-9.5-1.5-2-2.7-4.1-2.5-4.3 0 0 .9.1 1.8.3 4.6 1 9.1 1.3 16.5 1.3s11.2-.3 16-1.3c.9-.2 1.8-.3 1.8-.3.3.3-3 4.9-5 7-5.3 5.7-10.9 8-16 6.7zm-43.4-5.8c-.2-.5-.4-1.3-.4-1.8 0-.9 0-.9-.7-.1-.4.4-.9 1-1 1.3-.3.4-.3.4-.3-.6 0-1.6.4-2.3 3.3-5 2.2-2.1 2.5-2.5 2.3-2.9-.3-.6.2-2.3 2.9-9.7.9-2.3 1.6-4.3 1.6-4.5 0-.1-.3-.5-.7-.8-.6-.5-.7-.7-.5-1.5.1-.9 6.6-20.8 7.9-24.5.4-1 .7-2.1.7-2.4 0-.9 1.4-1.8 4.1-2.5 4.8-1.3 8.6-1.8 12.4-1.7 3.5.1 3.5.1 5.1-1.4 1.4-1.3 1.6-1.6 1.8-2.8.1-.7.4-1.9.6-2.6.4-1.2.4-1.2-1.1-3.3-1.5-2.1-1.5-2.1-3.4-1.3-2.4 1-5 1.2-7.2.7-1.6-.4-11.6-4.7-13.3-5.8-.5-.3-1-.4-1.1-.3-.1.1-1.9 3.2-4 7-5 9-7 12.4-7.8 12.9-.6.4-.8.4-1.5.1-.5-.2-.9-.2-.9-.1s-1.4 2.8-2.9 5.8c-2.4 4.7-2.9 5.6-3.5 5.6-.6.1-4.4 3.2-5.4 4.4-.3.3-.7 1.2-.9 1.9-.4 1.3-.4 1.3-.8.4-.3-.7-.3-1.1-.1-1.8.2-.6.2-.9-.1-.9-.4 0-1.3.9-1.7 1.5-.4.7-.7.2-.5-.8.2-1.2 1.7-2.5 4.7-4.3 2.2-1.2 2.5-1.5 2.4-2.1-.1-.4 1-2.2 3.5-5.9 3.2-4.7 3.5-5.3 3.2-5.7-.8-.9-.5-1.8 1.6-4.5 2.9-3.6 10.1-12.2 12.7-15.1 1.2-1.3 2.2-2.5 2.2-2.7 0-.3 1.2-.8 1.9-.8.3 0 2.1.5 4.1 1.2 3.9 1.3 8.9 3.7 11.4 5.6.9.7 1.7 1.2 1.9 1.2s.8-.2 1.5-.4c1.2-.4 1.2-.4.3-2.7-.8-2.1-1.3-3.9-1.8-7.5-.2-1.4-.2-1.4-1.9-2-2.1-.7-5.5-2.6-7-3.9-.9-.8-1.1-1.1-1.1-1.9 0-.9-.6-1.6-6.4-7.9-8.4-9.1-8.3-9-7.2-10.3.5-.6.5-.6-2.9-4.5-2.5-2.9-3.4-4-3.3-4.4.1-.5-.3-.8-3-1.9-3.5-1.5-4.8-2.6-4.8-4 0-.9 0-.9.4-.3.4.6 1.9 1.5 2.1 1.3.1-.1 0-.4-.2-.8-.3-.7-.1-2.6.2-2.6.1 0 .3.4.5.9s.7 1.3 1.2 1.8c1.3 1.3 4.4 3.4 5.3 3.6.7.1 1.2.8 3.5 4.3 1.5 2.3 2.8 4.3 3 4.5.2.3.3.3.7-.1 1-.9 1.9-.6 3.2 1.4s4.6 7.6 7.1 12.3c1.9 3.6 2 3.8 2.7 3.5.9-.3 1.8-.1 3.5 1 1.8 1 4.8 2.6 4.9 2.5 0 0 .3-.8.6-1.7 1.4-3.7 4.4-7.3 7.2-8.7 1.3-.6 1.3-.6-1.1-1.1-5.5-1-8.7-3.5-10.3-8.1-.9-2.6-.9-6.8.1-9.5.4-1.1 1-2.6 1.2-3.4 1.1-3 2.5-4.3 4.7-4.4.7 0 1.2-.1 1.2-.3 0-.5-2.3-3.4-3.9-4.8-.8-.7-2.4-1.9-3.5-2.6-3.1-2-3.5-2.4-3.5-3.4s1-2 2-2c1.6 0 6.5 4.4 8.9 8 .9 1.3 1.6 2.5 1.6 2.6s.5 0 1.1-.4c3.6-2.1 9.7-2.2 13.7-.3.8.4 1.5.7 1.6.7s.6-.7 1.1-1.7c1.8-3.3 6.2-7.7 8.6-8.7 1.1-.5 2.2-.1 2.6.9.6 1.3.1 2-3.1 4.1-3.4 2.3-3.8 2.6-6 5.3-1.7 2.1-2 2.9-.9 2.5 1.5-.5 3.9 1.2 4.7 3.3.2.5.7 1.8 1.1 2.8 1 2.4 1.3 4.2 1.3 6.6 0 7.4-4.2 11.9-12 13-1.3.2-1.3.2.2 1 2.8 1.4 5.7 5 7 8.7.3.8.6 1.5.7 1.5s1.5-.7 3.1-1.5c3.6-2 4.1-2.2 5-1.8.7.3.7.2 1.6-1.4.5-.9 1.7-3.1 2.7-5 2.6-4.7 5.6-9.8 6.4-10.6s1.5-.9 2.3-.3c.5.4.7.2 3.5-4.1 2.5-3.8 3.1-4.5 3.6-4.5.7 0 4.2-2.4 5.3-3.5.3-.3.9-1.2 1.2-1.8.5-1 .7-1.1.8-.7.1.3.1 1.1 0 1.8-.1.8 0 1.3.1 1.3s.7-.4 1.3-1c1.2-1 1.4-.8.7.8-.4 1.1-1.6 1.8-4.9 3.2-2.2 1-2.5 1.2-2.5 1.8 0 .5-1 1.7-3.4 4.4-1.8 2-3.4 3.8-3.4 4 0 .1.2.4.4.6.7.6.5 1.7-.5 2.8-.5.6-3.6 4-7 7.7-5.5 6-6.2 6.8-6.2 7.6 0 1.1-.2 1.4-2.5 3-2.2 1.6-4 2.5-6 3.1-1.5.4-1.5.4-1.6 2.5-.2 2.7-.7 4.4-2.4 8.6-.1.4.1.5 1.2.9 1.4.5 1.4.5 3.1-.8.9-.7 2.6-1.8 3.8-2.4 3.9-2.1 11.1-4.6 12.4-4.3.7.2 2.5 2.1 11.6 13.1 4.4 5.3 6.7 8.2 6.7 8.6 0 .3-.2.9-.4 1.2-.4.5-.2.8 3.2 5.9 2.7 4 3.5 5.5 3.4 5.9-.1.5.2.8 2.3 1.9 1.3.7 3 1.8 3.6 2.4 1 .9 1.1 1.2 1.1 2.1 0 1 0 1-.8 0-1.1-1.3-1.5-1.3-1.4.2.1.7 0 1.4-.2 1.8-.3.6-.4.5-.9-1-.5-1.2-1.1-2-2.5-3.3-1-1-2.4-2-3.1-2.4-1.3-.6-1.3-.7-4.1-6.2-1.6-3.2-3-5.5-3.2-5.5s-.6.1-.9.2c-1.2.4-2.1-.9-9.8-14.7-1.7-3-3.1-5.5-3.1-5.5 0-.1-.5.1-1 .4-1.4.8-12.4 5.5-13.8 5.9-1.8.5-4.5.2-6.8-.8-1.9-.8-1.9-.8-2.8.6-.5.7-1.2 1.7-1.5 2-.6.7-.6.7-.1 2.1.3.8.5 1.9.5 2.5 0 .9.2 1.2 1.6 2.6 1.6 1.6 1.6 1.6 6.2 1.6 3.6 0 5.2.2 7.5.6 6.1 1.3 8.4 2.3 8.4 3.8 0 .3 1.9 6.3 4.3 13.4 2.3 7.1 4.3 13.2 4.3 13.6 0 .7-.4 1.3-1.2 1.6-.1 0 .5 2 1.3 4.4 3.3 9.1 3.4 9.5 3 10-.3.5-.1.7 2.1 2.7 2.6 2.4 3.7 3.9 3.7 5.2 0 1-.4 1.4-.7.6-.2-.7-1.2-1.9-1.5-1.9-.1 0-.2.5-.2 1.2s-.2 1.4-.5 1.8c-.5.6-.5.6-.6-.8-.1-.7-.4-1.7-.6-2.2-.7-1.3-3.2-4.7-4.1-5.5-.7-.6-.9-1.4-2.4-7.3-.9-3.6-1.7-6.9-1.7-7.2-.1-.5-.3-.6-1-.6-1.9 0-2-.7-4.3-16.9-1.3-8.9-1.3-8.9-3.4-9-3.5-.3-12.3-1.8-14.1-2.4-1.9-.6-4.3-2.3-5.6-3.8-.5-.6-.9-1-.9-.9s-.4.6-.8 1.2c-.4.6-1.1 1.4-1.7 1.8-1 .7-1 .7 1 1.2 7.6 1.9 13.5 8.1 15.9 16.5.3 1.2.7 2.8.8 3.5.2 1.3.2 1.3-1.9 1.7-7.9 1.2-22 1.7-30.3 1.1-5-.4-12.7-1.3-13-1.6-.3-.3.7-4.8 1.6-7 2.1-5.3 5.5-9.4 10.1-12.1 1.6-.9 5.7-2.4 6.7-2.4.6 0 .5-.1-.6-1.1-.7-.6-1.5-1.5-1.8-2-.6-.9-.6-.9-2.3.8-1.7 1.6-3.4 2.7-5.3 3.3-1.3.4-12.8 2.2-14.3 2.2-1.2 0-1.3 0-1.4.9-.1.5-.7 4.8-1.4 9.5-1.5 10.6-2.1 14.1-2.6 14.9-.3.5-.7.6-1.4.6-1 0-1 0-2.7 7.1-1.7 7.3-2 8-2.5 8-.4 0-3.3 3.9-4.1 5.5-.3.7-.6 1.7-.6 2.4 0 1.4-.3 1.4-.9.2zm39.9-8.5c-4.3-.4-6.5-.7-9.7-1.3-3.6-.7-3.6-.7-3.9-1.5-2.2-6.1-2.9-9-3.1-13.6-.2-3.5-.2-3.5 1.3-3.4.8.1 3.6.4 6.2.8 4.3.5 5.9.6 15.7.6 9.7 0 11.5-.1 15.6-.6 2.5-.3 5.3-.7 6.1-.8 1.4-.2 1.4-.2 1.3 2.8-.1 1.7-.4 4.3-.7 5.9-.5 2.7-2.3 8.4-2.8 8.9-.3.3-4.7 1.2-8.4 1.7-2.8.4-15 .7-17.5.5z";
        let pencil_path = "m93.4 111.5c-2.2 1.3-4 2.3-4.2 2.3-.1-0-2.4-3.7-5-8.1-2.6-4.5-7.3-12.3-10.3-17.4-3-5.1-7.4-12.5-9.7-16.4s-5.9-9.9-7.9-13.2c-2-3.3-6.8-11.4-10.6-17.9-3.8-6.5-7.7-13-8.5-14.4-.9-1.4-1.5-2.7-1.4-2.8.6-.6 7.5-4.5 7.8-4.5.1 0 1.3 1.9 2.6 4.1 1.3 2.2 3.7 6.2 5.3 9 1.6 2.8 3.9 6.6 5 8.5 1.1 1.9 4.3 7.3 7.1 12 2.8 4.7 6.4 10.7 7.9 13.4 1.6 2.6 6.1 10.2 10.1 16.9s8.9 15 10.9 18.4c2 3.4 3.9 6.6 4.2 7l.5.8-3.9 2.3zm-22.5 13.3c-4 2.3-3.9 2.2-4.2 1.9-.1-.2-.8-1.3-1.6-2.6-1.7-2.8-19.6-33.1-32.3-54.5-17-28.5-19.3-32.6-19.2-32.8.1-.1 1.2-.9 2.6-1.6 1.3-.8 2.8-1.6 3.1-1.9.4-.3.8-.5.8-.5.1 0 1.4 2.2 3 4.8 1.6 2.7 3.5 6 4.3 7.3.8 1.4 2.9 4.9 4.6 7.8 1.7 2.9 7.4 12.5 12.7 21.3 5.2 8.8 10.3 17.3 11.1 18.8s5.3 8.9 9.8 16.5c4.5 7.5 8.1 13.7 8.1 13.8-.1 0-1.3.8-2.9 1.7zm22.4 11.8c-2.8 1.7-5.3 3.1-5.4 3.1-.2-0-15.6-8.7-16.7-9.3-.2-.2 1.1-1.1 4.2-2.9 2.5-1.5 8.6-5.1 13.6-8.1l9-5.3.2 5c.1 2.7.2 7.1.2 9.8l0 4.8-5.2 3.1zm-43.6 32.4c1 .8-1.5 1 23.5-1.8 6.4-.7 14.4-1.6 17.9-2 6.6-.7 7.3-.9 7.8-2.3.3-.9-.2-2.1-1-2.5-.3-.2-2.6-.7-5.1-1.2-2.5-.5-6.9-1.5-9.9-2.1-3-.6-6-1.3-6.8-1.5-.8-.2-1.4-.3-1.3-.4.1-.1 3.8-.5 14.1-1.5 3.5-.4 7.6-.8 9.1-.9 3.2-.3 3.9-.6 4.8-1.5l.6-.6-.3-20.7-.3-20.7-5.7-9.7c-8.3-13.9-13.2-22.2-33.3-56-3.6-6.1-7.1-11.9-7.7-13-.6-1-2.9-4.8-5-8.5C39.2 2.4 38.9 2 37.7 1.3 36.3.4 34 .2 32.4.6 31.1.9 4.5 16.6 3.1 17.8c-1.9 1.7-2.8 4.7-2 7 .2.6 5 8.9 10.7 18.4 5.7 9.5 11.8 19.8 13.6 22.9 1.8 3.1 5.9 9.9 9 15.2s8 13.5 10.8 18.3 6.4 10.7 7.9 13.4c1.6 2.6 4.8 8 7.1 11.9 4.3 7.2 4.3 7.2 5.8 8 3.5 2 14.4 8.1 22.1 12.4 3.3 1.8 5.4 3.1 5.2 3.2-.2.1-1.9.4-3.9.6-2 .2-5.9.7-8.7 1.1-2.8.4-8 1-11.5 1.5-3.5.4-6.6.9-7 .9-1.4.4-2 2.8-.8 3.7.4.4 3.8 1.1 11.8 2.7 6.2 1.2 11.4 2.3 11.6 2.4.2.1-1.7.4-4.2.6-2.5.3-5 .5-5.5.6-.5.1-3.2.3-6.1.6-2.8.3-6.8.7-8.9.9-2 .2-5 .5-6.6.7-4 .4-4.6.8-4.6 2.8 0 .6.2 1 .7 1.4z";
        //spellchecker:disable
        let modifiers_str = format!(
            "
    0\t3\tFlipTile
    0\t4\treplacetilecharacter\t8
    0\t6\treplacetilecharactersvg\t128\t156\t{ant_path}
    0\t12\ticoninsidetile\t104\t170\t{pencil_path}
    "
        );
        //spellchecker:enable

        let modifiers = AllDrawPaperModifiers::from_str(&modifiers_str).unwrap();
        let modifiers = modifiers.modifiers_per_level.get(&0).unwrap();

        let svg = level1.draw_paper_svg(
            "Furniture",
            "",
            &modifiers,
            &crate::special_characters::SpecialCharacters::NONE,
        );

        let svg = svg.replace(
            r#"xmlns="http://www.w3.org/2000/svg""#,
            r#"xmlns="http://www.w3.org/2000/svg" style="background:white""#,
        );

        let path = "furniture_modified.svg";

        match std::panic::catch_unwind(|| {
            insta::assert_snapshot!(svg.clone());
        }) {
            Ok(()) => {
                if !std::fs::exists(path).unwrap() {
                    std::fs::write(path, svg.clone()).unwrap();
                }
            }
            Err(_err) => {
                std::fs::write(path, svg.clone()).unwrap();
                insta::assert_snapshot!(svg);
            }
        }
    }

    #[test]
    pub fn test_paper_svg_hexagon() {
        let level1 = crate::designed_level::DesignedLevel::<19, Hexagon19FatLayout>::from_tsv_line(
            // spellchecker:disable-next-line
            r#"CREHAUSORLADIOMESTR		aroma[You might pick it up at a coffee shop]	choir[Ones who agree with you metaphorically]	Christmas[A famous father]	Carol[Number by a door]	crusade[Campaign religiously]	Treasure[Something found at "X"]	measure[Piano Bar]	medal[Come third or better]	salome[Dancer Of The Seven Veils]	tremor[It's a fault's fault]"#,
            true
        )
        .unwrap();

        let svg = level1.draw_paper_svg(
            "Furniture",
            "",
            &Default::default(),
            &crate::special_characters::SpecialCharacters::NONE,
        );

        let svg = svg.replace(
            r#"xmlns="http://www.w3.org/2000/svg""#,
            r#"xmlns="http://www.w3.org/2000/svg"" style="background:white""#,
        );

        let path = "furniture_hexagon.svg";

        match std::panic::catch_unwind(|| {
            insta::assert_snapshot!(svg.clone());
        }) {
            Ok(()) => {
                if !std::fs::exists("path").unwrap() {
                    std::fs::write(path, svg.clone()).unwrap();
                }
            }
            Err(_err) => {
                std::fs::write(path, svg.clone()).unwrap();
                insta::assert_snapshot!(svg);
            }
        }
    }

    #[test]
    pub fn test_level_difficulty_store() {
        let level1 = crate::designed_level::DesignedLevel::<19, Hexagon19FatLayout>::from_tsv_line(
            // spellchecker:disable-next-line
            r#"CREHAUSORLADIOMESTR		aroma[You might pick it up at a coffee shop]	choir[Ones who agree with you metaphorically]	Christmas[A famous father]	Carol[Number by a door]	crusade[Campaign religiously]	Treasure[Something found at "X"]	measure[Piano Bar]	medal[Come third or better]	salome[Dancer Of The Seven Veils]	tremor[It's a fault's fault]"#,
            true
        )
        .unwrap();

        let score = level1.difficulty_score();

        assert_eq!(score, 0.9383838)
    }

    #[test]
    pub fn test_tsv() {
        let tsv_initial = r#"CREHAUSORLADIOMESTR	My Puzzle[by mark]{#A8436B,#D97882,#EEA5AB}	aroma[You might pick it up at a coffee shop]	choir[Ones who agree with you metaphorically]	Christmas[A famous father]	Carol[Number by a door]	crusade[Campaign religiously]	Treasure[Something found at "X"]	measure[Piano Bar]	medal[Come third or better]	salome[Dancer Of The Seven Veils]	tremor[It's a fault's fault]"#;

        let level = crate::designed_level::DesignedLevel::<19, Hexagon19FatLayout>::from_tsv_line(
            tsv_initial,
            false,
        )
        .unwrap();

        let actual_tsv = level.tsv_line(&level.name, &level.special_characters);

        assert_eq!(tsv_initial, actual_tsv);
    }
}
