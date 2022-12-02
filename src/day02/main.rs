#![feature(const_trait_impl)]

#[derive(Debug, Clone, Copy, Eq)]
pub enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl const PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }
}

impl Hand {
    pub const fn beats(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub const fn against(self, other: Hand) -> Outcome {
        if self.beats() == other {
            Outcome::Win
        } else if other.beats() == self {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }

    pub const fn score(self) -> u32 {
        self as u32
    }

    pub const fn from_value(value: u32) -> Option<Self> {
        Some(match value {
            1 => Self::Rock,
            2 => Self::Paper,
            3 => Self::Scissors,
            _ => return None,
        })
    }

    pub const fn from_char(c: char, base: char) -> Option<Self> {
        Some(match (c as u32).checked_sub(base as u32) {
            Some(0) => Self::Rock,
            Some(1) => Self::Paper,
            Some(2) => Self::Scissors,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Outcome {
    Win = 1,
    Draw = 0,
    Lose = -1,
}

impl Outcome {
    pub const fn score(self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

fn main() {
    let mut total_score = 0;

    for strat in include_str!("input.txt").lines() {
        let mut strat = strat.chars();
        let theirs = Hand::from_char(strat.next().unwrap(), 'A').unwrap();
        assert_eq!(strat.next(), Some(' '));
        let mine = Hand::from_char(strat.next().unwrap(), 'X').unwrap();
        assert_eq!(strat.next(), None);

        let outcome = mine.against(theirs);
        let score = mine.score() + outcome.score();
        total_score += score;
    }

    println!("Total score: {total_score}");
}
