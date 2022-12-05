#![feature(let_chains)]
#![feature(iter_array_chunks)]
#![feature(get_many_mut)]

use std::fmt::{self, Write};

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

    pub fn move_group_from_to(&mut self, size: usize, from: usize, to: usize) {
        if from != to {
            let [from, to] = self.stacks.get_many_mut([from - 1, to - 1]).unwrap();
            let keep_from = from.crates.len() - size;
            let values = &from.crates[keep_from..];
            to.crates.extend_from_slice(values);
            from.crates.truncate(keep_from);
        }
    }

    pub fn stacks(&self) -> &[Stack] {
        self.stacks.as_ref()
    }

    pub fn tops<'a>(&'a self) -> StackTops<'a> {
        StackTops(self)
    }
}

#[derive(Debug, Clone, Copy)]
struct StackTops<'a>(&'a Stacks);

impl fmt::Display for StackTops<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stack in self.0.stacks() {
            f.write_char(stack.top().unwrap_or(' '))?;
        }
        Ok(())
    }
}

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut stacks_p1 = Stacks::parse(&mut lines);
    let mut stacks_p2 = stacks_p1.clone();

    for line in lines {
        let mut line = line.split(' ');
        assert_eq!(line.next(), Some("move"));
        let num_crates = line.next().unwrap().parse().unwrap();
        assert_eq!(line.next(), Some("from"));
        let from = line.next().unwrap().parse().unwrap();
        assert_eq!(line.next(), Some("to"));
        let to = line.next().unwrap().parse().unwrap();

        for _ in 0..num_crates {
            stacks_p1.move_from_to(from, to);
        }
        stacks_p2.move_group_from_to(num_crates, from, to);
    }

    println!("Top of stacks P1: [{}]", stacks_p1.tops());
    println!("Top of stacks P2: [{}]", stacks_p2.tops());
}
