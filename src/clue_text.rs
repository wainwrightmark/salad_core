use std::{
    fmt::{Display, Write},
    num::NonZeroUsize,
    str::FromStr,
};

use crate::{icon_paths::IconPathData, prelude::BasicWordTrait};
use const_sized_bit_set::prelude::{BitSet, BitSet32};
use serde::{Deserialize, Serialize};
use simple_color::Srgba;
use strum::EnumIs;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClueText {
    pub segments: Vec<ClueTextSegment>,
}

impl ClueText {
    pub fn referenced_answers(&self, clue_index: usize, num_clues: usize) -> BitSet32 {
        let mut set = BitSet32::EMPTY;
        for segment in self.segments.iter() {
            match segment {
                ClueTextSegment::TextModifier(TextModifier::AnswersBlock) => {
                    let other_clues = BitSet32::from_first_n_const(num_clues as u32)
                        .with_removed(clue_index as u32);
                    set.union_with_const(&other_clues);
                }
                ClueTextSegment::AnswerNumber(number) => {
                    if let Some(actual_number) = number.get().checked_sub(1) {
                        set.insert_const(actual_number as u32);
                    }
                }
                _ => {}
            }
        }
        set
    }

    pub fn split_into_lines<W: BasicWordTrait>(
        &self,
        max_line_length: usize,
        solved_clues: BitSet32,
        clue_words: &[W],
    ) -> Vec<Vec<ClueTextSegment>> {
        if !self.segments.iter().any(|x| x.is_new_line()) {
            let total_characters: usize = self
                .segments
                .iter()
                .map(|x| x.num_chars(solved_clues, clue_words))
                .sum();
            if total_characters <= max_line_length {
                return vec![self.segments.clone()];
            }
        }

        let mut new_lines = vec![];
        let mut current_line_segments = vec![];
        let mut current_line_characters = 0;

        let mut segments_iter = self.segments.clone().into_iter().peekable();

        while let Some(clue_text_segment) = segments_iter.peek_mut() {
            if clue_text_segment.is_new_line() {
                // leptos::logging::log!("New line detected");
                new_lines.push(std::mem::take(&mut current_line_segments));
                current_line_characters = 0;
                segments_iter.next().unwrap();
                continue;
            }

            let segment_chars = clue_text_segment.num_chars(solved_clues, clue_words);
            if current_line_characters + segment_chars <= max_line_length {
                current_line_segments.push(segments_iter.next().unwrap());
                current_line_characters += segment_chars;
                continue;
            }

            match clue_text_segment
                .try_remove_first_words(max_line_length - current_line_characters)
            {
                Some(extracted_word) => {
                    println!("Appended word: '{}'", extracted_word.to_string());
                    current_line_segments.push(extracted_word);
                }
                None => {}
            }

            new_lines.push(std::mem::take(&mut current_line_segments));
            current_line_characters = 0;
        }

        if current_line_segments.len() == 0 {
            // do nothing
        } else if current_line_segments.len() == 1
            && current_line_segments
                .last()
                .is_some_and(|x| x.is_word_length())
        {
            //special case to prevent word lengths being orphaned
            if let Some(last_line) = new_lines.last_mut() {
                last_line.push(current_line_segments.pop().unwrap());
            }
        } else {
            new_lines.push(current_line_segments);
        }

        for line in new_lines.iter_mut() {
            while line.last() == Some(&ClueTextSegment::Space) {
                let _ = line.pop();
            }
            while line.first() == Some(&ClueTextSegment::Space) {
                line.remove(0);
            }
        }

        // leptos::logging::log!("{new_lines:?}");

        new_lines
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumIs)]
pub enum ClueTextSegment {
    // Answers,
    AnswerNumber(NonZeroUsize),
    Text(String),
    Space,
    TextModifier(TextModifier),

    WordLength(String),

    NewLine,
    Icon { name_lower: String },
}

