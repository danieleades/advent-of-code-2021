#![deny(clippy::all)]
#![warn(clippy::pedantic)]

pub mod day1;
pub mod day2;

pub trait Solution {
    const DAY: u16;
    const INPUT: &'static str;

    type Output: std::fmt::Display;

    fn part_a(input: &str) -> Self::Output;
    fn part_b(input: &str) -> Self::Output;
}
