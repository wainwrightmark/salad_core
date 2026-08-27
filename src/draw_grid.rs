use bevy_color::prelude::Srgba;
use glam::{Vec2, prelude::FloatExt};

use crate::{
    grid_layout::{GridLayout, TileShape}, level_trait::LevelTrait, line_segment::make_line_segments, prelude::Solution, special_characters::SpecialCharacters, svg_hexagon::{get_hexagon_points_flat_top, get_hexagon_points_pointy_top},    
};

const RECT_SIZE: f32 = 100.0;    

pub(crate) fn draw<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
    level: &impl LevelTrait<GRID_SIZE>,
    special_characters: &SpecialCharacters,
    solution: &Solution<GRID_SIZE>,
    word_line_colors: &[Srgba]
) -> String {   
    
    ///Ratio of the small rect space to small rect size
    
    const TILE_FONT_SIZE: f32 = 40.0;   

    use std::fmt::Write;
    let mut svg = String::new();

    let board_dimensions = LAYOUT::board_dimensions(RECT_SIZE * 0.5);
    let width = board_dimensions.x ;
    let height = board_dimensions.y;


    writeln!(
        &mut svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}"  >"#
    )
    .unwrap();

    const MONTSERRAT: &str = "Montserrat";
    //const MONTSERRAT_ALTERNATES: &'static str = "Montserrat Alternates";
    
    const INNER_BORDER_COLOR: &str = "#221f21";
    const OUTER_BORDER_COLOR: &str = "#221f21";

    const TILE_TEXT_COLOR: &str = "#6b6e71";


    match LAYOUT::TILE_SHAPE {
        TileShape::Square => {
            //outer rectangle
            writeln!(&mut svg, r##"<rect width="{w}" height="{h}" x="{}" y="{}"  rx="0" stroke="{OUTER_BORDER_COLOR}" stroke-width="3" fill="transparent" style="fill:none"> </rect>"##, 
        
        0.0,
        0.0,
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

    

    for (tile, _rune) in level.grid().enumerate() {        

        //x and y are the coordinates of the top left of this square
        let Vec2 { x: top_left_x, y: top_left_y } = LAYOUT::tile_position(tile, RECT_SIZE, false);

        let Vec2 {
            x: center_x,
            y: center_y,
        } = LAYOUT::tile_position(tile, RECT_SIZE, true);

        match LAYOUT::TILE_SHAPE {
            crate::grid_layout::TileShape::Square => {
                writeln!(&mut svg, r##"<rect width="{RECT_SIZE}" height="{RECT_SIZE}" x="{top_left_x}" y="{top_left_y}"  rx="0" stroke="{INNER_BORDER_COLOR}" stroke-width="2" fill="transparent" style="fill:none"> </rect>"##, ).unwrap();
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
    }

    if !solution.is_empty(){
        draw_word_line::<GRID_SIZE, LAYOUT>(&mut svg,  solution, word_line_colors).unwrap();
    }    
    
    // Tile Texts
    for (tile, rune) in level.grid().enumerate() {

        let Vec2 {
            x: center_x,
            y: center_y,
        } = LAYOUT::tile_position(tile, RECT_SIZE, true);

        let font_family = MONTSERRAT;
        let tile_string = rune.to_tile_string(special_characters).to_string();
        let scale_string = "";      

        writeln!(
            &mut svg,
            r##"<text transform="translate({center_x},{center_y}) {scale_string}" font-size="{TILE_FONT_SIZE}" dominant-baseline="central" fill="{TILE_TEXT_COLOR}" font-family="{font_family}" font-weight="bold" style="text-align: center; text-anchor: middle;">{text} </text>"##,
            
            text=tile_string
            ).unwrap();        
    }

    

    svg.push_str("</svg>");

    svg
}


fn draw_word_line<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
    writer: &mut impl std::fmt::Write,
    solution: &Solution<GRID_SIZE>,
    word_line_colors: &[Srgba]
) -> std::fmt::Result {
    writeln! {writer,r##"<g>"##}?;

    let segments = make_line_segments::<GRID_SIZE, LAYOUT>(solution.clone());
    let total_segment_length = solution.len() as f32;

    for segment in segments {
        let color = if word_line_colors.is_empty() {Srgba::BLACK} else{word_line_colors[segment.index % word_line_colors.len()]} ;
        let color = color.to_hex();

        let Vec2 { x: x1, y: y1 } = LAYOUT::tile_position(segment.from, RECT_SIZE, true);

        let Vec2 { x: x2, y: y2 } = LAYOUT::tile_position(segment.to, RECT_SIZE, true);

        let this_segment_length = {
            let this_index = segment.index as f32;
            (total_segment_length - this_index - 1.0).clamp(0.0, 1.0)
        };

        let x2 = x1.lerp(x2, this_segment_length);

        let y2 = y1.lerp(y2, this_segment_length);
        let stroke_width = RECT_SIZE * 0.65;

        let segment_opacity = {
            if segment.index == 0 {
                1.0
            } else {
                (this_segment_length * 2.0).clamp(0.0, 1.0)
            }
        };

        writeln! {writer,
            r##"<line
	x1="{x1}"
	y1="{y1}"
	x2="{x2}"
	y2="{y2}"
	opacity="{segment_opacity}"
	visibility="visible"
	stroke={color}
	stroke-linecap="round"
	stroke-width="{stroke_width}"
	pointer-events="none" >
	</line>"##
        }?;
    }

    writeln! {writer,r##"</g>"##}?;

    Ok(())
}