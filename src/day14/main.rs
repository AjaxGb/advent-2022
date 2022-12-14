use std::{
    io::{self, stdout},
    num::ParseIntError,
    str::FromStr,
    time::Duration,
};

use advent_2022::Vec2;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tile {
    #[default]
    Air,
    Stone,
    Sand,
}

impl Tile {
    pub fn is_air(self) -> bool {
        self == Self::Air
    }
}

pub struct SandSim {
    width: usize,
    height: usize,
    top_left: Vec2,
    grid: Vec<Tile>,
    falling_sand: Option<Vec2>,
    resting_sand: u32,
}

impl SandSim {
    pub const SPAWN_POS: Vec2 = Vec2::new(500, 0);

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn resting_sand(&self) -> u32 {
        self.resting_sand
    }

    pub fn draw(&self, out: &mut impl io::Write) -> io::Result<()> {
        let mut buffer = vec![b'\0'; self.width + 1];
        buffer[self.width] = b'\n';
        for row in self.grid.chunks_exact(self.width) {
            for (i, tile) in row.iter().enumerate() {
                buffer[i] = match *tile {
                    Tile::Air => b'.',
                    Tile::Stone => b'#',
                    Tile::Sand => b'o',
                }
            }
            out.write(&buffer)?;
        }
        Ok(())
    }

    fn pos_to_index(&self, pos: Vec2) -> Option<usize> {
        let loc = pos - self.top_left;
        if loc.x < 0 || loc.x as usize > self.width {
            None
        } else if loc.y < 0 || loc.y as usize > self.height {
            None
        } else {
            Some(loc.x as usize + loc.y as usize * self.width)
        }
    }

    pub fn get_mut(&mut self, pos: Vec2) -> Option<&mut Tile> {
        self.pos_to_index(pos).map(|i| &mut self.grid[i])
    }

    pub fn get(&self, pos: Vec2) -> Option<&Tile> {
        self.pos_to_index(pos).map(|i| &self.grid[i])
    }

    pub fn update(&mut self) -> bool {
        let Some(old_pos) = self.falling_sand else {
            self.falling_sand = Some(Self::SPAWN_POS);
            let tile = self.get_mut(Self::SPAWN_POS).unwrap();
            *tile = Tile::Sand;
            return false;
        };

        for dir in [Vec2::DOWN, Vec2::DOWN_LEFT, Vec2::DOWN_RIGHT] {
            let new_pos = old_pos + dir;
            if let Some(new_tile) = self.get_mut(new_pos) {
                if new_tile.is_air() {
                    *new_tile = Tile::Sand;
                    *self.get_mut(old_pos).unwrap() = Tile::Air;
                    self.falling_sand = Some(new_pos);
                    return false;
                }
            } else {
                // It fell out of the sim space
                *self.get_mut(old_pos).unwrap() = Tile::Air;
                self.falling_sand = None;
                return true;
            };
        }

        self.falling_sand = None;
        self.resting_sand += 1;
        false
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
        let mut line_defs = vec![];
        let mut min_x = Self::SPAWN_POS.x;
        let mut max_x = Self::SPAWN_POS.x;
        let mut min_y = Self::SPAWN_POS.y;
        let mut max_y = Self::SPAWN_POS.y;
        for line in s.lines() {
            let mut points = line.split(" -> ").map(|s| {
                let Some((x, y)) = s.split_once(',') else {
                    return Err(InvalidPoint);
                };
                let x = x.parse()?;
                let y = y.parse()?;
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
                Ok(Vec2::new(x, y))
            });
            let Some(start) = points.next() else {
                continue;
            };
            let mut prev = start?;
            for point in points {
                let point = point?;
                line_defs.push(if prev.x == point.x {
                    if prev.y <= point.y {
                        (prev, point)
                    } else {
                        (point, prev)
                    }
                } else if prev.y == point.y {
                    if prev.x <= point.x {
                        (prev, point)
                    } else {
                        (point, prev)
                    }
                } else {
                    return Err(DiagonalLine(prev, point));
                });
                prev = point;
            }
        }
        let width = (max_x - min_x) as usize + 1;
        let height = (max_y - min_y) as usize + 1;
        let mut grid = vec![Tile::Air; width * height];
        let top_left = Vec2::new(min_x, min_y);
        for (a, b) in line_defs {
            for x in a.x..=b.x {
                for y in a.y..=b.y {
                    let point = Vec2::new(x, y) - top_left;
                    let index = point.x as usize + point.y as usize * width;
                    grid[index] = Tile::Stone;
                }
            }
        }
        Ok(Self {
            width,
            height,
            top_left,
            grid,
            falling_sand: None,
            resting_sand: 0,
        })
    }
}

fn main() {
    let mut out = stdout().lock();
    let mut sim: SandSim = include_str!("input.txt").parse().unwrap();
    sim.draw(&mut out).unwrap();
    loop {
        if sim.update() {
            break;
        }
    }
    println!("Resting sand: {}", sim.resting_sand());
}
