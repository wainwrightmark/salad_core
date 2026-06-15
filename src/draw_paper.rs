use std::{collections::BTreeMap, str::FromStr};

use glam::Vec2;
use itertools::Itertools;

use crate::{
    grid_layout::{GridLayout, SQRT_3, TileShape}, level_trait::LevelTrait, prelude::*, special_characters::SpecialCharacters, svg_hexagon::{get_hexagon_points_flat_top, get_hexagon_points_pointy_top}, tile_usages::TileUsages, word_trait::BasicWordTrait
};

#[derive(Debug, Default)]
pub struct AllDrawPaperModifiers {
    pub modifiers_per_level: BTreeMap<usize, DrawPaperModifiers>,
}

impl FromStr for AllDrawPaperModifiers {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modifiers_per_level: BTreeMap<usize, DrawPaperModifiers> = Default::default();

        for mut line in s.lines() {
            line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut splits = line.split('\t');
            let Some(puzzle_number) = splits.next().and_then(|x| x.parse::<usize>().ok()) else {
                anyhow::bail!("Line should start with a number: '{line}'");
            };

            let Some(tile_index) = splits.next().and_then(|x| x.parse::<usize>().ok()) else {
                anyhow::bail!("Line should have a puzzle index: '{line}'");
            };

            let Some(modifier_type) = splits.next().map(|x| x.to_lowercase()) else {
                anyhow::bail!("Line Should have a modifier type: '{line}'");
            };

            let modifier = match modifier_type.as_str() {
                //spellchecker:disable-next-line
                "replacetilecharacter" => {
                    let Some(replacement) = splits.next() else {
                        anyhow::bail!("Expected data in line '{line}'");
                    };
                    DrawPaperTileModifier::ReplaceTileCharacter(replacement.to_string())
                }
                //spellchecker:disable-next-line
                "replacetilecharactersvg" => {
                    let Some(width) = splits.next().and_then(|x| x.parse::<f32>().ok()) else {
                        anyhow::bail!("Expected data in line '{line}'");
                    };
                    let Some(height) = splits.next().and_then(|x| x.parse::<f32>().ok()) else {
                        anyhow::bail!("Expected data in line '{line}'");
                    };
                    let Some(path_d) = splits.next().map(|x| x.to_string()) else {
                        anyhow::bail!("Expected data in line '{line}'");
                    };
                    DrawPaperTileModifier::ReplaceTileCharacterSVG {
                        width,
                        height,
                        path_d,
                    }
                }
                //spellchecker:disable-next-line
                "fliptile" => DrawPaperTileModifier::FlipTile,
                //spellchecker:disable-next-line
                "iconinsidetile" => {
                    let Some(width) = splits.next().and_then(|x| x.parse::<f32>().ok()) else {
                        anyhow::bail!("Expected data in line '{line}'");
                    };
                    let Some(height) = splits.next().and_then(|x| x.parse::<f32>().ok()) else {
                        anyhow::bail!("Expected data in line '{line}'");
                    };
                    let Some(path_d) = splits.next().map(|x| x.to_string()) else {
                        anyhow::bail!("Expected data in line '{line}'");
                    };

                    DrawPaperTileModifier::IconInsideTile {
                        width,
                        height,
                        path_d,
                    }
                }
                _ => {
                    anyhow::bail!("Unrecognized modifier in line '{line}'");
                }
            };

            modifiers_per_level
                .entry(puzzle_number)
                .or_default()
                .tile_modifiers
                .entry(tile_index)
                .or_default()
                .push(modifier);
        }

        Ok(Self {
            modifiers_per_level,
        })
    }
}

