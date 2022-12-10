use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
enum Instr {
    NoOp,
    AddX(i32),
}

impl Instr {
    pub const fn cycles(&self) -> u32 {
        match self {
            Self::NoOp => 1,
            Self::AddX(_) => 2,
        }
    }
}

#[derive(Debug, Clone, Error)]
enum ParseInstrError {
    #[error("cannot parse instruction from empty string")]
    Empty,
    #[error("unknown instruction name {0:?}")]
    UnknownName(String),
    #[error("instruction had too many arguments")]
    TrailingArgs,
    #[error("instruction was missing expected arguments")]
    MissingArgs,
    #[error("invalid int argument: {0}")]
    IntArg(#[from] ParseIntError),
}

impl FromStr for Instr {
    type Err = ParseInstrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let name = s.next().ok_or(Self::Err::Empty)?;
        let instr = match name {
            "noop" => Self::NoOp,
            "addx" => {
                let v = s.next().ok_or(Self::Err::MissingArgs)?;
                Self::AddX(v.parse()?)
            }
            _ => return Err(Self::Err::UnknownName(name.to_owned())),
        };
        if s.next().is_some() {
            Err(Self::Err::TrailingArgs)
        } else {
            Ok(instr)
        }
    }
}

#[derive(Debug, Clone)]
struct DeviceSim<I: Iterator<Item = Instr>> {
    cycle: u32,
    register_x: i32,
    curr_instr: Option<Instr>,
    wait_cycles: u32,
    instrs: I,
}

impl<I: Iterator<Item = Instr>> DeviceSim<I> {
    pub fn new(mut instrs: I) -> Self {
        let curr_instr = instrs.next();
        let wait_cycles = curr_instr.map_or(0, |i| i.cycles());
        Self {
            cycle: 1,
            register_x: 1,
            curr_instr,
            wait_cycles,
            instrs,
        }
    }

    pub const fn is_done(&self) -> bool {
        self.curr_instr.is_none()
    }

    pub const fn curr_cycle(&self) -> u32 {
        self.cycle
    }

    pub const fn register_x(&self) -> i32 {
        self.register_x
    }

    pub fn exec_cycle(&mut self) {
        if let Some(curr_instr) = self.curr_instr {
            self.wait_cycles -= 1;
            if self.wait_cycles == 0 {
                match curr_instr {
                    Instr::NoOp => (),
                    Instr::AddX(v) => self.register_x += v,
                }
                self.curr_instr = self.instrs.next();
                if let Some(instr) = self.curr_instr {
                    self.wait_cycles = instr.cycles();
                }
            }
        }
        self.cycle += 1;
    }
}

fn main() {
    let instrs = include_str!("input.txt")
        .lines()
        .map(|s| s.parse().unwrap());
    let mut device = DeviceSim::new(instrs);
    let mut total_signals = 0;
    while !device.is_done() && device.curr_cycle() <= 220 {
        let cycle = device.curr_cycle();
        if cycle % 40 == 20 {
            let reg_x = device.register_x();
            let signal = cycle as i64 * reg_x as i64;
            println!("Cycle {cycle}: X = {reg_x}, signal = {signal}");
            total_signals += signal;
        }
        device.exec_cycle();
    }
    println!("Total signal strengths: {total_signals}");
}
