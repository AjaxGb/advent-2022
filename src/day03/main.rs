#![feature(exact_size_is_empty)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(iter_array_chunks)]

use std::fmt::{self, Debug};
use std::num::NonZeroU8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Item(NonZeroU8);

impl Item {
    pub const MIN_PRIORITY: u8 = 1;
    pub const MAX_PRIORITY: u8 = 52;

    pub const fn from_char(c: char) -> Option<Self> {
        Some(Self(unsafe {
            NonZeroU8::new_unchecked(match c {
                'a'..='z' => c as u8 - b'a' + 1,
                'A'..='Z' => c as u8 - b'A' + 27,
                _ => return None,
            })
        }))
    }

    pub const fn from_index(p: u8) -> Option<Self> {
        if p < Self::MAX_PRIORITY {
            Some(Self(unsafe { NonZeroU8::new_unchecked(p + 1) }))
        } else {
            None
        }
    }

    pub const fn to_char(self) -> char {
        (match self.0.get() {
            c @ 1..=26 => c - 1 + b'a',
            c @ 27..=52 => c - 27 + b'A',
            _ => unreachable!(),
        }) as char
    }

    pub const fn priority(self) -> u32 {
        self.0.get() as u32
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Item").field(&self.to_char()).finish()
    }
}

#[derive(PartialEq, Eq, Default)]
pub struct ItemSet(u64);

impl ItemSet {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    const fn item_to_flag(item: Item) -> u64 {
        1 << (item.0.get() as u64 - 1)
    }

    pub const fn contains(&self, item: Item) -> bool {
        self.0 & Self::item_to_flag(item) != 0
    }

    pub const fn add(&mut self, item: Item) {
        self.0 |= Self::item_to_flag(item);
    }

    pub const fn remove(&mut self, item: Item) {
        self.0 &= !Self::item_to_flag(item);
    }

    pub const fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    pub const fn first(&self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            let index = self.0.trailing_zeros();
            Item::from_index(index as u8)
        }
    }

    pub const fn pop_first(&mut self) -> Option<Item> {
        if self.is_empty() {
            None
        } else {
            let index = self.0.trailing_zeros();
            self.0 &= !(1 << index as u64);
            Item::from_index(index as u8)
        }
    }

    pub const fn iter(&self) -> ItemSetIter {
        ItemSetIter(self.clone())
    }
}

impl const Clone for ItemSet {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl FromIterator<Item> for ItemSet {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        let mut set = Self::new();
        for item in iter {
            set.add(item);
        }
        set
    }
}

impl Extend<Item> for ItemSet {
    fn extend<T: IntoIterator<Item = Item>>(&mut self, iter: T) {
        for item in iter {
            self.add(item);
        }
    }
}

impl IntoIterator for ItemSet {
    type Item = Item;

    type IntoIter = ItemSetIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for &ItemSet {
    type Item = Item;

    type IntoIter = ItemSetIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone)]
pub struct ItemSetIter(ItemSet);

impl ItemSetIter {
    pub const fn into_remaining(self) -> ItemSet {
        self.0
    }
}

impl ExactSizeIterator for ItemSetIter {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Iterator for ItemSetIter {
    type Item = Item;

    fn next(&mut self) -> Option<Item> {
        self.0.pop_first()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }
}

impl Debug for ItemSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self).finish()
    }
}

fn main() {
    let mut mispacked_sum = 0;
    let mut badges_sum = 0;

    for group in include_str!("input.txt").lines().array_chunks::<3>() {
        let group_items = group
            .map(|rucksack| {
                let (left, right) = rucksack.split_at(rucksack.len() / 2);
                let left: ItemSet = left.chars().map(|c| Item::from_char(c).unwrap()).collect();
                let right: ItemSet = right.chars().map(|c| Item::from_char(c).unwrap()).collect();
                for item in left.intersection(&right) {
                    mispacked_sum += item.priority();
                }
                left.union(&right)
            })
            .into_iter()
            .reduce(|a, b| a.intersection(&b))
            .unwrap();
        assert_eq!(group_items.len(), 1);
        let badge = group_items.first().unwrap();
        badges_sum += badge.priority();
    }

    println!("Total mispacked priority: {mispacked_sum}");
    println!("Total badge priority: {badges_sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_items() {
        for (i, c) in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
        {
            let item = Item::from_char(c).unwrap();
            assert_eq!(item.to_char(), c);
            assert_eq!(item.priority() as usize, i + 1);
            assert_eq!(Item::from_index(i as u8), Some(item));
            assert_eq!(format!("{item:?}"), format!("Item({c:?})"));
        }
    }

    #[test]
    fn invalid_items() {
        for c in "\0\x01?@[\\]^_`{|}\x7f\u{80}\u{fe}\u{ff}\u{100}".chars() {
            assert_eq!(Item::from_char(c), None);
        }
        for p in [52, 53, 54, 55, 127, 128, 254, 255] {
            assert_eq!(Item::from_index(p), None);
        }
    }
}
