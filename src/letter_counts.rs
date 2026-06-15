use crate::prelude::*;
use bit_bag::prelude::{BitBag, BitBag64, BitBagArray};
use std::fmt::{Display, Write};
pub type LetterBag = BitBagArray<BitBag64<5>, 4>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LetterCounts(pub LetterBag);

impl Display for LetterCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in self.clone().into_iter().map(|x| x.as_char()) {
            f.write_char(char)?;
        }

        Ok(())
    }
}

impl LetterCounts {
    pub fn try_from_iter(iter: impl IntoIterator<Item = Character>) -> Option<Self> {
        let mut bag = BitBagArray::EMPTY;

        for char in iter {
            bag = bag.with_checked_add(char as u8 as u32, 1)?;
        }

        Some(Self(bag))
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn count_instances(&self, character: Character) -> usize {
        self.0.element_count(character as u8 as u32) as usize
    }

    pub fn into_iter(self) -> impl Iterator<Item = Character> + Clone {
        self.0
            .iter_elements()
            .map(|x| Character::from_repr(x as u8).unwrap())
    }

    pub fn try_remove(&self, character: Character) -> Option<Self> {
        let bag = self.0.with_checked_sub(character as u8 as u32, 1)?;
        Some(Self(bag))
    }

    pub fn with_inserted(&self, character: Character) -> Self {
        let mut clone = self.clone();
        clone.0.saturating_add(character.as_u32(), 1);
        clone
    }
}
