use crate::prelude::*;
use bit_bag::prelude::{BitBag, BitBag64, BitBagArray, ElementGroup};
use std::{
    fmt::{Display, Write},
    num::NonZeroU32,
};
pub type LetterBag = BitBagArray<BitBag64<5>, 4>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LetterCounts(LetterBag);

impl Display for LetterCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in self.clone().iter_characters().map(|x| x.as_char()) {
            f.write_char(char)?;
        }

        Ok(())
    }
}

impl LetterCounts {
    pub const EMPTY: Self = Self::new(LetterBag::EMPTY);

    pub const fn new(bit_bag_array: LetterBag) -> Self {
        Self(bit_bag_array)
    }

    pub const fn inner(&self) -> LetterBag {
        self.0
    }

    pub fn set_element_count(&mut self, character: Character, count: u32) {
        self.0.set_element_count(character.as_u32(), count);
    }

    pub fn count_total(&self) -> u32 {
        self.0.count_total()
    }

    pub fn saturating_add(&mut self, character: Character, amount: u32) {
        self.0.saturating_add(character.as_u32(), amount);
    }

    pub fn saturating_sub(&mut self, character: Character, amount: u32) {
        self.0.saturating_sub(character.as_u32(), amount);
    }

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

    pub fn is_superset(&self, other: &Self) -> bool {
        self.0.is_superset(&other.0)
    }

    pub fn count_instances(&self, character: Character) -> usize {
        self.0.element_count(character as u8 as u32) as usize
    }

    pub fn iter_characters(self) -> impl Iterator<Item = Character> + Clone {
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

    pub fn with_union(&self, other: &Self) -> Self {
        Self(self.0.with_union(&other.0))
    }

    pub fn union_with(&mut self, other: &Self) {
        self.0.union_with(&other.0);
    }

    pub fn with_intersect(&self, other: &Self) -> Self {
        Self(self.0.with_intersect(&other.0))
    }

    pub fn intersect_with(&mut self, other: &Self) {
        self.0.intersect_with(&other.0);
    }

    pub fn saturating_sum_with(&mut self, other: &Self) {
        self.0.saturating_sum_with(&other.0);
    }

    pub fn saturating_difference_with(&mut self, other: &Self) {
        self.0.saturating_difference_with(&other.0);
    }

    pub fn with_checked_difference(&self, other: &Self) -> Option<Self> {
        Some(Self(self.0.with_checked_difference(&other.0)?))
    }

    pub fn with_saturating_difference(&self, other: &Self) -> Self {
        Self(self.0.with_saturating_difference(&other.0))
    }

    pub fn iter_character_groups(&self) -> impl Iterator<Item = (Character, NonZeroU32)> {
        self.0
            .iter_element_groups()
            .map(|ElementGroup { element, count }| {
                (Character::from_repr(element as u8).unwrap(), count)
            })
    }
}
