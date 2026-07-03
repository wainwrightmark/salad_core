use crate::{
    character::normalize_characters_array,
    prelude::*,
    word_trait::{BasicWordTrait, WordTrait},
};
use itertools::Itertools;
use std::{collections::BTreeMap, sync::LazyLock};
use ustr::Ustr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayWord<const GRID_SIZE: usize> {
    /// The characters needed to solve the word
    pub characters: CharsArray<GRID_SIZE>,
    /// The final display text of the word
    pub text: Ustr,
    /// The text when the word is hidden
    pub hidden_text: Ustr,
    /// The graphemes - used for partially hiding the word
    pub graphemes: Vec<CharGrapheme>,

    /// Should this word start off revealed
    pub show_initial: bool,

    /// Question text for quiz salad
    pub question_text: Option<Ustr>,
}

impl<const GRID_SIZE: usize> std::fmt::Display for DisplayWord<GRID_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.text.fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharGrapheme {
    pub is_game_char: bool,
    pub grapheme: Ustr,
}

impl<const GRID_SIZE: usize> BasicWordTrait for DisplayWord<GRID_SIZE> {
    fn text(&self) -> Ustr {
        self.text
    }

    fn characters_slice(&self) -> &[Character] {
        &self.characters
    }
}

impl<const GRID_SIZE: usize> WordTrait<GRID_SIZE> for DisplayWord<GRID_SIZE> {
    fn characters(&self) -> &CharsArray<GRID_SIZE> {
        &self.characters
    }

    fn quiz_question(&self) -> Option<Ustr> {
        self.question_text
    }
}

static COLORS_MAP: LazyLock<BTreeMap<&[Character], bevy_color::Srgba>> = LazyLock::new(|| {
    let pairs: &[(&'static str, u8, u8, u8)] = &[
        ("Aquamarine", 0x7f, 0xff, 0xd4),
        ("Azure", 0x00, 0x7f, 0xff),
        ("Beige", 0xf5, 0xf5, 0xdc),
        ("Black", 0x00, 0x00, 0x00),
        ("Noir", 0x00, 0x00, 0x00),
        ("Blue", 0x00, 0x00, 0xff),
        ("Bleu", 0x00, 0x00, 0xff),
        ("Brown", 0x7B, 0x3F, 0x00),
        ("Burgundy", 0x80, 0x00, 0x20),
        ("Cerise", 0xde, 0x31, 0x63),
        ("Cerulean", 0x00, 0x7b, 0xa7),
        ("Charcoal", 0x36, 0x45, 0x4f),
        ("Chestnut", 0x95, 0x45, 0x35),
        ("Claret", 0x81, 0x13, 0x31),
        ("Coffee", 0x6f, 0x4e, 0x37),
        ("Copper", 0xb8, 0x73, 0x33),
        ("Coral", 0xf8, 0x83, 0x79),
        ("Cream", 0xff, 0xfd, 0xd0),
        ("Crimson", 0xDC, 0x14, 0x3C),
        ("Cyan", 0x00, 0xFF, 0xFF),
        ("Ebony", 0x55, 0x5d, 0x50),
        ("Eggshell", 0xf0, 0xea, 0xd6),
        ("Emerald", 0x50, 0xc8, 0x78),
        ("Fuchsia", 0xff, 0x00, 0xff),
        ("Gold", 0xff, 0xd7, 0x00),
        ("Gray", 0x80, 0x80, 0x80),
        ("Green", 0x00, 0xff, 0x00),
        ("Indigo", 0x4b, 0x00, 0x82),
        ("Ivory", 0xff, 0xff, 0xf0),
        ("Jasmine", 0xf8, 0xde, 0x7e),
        ("Khaki", 0xc3, 0xb0, 0x91),
        ("Lilac", 0xc8, 0xa2, 0xc8),
        ("Lime", 0xbf, 0xff, 0x00),
        ("Magenta", 0xff, 0x00, 0xff),
        ("Mango", 0xF4, 0xBB, 0x44),
        ("Maroon", 0x80, 0x00, 0x00),
        ("Marron", 0x80, 0x00, 0x00),
        ("Mauve", 0xe0, 0xb0, 0xff),
        ("Mustard", 0xff, 0xdb, 0x58),
        ("Ochre", 0xcc, 0x77, 0x22),
        ("Olive", 0x80, 0x80, 0x00),
        ("Orange", 0xff, 0xa5, 0x00),
        ("Periwinkle", 0xcc, 0xcc, 0xff),
        ("Pink", 0xff, 0x69, 0xb4),
        ("Plum", 0xdd, 0xa0, 0xdd),
        ("Puce", 0xcc, 0x88, 0x99),
        ("Purple", 0x66, 0x33, 0x99),
        ("Raspberry", 0xe3, 0x0b, 0x5d),
        ("Rose", 0xff, 0x00, 0x7f),
        ("Ruby", 0xe0, 0x11, 0x5f),
        ("Rouge", 0xff, 0x00, 0x00),
        ("Russet", 0x80, 0x46, 0x1b),
        ("Rust", 0xb7, 0x41, 0x0e),
        ("Saffron", 0xf4, 0xc4, 0x30),
        ("Sage", 0xB2, 0xAC, 0x88),
        ("Salmon", 0xfa, 0x80, 0x72),
        ("Scarlet", 0xff, 0x24, 0x00),
        ("Sepia", 0x70, 0x42, 0x14),
        ("Silver", 0xc0, 0xc0, 0xc0),
        ("Tangerine", 0xf2, 0x85, 0x00),
        ("Taupe", 0x48, 0x3c, 0x32),
        ("Teal", 0x00, 0x80, 0x80),
        ("Turquoise", 0x30, 0xd5, 0xc8),
        ("Umber", 0x63, 0x51, 0x47),
        ("Vermilion", 0xe3, 0x42, 0x34),
        ("Violet", 0xee, 0x82, 0xee),
        ("Wheat", 0xf5, 0xde, 0xb3),
        ("White", 0xff, 0xff, 0xff),
        ("Blanc", 0xff, 0xff, 0xff),
        ("Yellow", 0xff, 0xff, 0x00),
        ("Jaune", 0xff, 0xff, 0x00),
    ];

    let iter = pairs.iter().map(|(name, r, g, b)| {
        let name = normalize_characters_array::<16>(
            name,
            &crate::special_characters::SpecialCharacters::NONE,
        )
        .unwrap();
        let color: bevy_color::Srgba = bevy_color::Srgba::rgb_u8(*r, *g, *b);
        let slice: &[Character] = Box::leak(name.to_vec().into_boxed_slice());
        (slice, color)
    });

    BTreeMap::from_iter(iter)
});

impl<const GRID_SIZE: usize> DisplayWord<GRID_SIZE> {

    pub fn custom_color(&self) -> Option<bevy_color::Srgba> {
        COLORS_MAP.get(&self.characters.as_slice()).cloned()
    }

    pub fn label_letters(&self) -> usize {
        self.graphemes.len().max(self.hidden_text.len())
    }
}

impl<const GRID_SIZE: usize> PartialOrd for DisplayWord<GRID_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const GRID_SIZE: usize> Ord for DisplayWord<GRID_SIZE> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.characters
            .iter()
            .map(|x| x.as_char())
            .cmp(other.characters.iter().map(|x| x.as_char()))
    }
}

