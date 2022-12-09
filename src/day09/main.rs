use std::collections::HashSet;
use std::ops::{AddAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0, 0);

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

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone)]
struct Rope<const N: usize> {
    knots: [Vec2; N],
    tail_history: HashSet<Vec2>,
}

impl<const N: usize> Rope<N> {
    pub fn new() -> Self {
        if N == 0 {
            panic!("Rope must have at least one element");
        }
        let mut tail_history = HashSet::new();
        tail_history.insert(Vec2::ZERO);
        Self {
            knots: [Vec2::ZERO; N],
            tail_history,
        }
    }

    pub const fn head(&self) -> Vec2 {
        self.knots[0]
    }

    fn head_mut(&mut self) -> &mut Vec2 {
        &mut self.knots[0]
    }

    pub const fn tail(&self) -> Vec2 {
        self.knots[N - 1]
    }

    fn tail_mut(&mut self) -> &mut Vec2 {
        &mut self.knots[N - 1]
    }

    pub const fn tail_history(&self) -> &HashSet<Vec2> {
        &self.tail_history
    }

    fn simulate_knot(target: Vec2, knot: &mut Vec2) -> bool {
        let to_target = target - *knot;
        let dist = to_target.abs();
        let knot_moves = dist.y > 1 || dist.x > 1;
        if knot_moves {
            *knot += to_target.signum();
        }
        knot_moves
    }

    pub fn move_head(&mut self, offset: Vec2) {
        *self.head_mut() += offset;

        let mut target_pos = self.head();
        for curr_pos in &mut self.knots[1..N - 1] {
            Self::simulate_knot(target_pos, curr_pos);
            target_pos = *curr_pos;
        }
        if Self::simulate_knot(target_pos, self.tail_mut()) {
            self.tail_history.insert(self.tail());
        }
    }
}

fn main() {
    let mut rope_p1 = Rope::<2>::new();
    let mut rope_p2 = Rope::<10>::new();

    for line in include_str!("input.txt").lines() {
        let (dir, dist) = line.split_once(' ').unwrap();
        let offset = match dir {
            "U" => Vec2::new(0, -1),
            "D" => Vec2::new(0, 1),
            "L" => Vec2::new(-1, 0),
            "R" => Vec2::new(1, 0),
            _ => panic!("unknown direction {dir:?}"),
        };
        let dist: u32 = dist.parse().unwrap();

        for _ in 0..dist {
            rope_p1.move_head(offset);
            rope_p2.move_head(offset);
        }
    }

    println!("Num tail positions (P1): {}", rope_p1.tail_history().len());
    println!("Num tail positions (P2): {}", rope_p2.tail_history().len());
}