impl ClueTextSegment {
    pub fn num_chars<W: BasicWordTrait>(&self, solved_clues: BitSet32, clue_words: &[W]) -> usize {
        match self {
            ClueTextSegment::AnswerNumber(n) => {
                let index = n.get().saturating_sub(1);
                if solved_clues.contains_const(index as u32) {
                    clue_words
                        .get(index)
                        .map(|word| word.text().len())
                        .unwrap_or_default()
                } else {
                    if n.get() >= 10 { 2 } else { 1 } //todo in theory this could be even bigger but not realistically
                }
            }
            ClueTextSegment::Text(text) => text.len(),
            ClueTextSegment::Space => 1,
            ClueTextSegment::TextModifier(_) => 0,
            ClueTextSegment::WordLength(w) => w.len(),
            ClueTextSegment::NewLine => 0,
            ClueTextSegment::Icon { .. } => 1,
        }
    }

    ///Split off the first word if possible
    pub fn try_remove_first_words(&mut self, max_first_segment_len: usize) -> Option<Self> {
        match self {
            ClueTextSegment::AnswerNumber(_) => None,
            ClueTextSegment::Space => None,
            ClueTextSegment::Text(text) => {
                let (_, (char_index, _)) = text
                    .char_indices()
                    .enumerate()
                    .filter(|(_total_chars, (_char_index, char))| char.is_whitespace())
                    .take_while(|(total_chars, (_char_index, _char))| {
                        *total_chars <= max_first_segment_len
                    })
                    .last()?;

                let (l, r) = text.split_at(char_index);

                let result = Some(Self::Text(l.to_string()));
                *text = r.to_string();
                result
            }
            ClueTextSegment::TextModifier(_) => None,
            ClueTextSegment::WordLength(_) => None,
            ClueTextSegment::NewLine => None,
            ClueTextSegment::Icon { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextModifier {
    // Normal,
    Bold,
    Italic,
    AnswersBlock,
    ColorAccent,
}

const BOLD_CONTROL_CHARACTER: char = 'b';
const BOLD_CONTROL_CHARACTER_STR: &str = "b";
const ITALICS_CONTROL_CHARACTER: char = 'i';
const ITALICS_CONTROL_CHARACTER_STR: &str = "i";
const ANSWERS_BLOCK_CONTROL_CHARACTER: char = 'a';
const ANSWERS_BLOCK_CONTROL_CHARACTER_STR: &str = "a";

const COLOR_ACCENT_CONTROL_CHARACTER: char = 'c';
const COLOR_ACCENT_CONTROL_CHARACTER_STR: &str = "c";

const NEWLINE_CHARACTER: char = 'n';
const NEWLINE_CHARACTER_STR: &str = "n";

const ICON_CONTROL_CHARACTER: char = ':';
const ICON_CONTROL_CHARACTER_STR: &str = ":";

impl Display for ClueTextSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClueTextSegment::NewLine => {
                f.write_char('\\')?;
                f.write_char(NEWLINE_CHARACTER)
            }

            ClueTextSegment::WordLength(wl) => wl.fmt(f),
            ClueTextSegment::AnswerNumber(n) => {
                f.write_char('\\')?;
                n.get().fmt(f)
            }
            ClueTextSegment::Icon { name_lower } => {
                f.write_char('\\')?;
                f.write_char(ICON_CONTROL_CHARACTER)?;
                f.write_str(&name_lower)
            }
            ClueTextSegment::Text(text) => text.fmt(f),
            ClueTextSegment::Space => f.write_char(' '),
            ClueTextSegment::TextModifier(modifier) => {
                let c = match modifier {
                    TextModifier::Bold => BOLD_CONTROL_CHARACTER,
                    TextModifier::Italic => ITALICS_CONTROL_CHARACTER,
                    TextModifier::AnswersBlock => ANSWERS_BLOCK_CONTROL_CHARACTER,
                    TextModifier::ColorAccent => COLOR_ACCENT_CONTROL_CHARACTER,
                };
                f.write_char('\\')?;
                f.write_char(c)
            }
        }
    }
}

impl Display for ClueText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in self.segments.iter() {
            segment.fmt(f)?;
        }
        Ok(())
    }
}

