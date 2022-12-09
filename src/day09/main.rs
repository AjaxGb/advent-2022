use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RopeVec {
    x: i32,
    y: i32,
}

impl RopeVec {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub const fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl Add for RopeVec {
    type Output = RopeVec;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for RopeVec {
    type Output = RopeVec;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for RopeVec {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn main() {
    let mut head_pos = RopeVec::new(0, 0);
    let mut tail_pos = RopeVec::new(0, 0);

    let mut all_tail_pos = HashSet::new();
    all_tail_pos.insert(tail_pos);

    for line in include_str!("input.txt").lines() {
        let (dir, dist) = line.split_once(' ').unwrap();
        let offset = match dir {
            "U" => RopeVec::new(0, -1),
            "D" => RopeVec::new(0, 1),
            "L" => RopeVec::new(-1, 0),
            "R" => RopeVec::new(1, 0),
            _ => panic!("unknown direction {dir:?}"),
        };
        let dist: u32 = dist.parse().unwrap();

        for _ in 0..dist {
            head_pos += offset;

            let to_head = head_pos - tail_pos;
            let dist = to_head.abs();
            if dist.y > 1 || dist.x > 1 {
                tail_pos += to_head.signum();
                all_tail_pos.insert(tail_pos);
            }
        }
    }

    println!("Num tail positions: {}", all_tail_pos.len());
}
