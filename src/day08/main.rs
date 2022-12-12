#![feature(let_chains)]

use std::collections::HashSet;

use advent_2022::Vec2;

struct Grid {
    width: usize,
    height: usize,
    values: Vec<u8>,
}

impl Grid {
    pub fn parse(text: &str) -> Self {
        let mut rows = text.lines();
        if let Some(first_row) = rows.next() {
            let width = first_row.len();
            let mut height = 1;
            let mut values: Vec<u8> = first_row.chars().map(Self::parse_digit).collect();
            for row in rows {
                assert_eq!(row.len(), width);
                values.extend(row.chars().map(Self::parse_digit));
                height += 1;
            }
            debug_assert_eq!(width * height, values.len());
            Self {
                width,
                height,
                values,
            }
        } else {
            Self {
                width: 0,
                height: 0,
                values: vec![],
            }
        }
    }

    fn parse_digit(c: char) -> u8 {
        c.to_digit(10).unwrap() as u8
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn max_x(&self) -> i32 {
        self.width as i32 - 1
    }

    pub const fn max_y(&self) -> i32 {
        self.height as i32 - 1
    }

    pub fn get(&self, pos: Vec2) -> Option<u8> {
        let x: usize = pos.x.try_into().ok()?;
        let y: usize = pos.y.try_into().ok()?;
        if x < self.width && y < self.height {
            Some(self.values[y * self.width + x])
        } else {
            None
        }
    }

    pub fn raycast_all_visible(&self, start: Vec2, offset: Vec2, visible: &mut HashSet<Vec2>) {
        let mut hit_height = self.get(start).unwrap();
        visible.insert(start);
        let mut pos = start + offset;
        while let Some(h) = self.get(pos) {
            if h > hit_height {
                hit_height = h;
                visible.insert(pos);
            }
            pos += offset;
        }
    }

    pub fn raycast_scenic_dist(&self, start: Vec2, offset: Vec2) -> u32 {
        let stop_height = self.get(start).unwrap();
        let mut dist = 0;
        let mut pos = start + offset;
        while let Some(h) = self.get(pos) {
            dist += 1;
            if h >= stop_height {
                break;
            }
            pos += offset;
        }
        dist
    }

    pub fn scenic_score(&self, pos: Vec2) -> u32 {
        let mut score = 1;
        for dir in Vec2::CARDINAL_DIRS {
            score *= self.raycast_scenic_dist(pos, dir);
            if score == 0 {
                break;
            }
        }
        score
    }
}

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));
    let mx = grid.max_x();
    let my = grid.max_y();

    let mut visible = HashSet::new();
    for y in 0..grid.height() as i32 {
        grid.raycast_all_visible(Vec2::new(0, y), Vec2::RIGHT, &mut visible);
        grid.raycast_all_visible(Vec2::new(mx, y), Vec2::LEFT, &mut visible);
    }
    for x in 0..grid.width() as i32 {
        grid.raycast_all_visible(Vec2::new(x, 0), Vec2::DOWN, &mut visible);
        grid.raycast_all_visible(Vec2::new(x, my), Vec2::UP, &mut visible);
    }

    println!("Visible trees: {}", visible.len());

    let mut max_scenic_score = 0;
    for y in 0..grid.height() as i32 {
        for x in 0..grid.width() as i32 {
            let scenic_score = grid.scenic_score(Vec2::new(x, y));
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("Max scenic score: {}", max_scenic_score);
}
