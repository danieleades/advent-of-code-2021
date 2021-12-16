use itertools::Itertools;

use crate::Solution;

pub struct Puzzle;

impl Solution for Puzzle {
    type Output = i32;

    fn part_a(input: &str) -> Self::Output {
        let positions = input.split(',').map(|s| s.parse().unwrap());
        let target = median(positions.clone()).expect("input is non-empty");

        positions.map(|p| (target - p).abs()).sum()
    }

    fn part_b(input: &str) -> Self::Output {
        todo!()
    }
}

fn median(input: impl IntoIterator<Item = i32>) -> Option<i32> {
    let values: Vec<_> = input.into_iter().sorted().collect();
    match values.len() {
        0 => None,
        // even case
        x if x % 2 == 0 => Some((values[x / 2] + values[x / 2 - 1]) / 2),

        // odd case
        x => Some(values[(x - 1) / 2]),
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use crate::Solution;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_a() {
        let expected = 37;

        assert_eq!(Puzzle::part_a(INPUT), expected);
    }

    #[test]
    fn part_b() {
        let expected = 230;

        assert_eq!(Puzzle::part_b(INPUT), expected);
    }
}