impl<const GRID_SIZE: usize> DisplayWord<GRID_SIZE> {
    pub fn from_string(
        mut s: &str,
        special_characters: &SpecialCharacters,
    ) -> Result<Self, &'static str> {
        let mut hidden_text: String = Default::default();
        let mut graphemes: Vec<CharGrapheme> = Default::default();
        let mut stack: usize = 0;
        let mut characters: ArrayVec<Character, GRID_SIZE> = Default::default();
        let show_initial: bool;

        if let Some(x) = s.strip_prefix('*') {
            s = x;
            show_initial = true;
        } else {
            show_initial = false;
        }

        let question_text: Option<Ustr> = {
            if s.ends_with(']') {
                if let Some(open_bracket_index) = s.find('[') {
                    let (start, end) = s.split_at(open_bracket_index);
                    s = start;
                    Some(end.trim_start_matches('[').trim_end_matches(']').into())
                } else {
                    return Err("Word contains ']' but not '['");
                }
            } else {
                None
            }
        };

        let iter = NormalizedCharacterIterator::new(s, special_characters);

        for r in iter {
            match r {
                NormalizedCharacterResult::Error { message, .. } => {
                    return Err(message);
                }
                NormalizedCharacterResult::RegularCharacter {
                    character,
                    grapheme,
                } => {
                    characters
                        .try_push(character)
                        .map_err(|_| "Word is too long")?;
                    stack += 1;

                    graphemes.push(CharGrapheme {
                        is_game_char: true,
                        grapheme: Ustr::from(grapheme),
                    });
                }
                NormalizedCharacterResult::SpecialCharacter {
                    character,
                    first_grapheme,
                    additional_graphemes,
                } => {
                    characters
                        .try_push(character)
                        .map_err(|_| "Word is too long")?;
                    stack += 1;

                    graphemes.push(CharGrapheme {
                        is_game_char: true,
                        grapheme: Ustr::from(
                            (std::iter::once(first_grapheme).chain(additional_graphemes))
                                .join("")
                                .as_str(),
                        ),
                    });
                }
                NormalizedCharacterResult::Blank { grapheme } => {
                    if let Some(char_to_push) = {
                        if ["-", "‐", "–", "—"].contains(&grapheme) {
                            Some('-')
                        } else if grapheme.chars().all(|x| x.is_ascii_whitespace()) {
                            Some(',') //use a comma instead of a space, like a crossword clue
                        } else {
                            None
                        }
                    } {
                        if stack > 0 {
                            hidden_text += stack.to_string().as_str();
                            stack = 0;
                        }
                        hidden_text.push(char_to_push);

                        graphemes.push(CharGrapheme {
                            is_game_char: false,
                            grapheme: Ustr::from(grapheme),
                        });
                    } else {
                        graphemes.push(CharGrapheme {
                            is_game_char: false,
                            grapheme: Ustr::from(grapheme),
                        });
                    }
                }
            }
        }

        if stack > 0 {
            hidden_text += stack.to_string().as_str();
        }

        Ok(Self {
            characters,
            text: Ustr::from(s),
            hidden_text: Ustr::from(&hidden_text),
            graphemes,
            show_initial,
            question_text,
        })
    }
}