impl FromStr for ClueText {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let mut remaining_slice = s;

        //let mut lines: Vec<ClueTextLine> = vec![];
        let mut segments = vec![];

        loop {
            let Some((left, right)) = remaining_slice.split_once('\\') else {
                if !remaining_slice.is_empty() {
                    segments.push(ClueTextSegment::Text(remaining_slice.to_string()));
                }

                break;
            };

            if !left.is_empty() {
                if left.chars().all(|x| x.is_whitespace()) {
                    segments.push(ClueTextSegment::Space);
                } else {
                    segments.push(ClueTextSegment::Text(left.to_string()));
                }
            }

            if right.is_empty() {
                anyhow::bail!("Missing Control Character.");
            }

            let c = &right[0..1];
            remaining_slice = &right[1..];
            //let mut r =

            match c {
                NEWLINE_CHARACTER_STR => segments.push(ClueTextSegment::NewLine),
                BOLD_CONTROL_CHARACTER_STR => {
                    segments.push(ClueTextSegment::TextModifier(TextModifier::Bold))
                }
                ITALICS_CONTROL_CHARACTER_STR => {
                    segments.push(ClueTextSegment::TextModifier(TextModifier::Italic))
                }
                ANSWERS_BLOCK_CONTROL_CHARACTER_STR => {
                    segments.push(ClueTextSegment::TextModifier(TextModifier::AnswersBlock))
                }
                COLOR_ACCENT_CONTROL_CHARACTER_STR => {
                    segments.push(ClueTextSegment::TextModifier(TextModifier::ColorAccent))
                }
                ICON_CONTROL_CHARACTER_STR => {
                    let (icon_name, remainder) = match remaining_slice.split_once(" ") {
                        Some((l, r)) => (l, r),
                        None => (remaining_slice, ""),
                    };
                    remaining_slice = remainder;

                    segments.push(ClueTextSegment::Icon {
                        name_lower: icon_name.to_lowercase(),
                    });
                }

                x => {
                    if let Ok(first_digit) = x.parse::<usize>() {
                        let mut total = first_digit;
                        for next_digit in remaining_slice.chars().map_while(|x| x.to_digit(10)) {
                            total *= 10;
                            total += next_digit as usize;
                        }
                        remaining_slice =
                            remaining_slice.trim_start_matches(|c: char| c.is_digit(10));

                        match NonZeroUsize::new(total) {
                            Some(x) => segments.push(ClueTextSegment::AnswerNumber(x)),
                            None => segments.push(ClueTextSegment::AnswerNumber(
                                NonZeroUsize::MIN.saturating_add(9),
                            )),
                        }
                    } else {
                        anyhow::bail!("'\\{x}' is not recognised as a control character.");
                    }
                }
            }
        }
        // if !segments.is_empty() {
        //     lines.push(ClueTextLine(segments));
        // }

        //leptos::logging::log!("Clue segments: {segments:?}");

        return Ok(Self { segments });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClueTextSVGArgs {
    /// Position of top left
    pub x: f32,
    /// Position of top left
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub font_size: f32,
    pub font_family: &'static str,
    pub icons_font_family: &'static str,
    pub fill: Srgba,
    pub special_text_fill: Srgba,
    pub word_length_fill: Srgba,
}

impl ClueText {
    pub fn push_segment(&mut self, segment: ClueTextSegment) {
        self.segments.push(segment);
    }

