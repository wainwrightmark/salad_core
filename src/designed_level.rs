use std::marker::PhantomData;

use crate::{grid_layout::GridLayout, level_trait::LevelTrait, prelude::*};
use base64::DecodeError;
use bevy_color::Srgba;
use itertools::Itertools;
use log::{error, info, warn};
use ustr::Ustr;

#[derive(Debug, Clone, PartialEq)]
pub struct DesignedLevel<const GRID_SIZE: usize, Layout: GridLayout<GRID_SIZE>> {
    pub name: Ustr,
    pub numbering: Option<Numbering>,
    pub special_characters: SpecialCharacters,

    /// Usually attribution
    pub extra_info: Option<Ustr>,
    pub grid: Grid<GRID_SIZE>,
    pub words: Vec<DisplayWord<GRID_SIZE>>,
    pub special_colors: Option<Vec<Srgba>>,
    pub hide_title_when_playing: bool,
    pub phantom: PhantomData<Layout>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Numbering {
    WordSaladNumber(usize),
    SequenceNumber(usize),
    InAttribution(usize),
}

impl<const GRID_SIZE: usize, Layout: GridLayout<GRID_SIZE>> std::fmt::Display
    for DesignedLevel<GRID_SIZE, Layout>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{grid}\t{name}\t{words}",
            grid = self.grid.iter().join(""),
            name = self.name,
            words = self.words.iter().join("\t")
        )
    }
}

impl<const GRID_SIZE: usize, Layout: GridLayout<GRID_SIZE>> DesignedLevel<GRID_SIZE, Layout> {
    /// The name of this level as it should appear in notifications
    pub fn notification_name(&self, include_attribution: bool) -> String {
        let title = match self.numbering {
            Some(Numbering::WordSaladNumber(wsn)) => format!("Word Salad #{wsn}"),
            _ => "Puzzle".to_string(),
        };

        if include_attribution {
            let attribution = match self.extra_info {
                Some(attribution) => format!(" {attribution}"),
                _ => "".to_string(),
            };

            format!("{title}{attribution}")
        } else {
            title
        }
    }

    pub fn try_from_path(mut path: &str, sort_words: bool) -> Option<Self> {
        path = path.trim_start_matches("https://wordsalad.online");
        path = path.trim_start_matches("https://hexagon-salad.netlify.app");

        if path.is_empty() || path.eq_ignore_ascii_case("/") {
            return None;
        }

        if path.to_ascii_lowercase().starts_with("/game/") {
            log::info!("Path starts with game");
            let data = path[6..].to_string();
            log::info!("{data}");

            use base64::Engine;

            let data = match base64::engine::general_purpose::URL_SAFE.decode(&data) {
                Ok(d) => d,
                Err(err) => {
                    if matches!(err, DecodeError::InvalidPadding) {
                        match base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(data) {
                            Ok(d) => d,
                            Err(err) => {
                                error!("{err}");
                                return None;
                            }
                        }
                    } else {
                        error!("{err}");
                        return None;
                    }
                }
            };

            let data = String::from_utf8(data).ok()?;

            info!("Data: {data}");

            match DesignedLevel::from_tsv_line(data.trim(), sort_words) {
                Ok(data) => Some(data),
                Err(err) => {
                    error!("{err}");
                    None
                }
            }
        } else {
            None
        }
    }

    ///The name including the number
    pub fn display_name(&self) -> Ustr {
        match self.numbering {
            Some(Numbering::SequenceNumber(num)) => {
                Ustr::from(format!("{} {num}", self.name).as_str())
            }
            Some(Numbering::WordSaladNumber(num)) => {
                Ustr::from(format!("#{num} {}", self.name).as_str())
            }
            Some(Numbering::InAttribution(_)) => self.name,
            None => self.name,
        }
    }

    pub fn logging_name(&self) -> Ustr {
        match self.numbering {
            Some(Numbering::InAttribution(num)) => {
                Ustr::from(format!("{}: {}", self.name, num).as_str())
            }
            _ => self.display_name(),
        }
    }

