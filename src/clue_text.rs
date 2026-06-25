use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use bevy_color::prelude::Srgba;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClueText {
    pub lines: Vec<ClueTextLine> 
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClueTextLine(Vec<ClueTextSegment>);

#[derive(Debug, Clone, PartialEq)]
pub enum ClueTextSegment {
    Answers,
    AnswerNumber(usize),
    Text(String),
    TextModifier(TextModifier),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextModifier {
    // Normal,
    Bold,
    Italic,
}

// const NORMAL_CONTROL_CHARACTER: char = 'r';
// const NORMAL_CONTROL_CHARACTER_STR: &str = "r";
const BOLD_CONTROL_CHARACTER: char = 'b';
const BOLD_CONTROL_CHARACTER_STR: &str = "b";
const ITALICS_CONTROL_CHARACTER: char = 'i';
const ITALICS_CONTROL_CHARACTER_STR: &str = "i";

impl Display for ClueTextSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            
            ClueTextSegment::Answers => f.write_str("\\a"),
            ClueTextSegment::AnswerNumber(n) => {
                let n = n % 10;
                f.write_char('\\')?;
                n.fmt(f)
            }
            ClueTextSegment::Text(ustr) => ustr.fmt(f),
            ClueTextSegment::TextModifier(modifier) => {
                let c = match modifier {
                    // TextModifier::Normal => NORMAL_CONTROL_CHARACTER,
                    TextModifier::Bold => BOLD_CONTROL_CHARACTER,
                    TextModifier::Italic => ITALICS_CONTROL_CHARACTER,
                };
                f.write_char('\\')?;
                f.write_char(c)
            }
        }
    }
}

impl Display for ClueText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for (index, line) in self.lines.iter().enumerate(){
            if index != 0{
                f.write_str("\\n")?;
            }
            for segment in line.0.iter(){
                segment.fmt(f)?;
            }
        }
     Ok(())   
    }
}

impl FromStr for ClueText {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        
        

        let mut remaining_slice = s;

        let mut lines: Vec<ClueTextLine> = vec![];
        let mut segments = vec![];

        loop {
            let Some((left, right)) = remaining_slice.split_once('\\') else {
                if !remaining_slice.is_empty() {
                    segments.push(ClueTextSegment::Text(remaining_slice.to_string()));
                }

                break;
            };

            if !left.is_empty() {
                segments.push(ClueTextSegment::Text(left.to_string()));
            }

            let c = &right[0..1];
            let r = &right[1..];

            match c {
                // NORMAL_CONTROL_CHARACTER_STR => {
                //     segments.push(ClueTextSegment::TextModifier(TextModifier::Normal))
                // }
                BOLD_CONTROL_CHARACTER_STR => {
                    segments.push(ClueTextSegment::TextModifier(TextModifier::Bold))
                }
                ITALICS_CONTROL_CHARACTER_STR => {
                    segments.push(ClueTextSegment::TextModifier(TextModifier::Italic))
                }

                "n" => {
                    lines.push(ClueTextLine(std::mem::take(&mut segments) ));
                    
                }

                "a" => {
                    segments.push(ClueTextSegment::Answers);
                }

                "1" => segments.push(ClueTextSegment::AnswerNumber(1)),
                "2" => segments.push(ClueTextSegment::AnswerNumber(2)),
                "3" => segments.push(ClueTextSegment::AnswerNumber(3)),
                "4" => segments.push(ClueTextSegment::AnswerNumber(4)),
                "5" => segments.push(ClueTextSegment::AnswerNumber(5)),
                "6" => segments.push(ClueTextSegment::AnswerNumber(6)),
                "7" => segments.push(ClueTextSegment::AnswerNumber(7)),
                "8" => segments.push(ClueTextSegment::AnswerNumber(8)),
                "9" => segments.push(ClueTextSegment::AnswerNumber(9)),
                "0" => segments.push(ClueTextSegment::AnswerNumber(10)),

                x => {
                    anyhow::bail!("'\\{x}' is not recognised as a control character.");
                }
            }
            remaining_slice = r;
        }
        if !segments.is_empty(){
            lines.push(ClueTextLine(segments));
        }

        return Ok(Self{lines});
    }
}

