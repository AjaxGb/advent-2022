#![feature(get_many_mut)]

use std::num::ParseIntError;
use std::str::FromStr;

use advent_2022::IteratorUtils;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WorryOp {
    Add,
    Multiply,
}

impl WorryOp {
    pub fn eval(&self, a: u64, b: u64) -> u64 {
        match self {
            WorryOp::Add => a + b,
            WorryOp::Multiply => a * b,
        }
    }
}

#[derive(Debug, Error)]
#[error("value was neither \"+\" nor \"*\"")]
struct ParseWorryOpError;

impl FromStr for WorryOp {
    type Err = ParseWorryOpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Multiply,
            _ => return Err(ParseWorryOpError),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WorryValue {
    Old,
    Const(u64),
}

impl WorryValue {
    pub fn eval(&self, old: u64) -> u64 {
        match self {
            WorryValue::Old => old,
            WorryValue::Const(val) => *val,
        }
    }
}

#[derive(Debug, Error)]
#[error("value was neither \"old\" nor a valid int ({0})")]
struct ParseWorryValueError(#[from] ParseIntError);

impl FromStr for WorryValue {
    type Err = ParseWorryValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "old" {
            Self::Old
        } else {
            Self::Const(s.parse()?)
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WorryExpr {
    pub a: WorryValue,
    pub op: WorryOp,
    pub b: WorryValue,
}

impl WorryExpr {
    pub fn eval(&self, old: u64) -> u64 {
        self.op.eval(self.a.eval(old), self.b.eval(old))
    }
}

#[derive(Debug, Error)]
enum ParseWorryExprError {
    #[error("expression did not have layout \"VAL OP VAL\"")]
    InvalidLayout,
    #[error("invalid value: {0}")]
    InvalidValue(#[from] ParseWorryValueError),
    #[error("invalid op: {0}")]
    InvalidOp(#[from] ParseWorryOpError),
}

impl FromStr for WorryExpr {
    type Err = ParseWorryExprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseWorryExprError::InvalidLayout;
        let mut s = s.split_whitespace();
        let a = s.next().ok_or(InvalidLayout)?.parse()?;
        let op = s.next().ok_or(InvalidLayout)?.parse()?;
        let b = s.next().ok_or(InvalidLayout)?.parse()?;
        if s.next().is_some() {
            return Err(InvalidLayout);
        }
        Ok(Self { a, op, b })
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: WorryExpr,
    test_divisor: u64,
    true_target: usize,
    false_target: usize,
    items_inspected: u64,
}

#[derive(Debug, Error)]
enum ParseMonkeyError {
    #[error("monkey definition did not have expected layout")]
    InvalidLayout,
    #[error("invalid monkey id: {0}")]
    InvalidId(#[source] ParseIntError),
    #[error("invalid item worry level: {0}")]
    InvalidItem(#[source] ParseIntError),
    #[error("invalid operation: {0}")]
    InvalidOperation(#[from] ParseWorryExprError),
    #[error("invalid test divisor: {0}")]
    InvalidTestDivisor(#[source] ParseIntError),
    #[error("invalid target: {0}")]
    InvalidTarget(#[source] ParseIntError),
}

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ParseMonkeyError::InvalidLayout;
        let mut s = s.lines();

        macro_rules! parse_line {
            ($prefix:literal $(, $suffix:literal)?) => {
                s.next()
                    .ok_or(InvalidLayout)?
                    .strip_prefix($prefix)
                    .ok_or(InvalidLayout)?
                    $(
                        .strip_suffix($suffix)
                        .ok_or(InvalidLayout)?
                    )?
            };
        }

        let id = parse_line!("Monkey ", ':')
            .parse()
            .map_err(Self::Err::InvalidId)?;
        let items = parse_line!("  Starting items: ")
            .split(", ")
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(Self::Err::InvalidItem)?;
        let operation = parse_line!("  Operation: new = ").parse()?;
        let test_divisor = parse_line!("  Test: divisible by ")
            .parse()
            .map_err(Self::Err::InvalidTestDivisor)?;
        let true_target = parse_line!("    If true: throw to monkey ")
            .parse()
            .map_err(Self::Err::InvalidTarget)?;
        let false_target = parse_line!("    If false: throw to monkey ")
            .parse()
            .map_err(Self::Err::InvalidTarget)?;

        Ok(Self {
            id,
            items,
            operation,
            test_divisor,
            true_target,
            false_target,
            items_inspected: 0,
        })
    }
}

fn run_monkeys(mut monkeys: Vec<Monkey>, wrap_at: u64, is_p1: bool) {
    let rounds = if is_p1 { 20 } else { 10_000 };

    for _ in 0..rounds {
        for monkey in 0..monkeys.len() {
            let (true_target, false_target) = {
                let monkey = &monkeys[monkey];
                (monkey.true_target, monkey.false_target)
            };
            let [monkey, true_target, false_target] = monkeys
                .get_many_mut([monkey, true_target, false_target])
                .unwrap();

            for item_worry in monkey.items.drain(..) {
                let mut item_worry = monkey.operation.eval(item_worry);
                monkey.items_inspected += 1;
                if is_p1 {
                    item_worry /= 3;
                }
                item_worry %= wrap_at;
                if item_worry % monkey.test_divisor == 0 {
                    true_target.items.push(item_worry);
                } else {
                    false_target.items.push(item_worry);
                }
            }
        }
    }

    for m in &monkeys {
        println!(
            "Monkey {} inspected items {} times.",
            m.id, m.items_inspected
        );
    }

    let best_monkeys = monkeys
        .into_iter()
        .max_n_by_key::<_, _, 2>(|m| m.items_inspected);
    print!("Most active monkeys:");
    for m in &best_monkeys {
        print!(" {}", m.id);
    }
    println!();

    let monkey_business: u64 = best_monkeys
        .into_iter()
        .map(|m| m.items_inspected)
        .product();
    println!("Monkey business: {monkey_business}");
}

fn main() {
    let monkeys: Vec<_> = include_str!("input.txt")
        .split("\n\n")
        .enumerate()
        .map(|(i, monkey_def)| {
            let monkey: Monkey = monkey_def.parse().unwrap();
            assert_eq!(monkey.id, i);
            monkey
        })
        .collect();

    let wrap_at = monkeys.iter().map(|m| m.test_divisor).product();

    println!("Part 1:");
    run_monkeys(monkeys.clone(), wrap_at, true);
    println!();
    println!("Part 2:");
    run_monkeys(monkeys, wrap_at, false);
}
