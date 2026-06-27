use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use bevy_color::prelude::Srgba;
use const_sized_bit_set::prelude::BitSet32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClueText {
    pub lines: Vec<ClueTextLine> 
}


impl ClueText{

    pub fn referenced_answers(&self)-> BitSet32{
        let mut set = BitSet32::EMPTY;
        for line in self.lines.iter(){
            for segment in line.0.iter(){
                if let ClueTextSegment::AnswerNumber(number) = segment {
                    set.insert_const(*number as u32);
                }
            }
        }
        set
    }

    pub fn try_split_lines(&self, max_line_length: usize, max_lines: usize)-> Option<Self>{
        let line_count = self.lines.len();
        let longest_line = self.lines.iter().map(|x|x.line_length()).max().unwrap_or_default();

        if longest_line <= max_line_length && line_count <= max_lines{
            
            return Some(self.clone());
        }

        let total_characters: usize = self.lines.iter().map(|x|x.line_length()).sum();
        let max_characters = max_line_length * max_lines;
        if total_characters > max_characters{
            
            return None;
        }

        let mut new_lines = vec![];
        let mut current_line_segments = vec![];
        let mut current_line_characters = 0;
        
        let mut segments_iter = self.lines.clone().into_iter().flat_map(|line|line.0.into_iter()).peekable();

        while let Some(clue_text_segment) = segments_iter.peek_mut() {
            let segment_chars = clue_text_segment.num_chars();
            if current_line_characters + segment_chars <= max_line_length{
                current_line_segments.push(segments_iter.next().unwrap());
                current_line_characters += segment_chars;
                continue;
            }
            
            
            

            match clue_text_segment.try_remove_first_words(max_line_length - current_line_characters){
                Some(extracted_word) => {                    
                    println!("Appended word: '{}'", extracted_word.to_string());
                    current_line_segments.push(extracted_word);                    
                },
                None => {
                    
                },
            }

            

            

            new_lines.push(ClueTextLine(std::mem::take(&mut current_line_segments)));
            if new_lines.len() > max_lines{
                
                return None;
            }
            current_line_characters= 0;
                    
        }

        new_lines.push(ClueTextLine(std::mem::take(&mut current_line_segments)));
        if new_lines.len() > max_lines{
                
                return None;
            }


        Some(Self{
            lines: new_lines
        })    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClueTextLine(Vec<ClueTextSegment>);

impl ClueTextLine{
    pub fn line_length(&self)-> usize{
        self.0.iter().map(|x|x.num_chars()).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClueTextSegment {
    Answers,
    AnswerNumber(usize),
    Text(String),
    TextModifier(TextModifier),

    WordLength(String)
}

impl ClueTextSegment{
    pub fn num_chars(&self)-> usize{
        match self{
            ClueTextSegment::Answers => 7,
            ClueTextSegment::AnswerNumber(n) => if *n == 10 {2} else{1},
            ClueTextSegment::Text(text) => text.len(),
            ClueTextSegment::TextModifier(_) => 0,
            ClueTextSegment::WordLength(w) => w.len(),
        }
    }

    ///Split off the first word if possible
    pub fn try_remove_first_words(&mut self,max_first_segment_len: usize)-> Option<Self>{
        match self{
            ClueTextSegment::Answers => None,
            ClueTextSegment::AnswerNumber(_) => None,
            ClueTextSegment::Text(text) => {

                let (_,(char_index,_)) = text.char_indices().enumerate()
                .filter(|(_total_chars,(_char_index, char))|char.is_whitespace())
                .take_while(|(total_chars,(_char_index, _char))|*total_chars <= max_first_segment_len).last()?;

                let (l, r) = text.split_at(char_index);
                 
                 let result = Some(Self::Text(l.to_string()));
                 *text = r.to_string();
                 result
            },
            ClueTextSegment::TextModifier(_) => None,
            ClueTextSegment::WordLength(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
            ClueTextSegment::WordLength(wl) => wl.fmt(f),
            ClueTextSegment::AnswerNumber(n) => {
                let n = n % 10;
                f.write_char('\\')?;
                n.fmt(f)
            }
            ClueTextSegment::Text(text) => text.fmt(f),
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

    pub fn push_segment(&mut self, segment: ClueTextSegment){
        match self.lines.last_mut() {
            Some(line) => line.0.push(segment),
            None => {
                self.lines.push(ClueTextLine(vec![segment]));
            },
        }
    }

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
                    ClueTextSegment::Text(text) => {
                        let weight = if is_bold {700} else {500};
                        let font_style = if is_italic {"italic"} else{"normal"};

                        writeln!(&mut svg, r#"<tspan font-weight="{weight}" font-style="{font_style}">{text}</tspan>"#).unwrap();
                    },
                    ClueTextSegment::TextModifier(text_modifier) => {
                        match text_modifier{
                            TextModifier::Bold => is_bold = !is_bold,
                            TextModifier::Italic => is_italic = !is_italic,
                        }
                    },
                    ClueTextSegment::WordLength(wl)=>{
                        writeln!(&mut svg, r#"<tspan font-weight="500" font-style="normal">{wl}</tspan>"#).unwrap();
                    }
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



    #[test]
    fn test_line_split(){
        let text = r#"Hello World \i italics \b bold \a answer \1 number"#;

        let clue = ClueText::from_str(text).unwrap();
        let clue =clue.try_split_lines(30, 2).unwrap();

        let expected = r#"Hello World \i italics \b bold \n\a answer \1 number"#;

        assert_eq!(expected, clue.to_string());

    }
    
    #[test]
    fn test_line_split2(){
        let text = r#"You might pick it up at a coffee shop"#;

        let mut clue = ClueText::from_str(text).unwrap();
        clue.push_segment(crate::clue_text::ClueTextSegment::WordLength(format!("(5)")));
        let clue =clue.try_split_lines(30, 2).unwrap();

        let expected = r#"You might pick it up at a\n coffee shop(5)"#;

        assert_eq!(expected, clue.to_string());

    }
}
