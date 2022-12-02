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
    pub const fn wins_against(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub const fn loses_against(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub const fn opponent_for(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => self.wins_against(),
            Outcome::Draw => self,
            Outcome::Lose => self.loses_against(),
        }
    }

    pub const fn against(self, other: Hand) -> Outcome {
        if self.wins_against() == other {
            Outcome::Win
        } else if self == other {
            Outcome::Draw
        } else {
            Outcome::Lose
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
    pub const fn invert(self) -> Self {
        match self {
            Self::Win => Self::Lose,
            Self::Draw => Self::Draw,
            Self::Lose => Self::Win,
        }
    }

    pub const fn score(self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    pub const fn from_char(c: char, base: char) -> Option<Self> {
        Some(match (c as u32).checked_sub(base as u32) {
            Some(0) => Self::Lose,
            Some(1) => Self::Draw,
            Some(2) => Self::Win,
            _ => return None,
        })
    }
}

fn main() {
    let mut total_score_p1 = 0;
    let mut total_score_p2 = 0;

    for strat in include_str!("input.txt").lines() {
        let mut strat = strat.chars();
        let strat_a = strat.next().unwrap();
        assert_eq!(strat.next(), Some(' '));
        let strat_b = strat.next().unwrap();

        let theirs = Hand::from_char(strat_a, 'A').unwrap();
        // Part 1
        {
            let mine = Hand::from_char(strat_b, 'X').unwrap();
            let outcome = mine.against(theirs);
            total_score_p1 += mine.score() + outcome.score();
        }
        // Part 2
        {
            let outcome = Outcome::from_char(strat_b, 'X').unwrap();
            let mine = theirs.opponent_for(outcome.invert());
            total_score_p2 += mine.score() + outcome.score();
        }
    }

    println!("Total score, part 1: {total_score_p1}");
    println!("Total score, part 2: {total_score_p2}");
}
