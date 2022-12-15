#![feature(iter_next_chunk)]

mod iter_utils;
mod simple_parse;
mod vec2;

pub use iter_utils::*;
pub type Vec2 = vec2::Vec2<i32>;
