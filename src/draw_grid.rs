use glam::Vec2;

use crate::{
    grid_layout::{GridLayout, TileShape},
    level_trait::LevelTrait,
    
    special_characters::SpecialCharacters,
    svg_hexagon::{get_hexagon_points_flat_top, get_hexagon_points_pointy_top},    
};

pub(crate) fn draw<const GRID_SIZE: usize, LAYOUT: GridLayout<GRID_SIZE>>(
    self1: &impl LevelTrait<GRID_SIZE>,
    special_characters: &SpecialCharacters,
) -> String {
    

    const RECT_SIZE: f32 = 100.0;
    
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

    //Tiles

    for (tile, rune) in self1.grid().enumerate() {
        

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
