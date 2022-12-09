use std::collections::HashSet;
use std::ops::{AddAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RopeVec {
    x: i32,
    y: i32,
}

impl RopeVec {
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

impl AddAssign for RopeVec {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

#[derive(Debug, Clone)]
struct Rope<const N: usize> {
    knots: [RopeVec; N],
    tail_history: HashSet<RopeVec>,
}

impl<const N: usize> Rope<N> {
    pub fn new() -> Self {
        if N == 0 {
            panic!("Rope must have at least one element");
        }
        let mut tail_history = HashSet::new();
        tail_history.insert(RopeVec::ZERO);
        Self {
            knots: [RopeVec::ZERO; N],
            tail_history,
        }
    }

    pub const fn head(&self) -> RopeVec {
        self.knots[0]
    }

    pub const fn tail(&self) -> RopeVec {
        self.knots[N - 1]
    }

    pub const fn tail_history(&self) -> &HashSet<RopeVec> {
        &self.tail_history
    }

    fn simulate_knot(target: RopeVec, knot: &mut RopeVec) -> bool {
        let to_target = target - *knot;
        let dist = to_target.abs();
        let knot_moves = dist.y > 1 || dist.x > 1;
        if knot_moves {
            *knot += to_target.signum();
        }
        knot_moves
    }

    pub fn move_head(&mut self, offset: RopeVec) {
        self.knots[0] += offset;

        let mut target_pos = self.head();
        for curr_pos in &mut self.knots[1..N - 1] {
            Self::simulate_knot(target_pos, curr_pos);
            target_pos = *curr_pos;
        }
        if Self::simulate_knot(target_pos, &mut self.knots[N - 1]) {
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
            "U" => RopeVec::new(0, -1),
            "D" => RopeVec::new(0, 1),
            "L" => RopeVec::new(-1, 0),
            "R" => RopeVec::new(1, 0),
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
