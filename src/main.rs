#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use solutions::{day1, day2, Solution};

fn main() {
    day1::Puzzle::run_and_display();
    day2::Puzzle::run_and_display();
}

trait SolutionExt {
    fn run_and_display();
}

impl<T> SolutionExt for T
where
    T: Solution,
{
    fn run_and_display() {
        println!("day {}", Self::DAY);
        println!("  part a: {}", Self::part_a(Self::INPUT));
        println!("  part b: {}", Self::part_b(Self::INPUT));
    }
}
