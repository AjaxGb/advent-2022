use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use advent_2022::Vec2;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct SandSim {
    grid: HashSet<Vec2>,
    falling_sand: Option<Vec2>,
    resting_sand: u32,
    floor_y: i32,
    has_floor: bool,
}

impl SandSim {
    pub const SPAWN_POS: Vec2 = Vec2::new(500, 0);

    pub const fn resting_sand(&self) -> u32 {
        self.resting_sand
    }

    pub fn update(&mut self) -> bool {
        if let Some(old_pos) = self.falling_sand {
            if old_pos.y + 1 >= self.floor_y {
                self.falling_sand = None;
                if self.has_floor {
                    self.resting_sand += 1;
                    false
                } else {
                    self.grid.remove(&old_pos);
                    true
                }
            } else {
                for dir in [Vec2::DOWN, Vec2::DOWN_LEFT, Vec2::DOWN_RIGHT] {
                    let new_pos = old_pos + dir;
                    if self.grid.insert(new_pos) {
                        self.grid.remove(&old_pos);
                        self.falling_sand = Some(new_pos);
                        return false;
                    }
                }
                self.falling_sand = None;
                self.resting_sand += 1;
                false
            }
        } else {
            if self.grid.insert(Self::SPAWN_POS) {
                self.falling_sand = Some(Self::SPAWN_POS);
                false
            } else {
                true
            }
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum ParseSandSimError {
    #[error("points must be formatted as X,Y")]
    InvalidPoint,
    #[error("coordinate was not a valid int: {0}")]
    InvalidInt(#[from] ParseIntError),
    #[error("diagonal lines ({0} to {1}) are not supported")]
    DiagonalLine(Vec2, Vec2),
}

impl FromStr for SandSim {
    type Err = ParseSandSimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseSandSimError::*;
        let mut grid = HashSet::new();
        let mut max_y = Self::SPAWN_POS.y;
        for line in s.lines() {
            let mut points = line.split(" -> ").map(|s| {
                let Some((x, y)) = s.split_once(',') else {
                    return Err(InvalidPoint);
                };
                let x = x.parse()?;
                let y = y.parse()?;
                max_y = max_y.max(y);
                Ok(Vec2::new(x, y))
            });
            let Some(start) = points.next() else {
                continue;
            };
            let mut prev = start?;
            for point in points {
                let point = point?;
                if prev.x != point.x && prev.y != point.y {
                    return Err(DiagonalLine(prev, point));
                }
                for x in prev.x.min(point.x)..=prev.x.max(point.x) {
                    for y in prev.y.min(point.y)..=prev.y.max(point.y) {
                        grid.insert(Vec2 { x, y });
                    }
                }
                prev = point;
            }
        }
        Ok(Self {
            grid,
            falling_sand: None,
            resting_sand: 0,
            floor_y: max_y + 2,
            has_floor: false,
        })
    }
}

fn main() {
    let mut sim1: SandSim = include_str!("input.txt").parse().unwrap();
    let mut sim2 = sim1.clone();
    sim2.has_floor = true;
    
    while !sim1.update() {}
    println!("Resting sand, Part 1: {}", sim1.resting_sand());
    
    while !sim2.update() {}
    println!("Resting sand, Part 2: {}", sim2.resting_sand());
}