#[derive(Debug, Clone, Copy, PartialEq,  )]
pub struct ClueTextSVGArgs {
    /// Position of top left
    pub x: f32,
    /// Position of top left
    pub y: f32,
    pub width: f32,
    pub height: f32,    
    pub font_size: f32,
    pub font_family: &'static str,
    pub fill: Srgba,
}

impl ClueText {
    pub fn write_svg_element(&self, args: ClueTextSVGArgs) -> String {
        let ClueTextSVGArgs {
            x,
            y,
            width,
            height,
            font_size,
            font_family,
            fill,
        } = args;

        let mut svg = String::new();
        let center_x = x + (width * 0.5); //

        let fill = fill.to_hex();

        writeln!(
            &mut svg,
            
                r#"<text font-size={font_size} font-family={font_family} fill={fill} dominant-baseline="central" style="text-anchor: middle;">"#
            
        ).unwrap();

        let mut is_bold = false;
            let mut is_italic = false;

        for (line_index, line) in self.lines.iter().enumerate(){
            let numerator = line_index + 1;
            let denominator = self.lines.len() + 1;
            let line_y = ((numerator as f32 * height) / (denominator as f32)) + y;

            

            writeln!(&mut svg, r#"<tspan x={center_x:.1} y={line_y:.1}>"#).unwrap();
            for segment in line.0.iter(){
                match segment{
                    ClueTextSegment::Answers => {
                        writeln!(&mut svg, r#"<tspan font-weight="800">ANSWERS</tspan>"#).unwrap();
                    },
                    ClueTextSegment::AnswerNumber(answer_number) => {
                        writeln!(&mut svg, r#"<tspan font-weight="800">{answer_number}</tspan>"#).unwrap();
                    },
                    ClueTextSegment::Text(ustr) => {
                        let weight = if is_bold {700} else {500};
                        let font_style = if is_italic {"italic"} else{"normal"};

                        writeln!(&mut svg, r#"<tspan font-weight="{weight}" font-style="{font_style}">{ustr}</tspan>"#).unwrap();
                    },
                    ClueTextSegment::TextModifier(text_modifier) => {
                        match text_modifier{
                            TextModifier::Bold => is_bold = !is_bold,
                            TextModifier::Italic => is_italic = !is_italic,
                        }
                    },
                }
            }

            writeln!(&mut svg, "</tspan>").unwrap();

        }


        writeln!(&mut svg, "</text>").unwrap();

        svg
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use test_case::test_case;

    use crate::clue_text::ClueText;

    #[test_case("Hello World")]
    #[test_case(r#"Hello \1 World"#)]
    #[test_case(r#"Hello \2 World"#)]
    #[test_case(r#"Hello \0 World"#)]
    #[test_case(r#"Hello \a World"#)]
    #[test_case(r#"Hello \i World"#)]
    #[test_case(r#"Hello \b World"#)]
    
    #[test_case(r#"Hello World \i italics \b bold \n \a answer \1 number"#)]
    fn test_clue_text_round_trip(text: &str) {
        let clue = ClueText::from_str(text).unwrap();
        let actual = clue.to_string();

        assert_eq!(text, actual)
    }


    #[test]
    fn test_clues_svg(){
        let text = r#"Hello World \i italics \b bold \n \a answer \1 number"#;
        let clue = ClueText::from_str(text).unwrap();

        let args = super::ClueTextSVGArgs { x: 0.0, y: 0.0, width: 200.0, height: 50.0, font_size: 12.0, font_family: "montserrat", fill: bevy_color::prelude::Srgba::BLACK };

        let svg = clue.write_svg_element(args);

        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg"" style=" background:white" viewBox="0 0 200 50">
{svg}            
</svg>"#
        );

        let path = "clue_text.svg";

        match std::panic::catch_unwind(|| {
            insta::assert_snapshot!("test_clues_svg", svg.clone());
        }) {
            Ok(()) => {
                if !std::fs::exists("path").unwrap() {
                    std::fs::write(path, svg.clone()).unwrap();
                }
            }
            Err(_err) => {
                std::fs::write(path, svg.clone()).unwrap();
                insta::assert_snapshot!("test_clues_svg", svg);
            }
        }

        

        
    }
}
