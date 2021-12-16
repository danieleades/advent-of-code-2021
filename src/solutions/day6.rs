use std::str::FromStr;

use crate::Solution;

pub struct Puzzle;

impl Solution for Puzzle {
    type Output = u64;

    fn part_a(input: &str) -> Self::Output {
        total_population(input, 80)
    }

    fn part_b(input: &str) -> Self::Output {
        total_population(input, 256)
    }
}

fn total_population(input: &str, days: usize) -> u64 {
    let mut population = Population::new(input.split(',').map(|s| usize::from_str(s).unwrap()));
    for _ in 0..days {
        population.step();
    }
    population.total()
}

#[derive(Debug)]
struct Population {
    bins: [u64; 9],
}

impl Population {
    fn new(input: impl IntoIterator<Item = usize>) -> Self {
        let mut bins = [0; 9];
        for i in input {
            bins[i] += 1;
        }

        Self { bins }
    }

    fn step(&mut self) {
        let empty_nesters = self.bins[0];

        // every fish gets a day older
        self.bins.rotate_left(1);

        // new parents return to the population at 6
        self.bins[6] += empty_nesters;
    }

    fn total(&self) -> u64 {
        self.bins.iter().sum()
    }
}

#[cfg(test)]
mod tests {

    use test_case::test_case;

    use super::Population;

    #[test_case(18 => 26)]
    #[test_case(80 => 5934)]
    fn population(days: usize) -> u64 {
        let mut population = Population::new([3, 4, 3, 1, 2]);

        for _ in 0..days {
            population.step();
        }

        population.total()
    }
}