    pub fn unknown() -> Self {
        Self {
            name: Ustr::from("Unknown"),
            numbering: None,
            extra_info: None,
            grid: Grid::default(),
            words: vec![],
            special_colors: None,
            hide_title_when_playing: false,
            special_characters: SpecialCharacters::NONE,

            phantom: PhantomData,
        }
    }

    pub fn letter_counts(&self) -> Option<LetterCounts> {
        LetterCounts::try_from_iter(self.grid.iter())
    }

    pub fn from_tsv_line(line: &str, sort_words: bool) -> Result<Self, String> {
        let mut iter = line.trim_matches(char::is_whitespace).split('\t');

        let grid_chars: &str = iter
            .next()
            .ok_or_else(|| format!("Level '{line}' should have a grid"))?;
        let name: &str = iter
            .next()
            .ok_or_else(|| format!("Level '{line}' should have a name"))?;

        let (special_characters, grid_chars) =
            SpecialCharacters::extract_from_grid_characters(grid_chars);

        let grid = try_make_grid(grid_chars)
            .ok_or_else(|| format!("Level '{line}' should be able to make grid"))?;

        let mut words: Vec<DisplayWord<GRID_SIZE>> = iter
            .map(|x| {
                DisplayWord::from_string(x.trim(), &special_characters)
                    .map_err(|e| format!("Word '{x}' is not valid {e}"))
            })
            .try_collect()?;

        if sort_words {
            words.sort();
        }

        let mut name = name;

        let special_colors = if name.ends_with('}') {
            if let Some(index) = name.find('{') {
                let (prefix, colors) = name.split_at(index);
                name = prefix.trim_end();
                let colors = &colors[1..(colors.len() - 1)];
                let mut colors_vec = Vec::<Srgba>::default();

                for c in colors.split(',') {
                    if let Ok(color) = Srgba::hex(c) {
                        colors_vec.push(color);
                    } else {
                        warn!("Could not parse color '{c}'");
                    }
                }
                if colors_vec.is_empty() {
                    None
                } else {
                    Some(colors_vec)
                }
            } else {
                None
            }
        } else {
            None
        };

        let extra_info = if name.ends_with(']') {
            if let Some(index) = name.find('[') {
                let (prefix, extra_info) = name.split_at(index);
                name = prefix.trim_end();
                let extra_info = Ustr::from(&extra_info[1..(extra_info.len() - 1)]);
                Some(extra_info)
            } else {
                None
            }
        } else {
            None
        };

        let hide_title_when_playing: bool;
        if let Some(n) = name.strip_suffix('*') {
            name = n;
            hide_title_when_playing = true;
        } else {
            hide_title_when_playing = false;
        }

        let name = Ustr::from(name.trim_end());

        Ok(Self {
            name,
            numbering: None,
            special_characters,
            extra_info,
            grid,
            words,
            special_colors,
            hide_title_when_playing,

            phantom: PhantomData,
        })
    }
}

impl<const GRID_SIZE: usize, Layout: GridLayout<GRID_SIZE>> LevelTrait<GRID_SIZE>
    for DesignedLevel<GRID_SIZE, Layout>
{
    type Word = DisplayWord<GRID_SIZE>;
    type Layout = Layout;

    fn grid(&self) -> Grid<GRID_SIZE> {
        self.grid
    }

    fn grid_mut(&mut self) -> &mut Grid<GRID_SIZE> {
        &mut self.grid
    }

    fn words(&self) -> &[Self::Word] {
        self.words.as_slice()
    }

    fn extra_info(&self) -> Option<Ustr> {
        self.extra_info
    }

    fn special_colors(&self) -> Option<&[bevy_color::prelude::Srgba]> {
        match &self.special_colors {
            Some(sc) => Some(sc.as_slice()),
            None => None,
        }
    }
}