    pub fn write_svg_element_from_lines<W: crate::prelude::BasicWordTrait>(
        lines: Vec<Vec<ClueTextSegment>>,
        args: ClueTextSVGArgs,
        solved_clues: BitSet32,
        clue_words: &[W],
        icon_paths: &IconPathData,
    ) -> String {
        let ClueTextSVGArgs {
            x,
            y,
            width,
            height,
            font_size,
            font_family,
            icons_font_family,
            fill,
            special_text_fill,
            word_length_fill,
        } = args;

        let mut svg = String::new();
        let center_x = x + (width * 0.5); //

        let fill = fill.to_hex();
        let word_length_fill = word_length_fill.to_hex();
        let special_text_fill = special_text_fill.to_hex();

        //Note: use write! not writeln! as we are white-space:break-spaces
        write!(
            &mut svg,

                r#"<text font-size=0 font-family={font_family} dominant-baseline="central" style="text-anchor: middle; white-space:break-spaces;">"#

        ).unwrap();

        let mut is_bold = false;
        let mut is_italic = false;
        let mut is_answers = false;
        let mut is_color_accent = false;

        for (line_index, line) in lines.iter().enumerate() {
            let numerator = line_index + 1;
            let denominator = lines.len() + 1;
            let line_y = ((numerator as f32 * height) / (denominator as f32)) + y;

            write!(
                &mut svg,
                r#"<tspan x={center_x:.1} y={line_y:.1} dominant-baseline="central">"#
            )
            .unwrap();
            for segment in line.iter() {
                match segment {
                    ClueTextSegment::NewLine => {
                        //Just skip
                    }

                    ClueTextSegment::AnswerNumber(answer_number) => {
                        let index = answer_number.get().saturating_sub(1);
                        if solved_clues.contains_const(index as u32) {
                            let word_text = clue_words
                                .get(index)
                                .map(|x| x.text().to_uppercase())
                                .unwrap_or_default();

                            write!(&mut svg, r#"<tspan font-size={font_size} font-weight="800" fill={special_text_fill} dominant-baseline="central">{word_text}</tspan>"#).unwrap();
                        } else {
                            write!(&mut svg, r#"<tspan font-size={font_size} font-weight="800" fill={special_text_fill} dominant-baseline="central">{answer_number}</tspan>"#).unwrap();
                        }
                    }

                    ClueTextSegment::Icon { name_lower } => {
                        let (weight, font_style, fill) = if is_answers {
                            (800, "normal", &special_text_fill)
                        } else {
                            let weight = if is_bold { 700 } else { 600 };
                            let font_style = if is_italic { "italic" } else { "normal" };
                            let fill = if is_color_accent {
                                &special_text_fill
                            } else {
                                &fill
                            };

                            (weight, font_style, fill)
                        };
                        match icon_paths.get_icon_character(name_lower) {
                            Some(char) => {
                                write!(&mut svg, r#"<tspan font-family="{icons_font_family}" font-size={font_size} font-weight="{weight}" font-style="{font_style}" fill={fill} dominant-baseline="central">{char}</tspan>"#).unwrap();
                            }
                            None => {
                                write!(&mut svg, r#"<tspan font-size={font_size} font-weight="{weight}" font-style="{font_style}" fill={fill} dominant-baseline="central">{name_lower}</tspan>"#).unwrap();
                            }
                        }
                    }

                    ClueTextSegment::Text(text) => {
                        let (weight, font_style, fill) = if is_answers {
                            (800, "normal", &special_text_fill)
                        } else {
                            let weight = if is_bold { 700 } else { 600 };
                            let font_style = if is_italic { "italic" } else { "normal" };
                            let fill = if is_color_accent {
                                &special_text_fill
                            } else {
                                &fill
                            };

                            (weight, font_style, fill)
                        };

                        write!(&mut svg, r#"<tspan font-size={font_size} font-weight="{weight}" font-style="{font_style}" fill={fill} dominant-baseline="central">{text}</tspan>"#).unwrap();
                    }
                    ClueTextSegment::Space => {
                        let (weight, font_style, fill) = if is_answers {
                            (800, "normal", &special_text_fill)
                        } else {
                            let weight = if is_bold { 700 } else { 600 };
                            let font_style = if is_italic { "italic" } else { "normal" };
                            let fill = if is_color_accent {
                                &special_text_fill
                            } else {
                                &fill
                            };

                            (weight, font_style, fill)
                        };

                        write!(&mut svg, r#"<tspan font-size={font_size} font-weight="{weight}" font-style="{font_style}" fill={fill} dominant-baseline="central">&nbsp;</tspan>"#).unwrap();
                    }

                    ClueTextSegment::TextModifier(text_modifier) => match text_modifier {
                        TextModifier::Bold => is_bold = !is_bold,
                        TextModifier::Italic => is_italic = !is_italic,
                        TextModifier::ColorAccent => is_color_accent = !is_color_accent,

                        TextModifier::AnswersBlock => is_answers = !is_answers,
                    },
                    ClueTextSegment::WordLength(wl) => {
                        let smaller_font_size = (font_size * 15.0 / 18.0).round();

                        write!(&mut svg, r#"<tspan font-size={smaller_font_size} font-weight="400" font-style="normal" fill={word_length_fill} dominant-baseline="central">{wl}</tspan>"#).unwrap();
                    }
                }
            }

            write!(&mut svg, "</tspan>").unwrap();
        }

        write!(&mut svg, "</text>").unwrap();

        svg
    }

    pub fn write_svg_element<W: crate::prelude::BasicWordTrait>(
        &self,
        args: ClueTextSVGArgs,
        solved_clues: BitSet32,
        clue_words: &[W],
        icon_paths: &IconPathData,
    ) -> String {
        const MAX_LINE_LENGTH: usize = 30;
        let lines = self.split_into_lines(MAX_LINE_LENGTH, solved_clues, clue_words);

        Self::write_svg_element_from_lines(lines, args, solved_clues, clue_words, icon_paths)
    }
}

#[cfg(test)]
mod tests {

    use std::{str::FromStr, sync::LazyLock};

    use const_sized_bit_set::prelude::BitSet32;
    use test_case::test_case;

    use crate::{clue_text::ClueText, icon_paths::IconPathData, prelude::DisplayWord};

    #[test_case("Hello World")]
    #[test_case(r#"Hello \1 World"#)]
    #[test_case(r#"Hello \2 World"#)]
    #[test_case(r#"Hello \10 World"#)]
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
    fn test_clues_svg() {
        let text = r#"Hello World \i italics \b bold \a ans \1 number \:sun \:sun sun"#;
        let mut clue = ClueText::from_str(text).unwrap();
        clue.push_segment(super::ClueTextSegment::WordLength("(5,6)".to_string()));

        let args = super::ClueTextSVGArgs {
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 50.0,
            font_size: 12.0,
            font_family: "montserrat",
            icons_font_family: "gameicons",
            fill: simple_color::Srgba::BLACK,
            special_text_fill: simple_color::Srgba::BLUE,
            word_length_fill: simple_color::Srgba::rgb_u8(127, 127, 127),
        };

        let icons_paths = IconPathData::new(LazyLock::new(|| "sun".lines().collect()));

        let svg =
            clue.write_svg_element::<DisplayWord<19>>(args, BitSet32::EMPTY, &[], &icons_paths);

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

    // #[test]
    // fn test_line_split() {
    //     let text = r#"Hello World \i italics \b bold \a answer \1 number"#;

    //     let clue = ClueText::from_str(text).unwrap();
    //     let clue = clue.try_split_lines(30, 2).unwrap();

    //     let expected = r#"Hello World \i italics \b bold \n\a answer \1 number"#;

    //     assert_eq!(expected, clue.to_string());
    // }

    // #[test]
    // fn test_line_split2() {
    //     let text = r#"You might pick it up at a coffee shop"#;

    //     let mut clue = ClueText::from_str(text).unwrap();
    //     clue.push_segment(crate::clue_text::ClueTextSegment::WordLength(format!(
    //         "(5)"
    //     )));
    //     let clue = clue.try_split_lines(30, 2).unwrap();

    //     let expected = r#"You might pick it up at a\n coffee shop(5)"#;

    //     assert_eq!(expected, clue.to_string());
    // }
}
