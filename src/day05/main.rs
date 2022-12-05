#![feature(let_chains)]
#![feature(iter_array_chunks)]

use advent_2022::IteratorUtils;

#[derive(Debug, Clone, Default)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    pub fn top(&self) -> Option<char> {
        self.crates.last().copied()
    }
}

#[derive(Debug, Clone, Default)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    pub fn parse<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Self {
        let mut stacks = vec![];
        while let line = lines.next().unwrap() && !line.is_empty() {
            for (col, [l, value, r]) in line.chars().array_chunks_sep::<3, 1>().enumerate() {
                if stacks.len() <= col {
                    stacks.push(Stack::default());
                }
                if (l, r) == ('[', ']') {
                    stacks[col].crates.push(value);
                }
            }
        }
        for stack in &mut stacks {
            stack.crates.reverse();
        }
        Self { stacks }
    }

    pub fn move_from_to(&mut self, from: usize, to: usize) {
        if from != to {
            let value = self.stacks[from - 1].crates.pop().unwrap();
            self.stacks[to - 1].crates.push(value);
        }
    }

    pub fn stacks(&self) -> &[Stack] {
        self.stacks.as_ref()
    }
}

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut stacks = Stacks::parse(&mut lines);

    for line in lines {
        let mut line = line.split(' ');
        assert_eq!(line.next(), Some("move"));
        let num_crates: u32 = line.next().unwrap().parse().unwrap();
        assert_eq!(line.next(), Some("from"));
        let from = line.next().unwrap().parse().unwrap();
        assert_eq!(line.next(), Some("to"));
        let to = line.next().unwrap().parse().unwrap();
        for _ in 0..num_crates {
            stacks.move_from_to(from, to);
        }
    }

    print!("Top of stacks: [");
    for stack in stacks.stacks() {
        print!("{}", stack.top().unwrap());
    }
    println!("]")
}
