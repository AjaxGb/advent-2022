use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    str::FromStr,
};

use advent_2022::Vec2;
use thiserror::Error;

#[derive(PartialEq, Eq)]
struct Queued {
    dist: u32,
    pos: Vec2,
    height: u8,
}

impl Ord for Queued {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

impl PartialOrd for Queued {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    values: Vec<u8>,
    start: Vec2,
    end: Vec2,
}

impl Grid {
    pub fn get(&self, pos: Vec2) -> Option<u8> {
        let x: usize = pos.x.try_into().ok()?;
        let y: usize = pos.y.try_into().ok()?;
        if x < self.width && y < self.height {
            Some(self.values[y * self.width + x])
        } else {
            None
        }
    }

    pub fn pathfind(&self) -> (Option<u32>, Option<(Vec2, u32)>) {
        let mut to_visit = BinaryHeap::new();
        let mut best_path_to = HashMap::new();

        // Go from end to start, so we can find all starts
        {
            let end = self.end;
            let height = self.get(end).unwrap();
            let dist = 0;
            to_visit.push(Queued {
                dist,
                pos: end,
                height,
            });
            best_path_to.insert(end, dist);
        }

        let mut to_orig_start = None;
        let mut to_best_start = None;

        while let Some(Queued { pos, height, .. }) = to_visit.pop() {
            let dist = *best_path_to.get(&pos).unwrap();
            if pos == self.start && to_orig_start.is_none() {
                to_orig_start = Some(dist);
                if to_best_start.is_some() {
                    break;
                }
            }
            if height == 0 && to_best_start.is_none() {
                to_best_start = Some((pos, dist));
                if to_orig_start.is_some() {
                    break;
                }
            }
            for dir in Vec2::CARDINAL_DIRS {
                let next_pos = pos + dir;
                let Some(next_height) = self.get(next_pos) else {
                    continue;
                };
                if next_height >= height || next_height == height - 1 {
                    let dist_to_next = dist + 1;
                    let is_better = match best_path_to.entry(next_pos) {
                        Entry::Occupied(mut e) => {
                            let is_better = dist_to_next < *e.get();
                            if is_better {
                                e.insert(dist_to_next);
                            }
                            is_better
                        }
                        Entry::Vacant(e) => {
                            e.insert(dist_to_next);
                            true
                        }
                    };
                    if is_better {
                        to_visit.push(Queued {
                            dist: dist_to_next,
                            pos: next_pos,
                            height: next_height,
                        });
                    }
                }
            }
        }

        (to_orig_start, to_best_start)
    }
}

#[derive(Debug, Clone, Error)]
pub enum ParseGridError {
    #[error("cannot parse grid from empty string")]
    Empty,
    #[error("row {row} had width {row_width} (expected {expected_width})")]
    InconsistentWidth {
        row: usize,
        row_width: usize,
        expected_width: usize,
    },
    #[error("found more than one start position (at {0:?}, {1:?})")]
    MultipleStarts(Vec2, Vec2),
    #[error("found more than one end position (at {0:?}, {1:?})")]
    MultipleEnds(Vec2, Vec2),
    #[error("grid contained unexpected char: {0:?}")]
    InvalidChar(char),
    #[error("grid did not have a start")]
    NoStart,
    #[error("grid did not have an end")]
    NoEnd,
}

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseGridError::*;
        let mut rows = s.lines().peekable();
        let width = rows.peek().ok_or(Empty)?.len();
        let mut height = 0;
        let mut values = vec![];
        let mut start = None;
        let mut end = None;
        for (y, row) in rows.enumerate() {
            if row.len() != width {
                return Err(InconsistentWidth {
                    row: y + 1,
                    row_width: row.len(),
                    expected_width: width,
                });
            }
            values.reserve(width);
            for (x, c) in row.chars().enumerate() {
                let height = match c {
                    'S' => {
                        let pos = Vec2::new(x as i32, y as i32);
                        if let Some(old) = start.replace(pos) {
                            return Err(MultipleStarts(old, pos));
                        }
                        0
                    }
                    'E' => {
                        let pos = Vec2::new(x as i32, y as i32);
                        if let Some(old) = end.replace(pos) {
                            return Err(MultipleEnds(old, pos));
                        }
                        b'z' - b'a'
                    }
                    c => {
                        if !c.is_ascii_lowercase() {
                            return Err(InvalidChar(c));
                        }
                        c as u8 - b'a'
                    }
                };
                values.push(height);
            }
            height += 1;
        }
        debug_assert_eq!(width * height, values.len());
        let start = start.ok_or(NoStart)?;
        let end = end.ok_or(NoEnd)?;
        Ok(Self {
            width,
            height,
            values,
            start,
            end,
        })
    }
}

fn main() {
    let grid: Grid = include_str!("input.txt").parse().unwrap();
    let (orig_start, best_start) = grid.pathfind();
    println!("Distance to original start: {}", orig_start.unwrap());
    let (best_start, dist_to_best_start) = best_start.unwrap();
    println!(
        "Distance to best start ({}): {}",
        best_start, dist_to_best_start
    );
}
