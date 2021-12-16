#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod loader;
pub mod solutions;
pub use loader::Loader;

pub trait Solution {
    type Output: std::fmt::Display;

    fn part_a(input: &str) -> Self::Output;
    fn part_b(input: &str) -> Self::Output;
}