#[derive(Debug, Default)]
pub struct DrawPaperModifiers {
    pub tile_modifiers: BTreeMap<usize, Vec<DrawPaperTileModifier>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DrawPaperTileModifier {
    ReplaceTileCharacter(String),
    ReplaceTileCharacterSVG {
        width: f32,
        height: f32,
        path_d: String,
    },
    FlipTile,
    IconInsideTile {
        width: f32,
        height: f32,
        path_d: String,
    },
}

pub(crate) fn draw<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
    self1: &impl LevelTrait<GRID_SIZE>,
    title: &str,
    subtitle: &str,
    modifiers: &DrawPaperModifiers,
    special_characters: &SpecialCharacters,
) -> String {
    const TITLE_FONT_SIZE_BIG: f32 = 22.0;
    const TITLE_FONT_SIZE_SMALL: f32 = 17.6;
    const TILE_FONT_SIZE: f32 = 40.0;
    const HIDDEN_TEXT_FONT_SIZE: f32 = 20.0;

    const RECT_SIZE: f32 = 100.0;
    const SMALL_TO_BIG_RECT_RATIO: f32 = 0.15;
    ///Ratio of the small rect space to small rect size
    const SMALL_RECT_PROPORTION: f32 = 0.8;

    const WORD_LINE_SPACING: f32 = 44.0;
    const GRID_TOP_OFFSET: f32 = 1.1;
    const GRID_WORDS_GAP: f32 = 0.42;

    const SIDE_MARGIN: f32 = 0.5;

    fn xml_escape(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\'', "&apos;")
            .replace('"', "&quot;")
    }

    use std::fmt::Write;
    let mut svg = String::new();

    let board_dimensions = LAYOUT::board_dimensions(RECT_SIZE * 0.5);
    let width = board_dimensions.x + RECT_SIZE;
    let height = (RECT_SIZE * (GRID_TOP_OFFSET + GRID_WORDS_GAP))
        + board_dimensions.y
        + (WORD_LINE_SPACING * self1.words().len() as f32);

    let usages = TileUsages::try_from_level::<LAYOUT>(self1).unwrap_or(TileUsages([0; GRID_SIZE]));

    let title_font_size = if title.len() <= 18 {
        TITLE_FONT_SIZE_BIG
    } else {
        TITLE_FONT_SIZE_SMALL
    };

    writeln!(
        &mut svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}"  >"#
    )
    .unwrap();

    const MONTSERRAT: &str = "Montserrat";
    //const MONTSERRAT_ALTERNATES: &'static str = "Montserrat Alternates";

    const TITLE_COLOR: &str = "#221f21";
    const INNER_BORDER_COLOR: &str = "#221f21";
    const OUTER_BORDER_COLOR: &str = "#221f21";
    const CHECK_BOX_COLOR: &str = "#221f21";

    const TILE_TEXT_COLOR: &str = "#6b6e71";
    const WORD_LENGTH_TEXT_COLOR: &str = "#221f21";

    //Title
    writeln!(
        &mut svg,
        r##"<text x="{x1}" y="{y1}" dominant-baseline="central" fill="{TITLE_COLOR}" font-size="{title_font_size}pt" font-family="{MONTSERRAT}" font-weight="700" style="text-align: left; text-anchor: left;">{} </text>"##,
        
        xml_escape(title),
        x1 = RECT_SIZE * (SIDE_MARGIN + 0.5),
        y1 = RECT_SIZE * 0.22,
    ).unwrap();

    //Subtitle
    if !subtitle.trim().is_empty() {
        writeln!(
        &mut svg,
        r##"<text x="{}" y="{}" dominant-baseline="central" fill="{TITLE_COLOR}" font-size="{TITLE_FONT_SIZE_SMALL}pt" font-family="{MONTSERRAT}" font-weight="500" style="text-align: left; text-anchor: left;">{} </text>"##,
        RECT_SIZE * (SIDE_MARGIN + 0.5 ),
        RECT_SIZE * 0.63,
        xml_escape(subtitle)
    ).unwrap();
    }

    match LAYOUT::TILE_SHAPE {
        TileShape::Square => {
            //outer rectangle
            writeln!(&mut svg, r##"<rect width="{w}" height="{h}" x="{}" y="{}"  rx="0" stroke="{OUTER_BORDER_COLOR}" stroke-width="3" fill="transparent" style="fill:none"> </rect>"##, 
        
        SIDE_MARGIN * RECT_SIZE,
        GRID_TOP_OFFSET * RECT_SIZE,
        w = board_dimensions.x,
        h = board_dimensions.y,
    ).unwrap();
        }
        TileShape::HexagonPointyTop | TileShape::HexagonFlatTop => {
            //actually don't draw anything
            //     let center = Vec2 {
            //         x: SIDE_MARGIN * RECT_SIZE + (board_dimensions.x * 0.5),
            //         y: GRID_TOP_OFFSET * RECT_SIZE + (board_dimensions.x * 0.5) };

            //     let radius = board_dimensions.max_element() * 0.5;

            //     let points = get_hexagon_points_rotated(center, radius);

            //     writeln!(&mut svg, r##"<polygon points="{points}"  stroke="{OUTER_BORDER_COLOR}" stroke-width="3" fill="transparent" style="fill:none"> </polygon>"##

            // ).unwrap();
        }
    }

    //Tiles

    for (tile, rune) in self1.grid().enumerate() {
        let modifiers = modifiers
            .tile_modifiers
            .get(&tile.inner_usize())
            .cloned()
            .unwrap_or_default();

        //x and y are the coordinates of the top left of this square
        let Vec2 { x, y } = LAYOUT::tile_position(tile, RECT_SIZE, false)
            + (Vec2 {
                x: SIDE_MARGIN,
                y: GRID_TOP_OFFSET,
            } * RECT_SIZE);

        let Vec2 {
            x: center_x,
            y: center_y,
        } = LAYOUT::tile_position(tile, RECT_SIZE, true)
            + (Vec2 {
                x: SIDE_MARGIN,
                y: GRID_TOP_OFFSET,
            } * RECT_SIZE);

        match LAYOUT::TILE_SHAPE {
            crate::grid_layout::TileShape::Square => {
                writeln!(&mut svg, r##"<rect width="{RECT_SIZE}" height="{RECT_SIZE}" x="{x}" y="{y}"  rx="0" stroke="{INNER_BORDER_COLOR}" stroke-width="2" fill="transparent" style="fill:none"> </rect>"##, ).unwrap();
            }
            crate::grid_layout::TileShape::HexagonPointyTop => {
                let radius = RECT_SIZE * 0.5;
                let points = get_hexagon_points_pointy_top(
                    Vec2 {
                        x: center_x,
                        y: center_y,
                    },
                    radius,
                );
                writeln!(&mut svg, r##"<polygon points="{points}"  stroke="{INNER_BORDER_COLOR}" stroke-width="2" fill="transparent" style="fill:none"> </polygon>"##).unwrap();
            }
            crate::grid_layout::TileShape::HexagonFlatTop => {
                let radius = RECT_SIZE * 0.5;
                let points = get_hexagon_points_flat_top(
                    Vec2 {
                        x: center_x,
                        y: center_y,
                    },
                    radius,
                );
                writeln!(&mut svg, r##"<polygon points="{points}"  stroke="{INNER_BORDER_COLOR}" stroke-width="2" fill="transparent" style="fill:none"> </polygon>"##).unwrap();
            }
        }

        let font_family = MONTSERRAT;
        let mut tile_string = rune.to_tile_string(special_characters).to_string();
        let mut scale_string = "";

        for modifier in modifiers {
            match modifier {
                DrawPaperTileModifier::ReplaceTileCharacter(replacement) => {
                    tile_string = replacement
                }
                DrawPaperTileModifier::ReplaceTileCharacterSVG {
                    width,
                    height,
                    path_d,
                } => {
                    tile_string = "".to_string();
                    writeln!(&mut svg, r#"<svg viewBox="0 0  {width} {height}" x="{x}" y="{y}" width="{size}" height="{size}"> <path fill="{TILE_TEXT_COLOR}" d="{path_d}"/> </svg>"#,
                        size = RECT_SIZE * 0.5,
                        x = x + (RECT_SIZE * 0.25),
                        y = y + (RECT_SIZE * 0.25),
                    
                    ).unwrap();
                }
                DrawPaperTileModifier::FlipTile => {
                    scale_string = r#"scale (1, -1)"#;
                }
                DrawPaperTileModifier::IconInsideTile {
                    width,
                    height,
                    path_d,
                } => {
                    writeln!(&mut svg, r#"<svg viewBox="0 0  {width} {height}" x="{x}" y="{y}" width="{size}" height="{size}"> <path fill="{TILE_TEXT_COLOR}" d="{path_d}"/> </svg>"#,
                        size = RECT_SIZE * 0.25,
                        x = x + (RECT_SIZE * 0.125),
                        y = y + (RECT_SIZE * 0.575),
                    
                    ).unwrap();
                }
            }
        }

        writeln!(
            &mut svg,
            r##"<text transform="translate({x},{y}) {scale_string}" font-size="{TILE_FONT_SIZE}" dominant-baseline="central" fill="{TILE_TEXT_COLOR}" font-family="{font_family}" font-weight="bold" style="text-align: center; text-anchor: middle;">{text} </text>"##,
            x=center_x,
            y=center_y,
            text=tile_string
            ).unwrap();

        let extra_letters = usages.0[tile.inner_usize()].saturating_sub(1);

        match LAYOUT::TILE_SHAPE {
            crate::grid_layout::TileShape::Square => {
                for l_index in 0..extra_letters {
                    const SMALL_RECT_SIZE: f32 = RECT_SIZE * SMALL_TO_BIG_RECT_RATIO;
                    let x = x + RECT_SIZE - (SMALL_RECT_SIZE);
                    let y = y
                        + (SMALL_RECT_SIZE * (1.0 - SMALL_RECT_PROPORTION))
                        + (l_index as f32 * SMALL_RECT_SIZE);

                    writeln!(&mut svg, r##"<rect width="{size}" height="{size}" x="{x}" y="{y}"  rx="0" stroke="{CHECK_BOX_COLOR}" stroke-width="1" fill="transparent" style="fill:none"> </rect>"##,
                        size = SMALL_RECT_SIZE * SMALL_RECT_PROPORTION
                
                
                ).unwrap();
                }
            }
            TileShape::HexagonPointyTop | TileShape::HexagonFlatTop => {
                //todo this should actually be different for the two different hexagons
                for l_index in 0..extra_letters {
                    const SMALL_HEXAGON_RADIUS: f32 = RECT_SIZE * 0.1;

                    let (steps_right, steps_down) = match l_index {
                        0 => (0, -4),
                        1 => (1, -3),
                        2 => (2, -2),
                        3 => (2, 0),
                        4 => (2, 2),
                        5 => (1, 3),
                        6 => (0, 4),
                        7 => (-1, 3),
                        8 => (-2, 2),
                        9 => (-2, 0),
                        10 => (-2, -2),
                        11 => (-1, -3),
                        12 => (0, -2),
                        13 => (1, -1),
                        14 => (1, 1),
                        15 => (0, 2),
                        16 => (-1, -1),
                        17 => (-1, 1),
                        _ => {
                            continue;
                        }
                    };

                    let x = center_x + (steps_right as f32 * SMALL_HEXAGON_RADIUS * SQRT_3);
                    let y = center_y + (steps_down as f32 * SMALL_HEXAGON_RADIUS);

                    let points = get_hexagon_points_pointy_top(Vec2 { x, y }, SMALL_HEXAGON_RADIUS);

                    writeln!(&mut svg, r##"<polygon points="{points}" stroke="{CHECK_BOX_COLOR}" stroke-width="1" fill="transparent" style="fill:none"> </polygon>"##,                                                               
                    ).unwrap();
                }
            }
        }
    }

    let mut words = self1
        .words()
        .iter()
        .flat_map(|word| DisplayWord::from_string(word.text().as_str(), special_characters).ok())
        .collect_vec();

    words.sort_by_key(|x: &DisplayWord<GRID_SIZE>| (x.characters.len(), x.graphemes.len()));

    const WORD_LINE_BASE: f32 = 66.0;
    const WORD_LINE_PER_LETTER: f32 = 23.0;

    for (index, display_word) in words.into_iter().enumerate() {
        let y = (RECT_SIZE * (GRID_TOP_OFFSET + GRID_WORDS_GAP))
            + board_dimensions.y
            + (WORD_LINE_SPACING * index as f32); // + 10.0;

        let hidden_text = display_word.hidden_text;

        writeln!(
                &mut svg,
                r##"<text x="{}" y="{}" font-size="{HIDDEN_TEXT_FONT_SIZE}"  dominant-baseline="central" fill="{WORD_LENGTH_TEXT_COLOR}" font-family="{MONTSERRAT}" font-weight="500" style="text-align: left; text-anchor: left;">{} </text>"##,
                (SIDE_MARGIN + 0.5) * RECT_SIZE,
                y,
                hidden_text
            )
            .unwrap();

        //todo draw a line here

        const X1: f32 = RECT_SIZE * (0.5 + SIDE_MARGIN);
        let x2: f32 =
            X1 + (WORD_LINE_BASE + (display_word.characters.len() as f32 * WORD_LINE_PER_LETTER));

        let line_y = y + (WORD_LINE_SPACING * 0.5);

        writeln!(&mut svg,
                r##"<line x1="{X1}" x2="{x2}" y1="{line_y}" y2="{line_y}"  stroke="black" stroke-width="1"/>"##
            ).unwrap();
    }

    svg.push_str("</svg>");

    svg
}
