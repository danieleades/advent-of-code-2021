use itertools::Itertools;

use crate::Solution;

pub struct Puzzle;

impl Solution for Puzzle {
    type Output = usize;

    const DAY: u16 = 1;
    const INPUT: &'static str = include_str!("inputs/day1");

    fn part_a(input: &str) -> Self::Output {
        let elements = parse(input);

        count_increases(elements)
    }

    fn part_b(input: &str) -> Self::Output {
        let elements = parse(input)
            .tuple_windows::<(_, _, _)>()
            .map(|(a, b, c)| a + b + c);

        count_increases(elements)
    }
}

fn parse(raw: &str) -> impl Iterator<Item = i32> + '_ {
    raw.lines().map(str::parse).map(Result::unwrap)
}

fn count_increases(input: impl Iterator<Item = i32>) -> usize {
    input
        .tuple_windows::<(_, _)>()
        .map(|(prev, x)| x - prev)
        .filter(|difference| difference.is_positive())
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{day1::Puzzle, Solution};

    const INPUT: &str = r#"199
200
208
210
200
207
240
269
260
263
"#;

    #[test]
    fn part_a() {
        let expected = 7;

        assert_eq!(Puzzle::part_a(INPUT), expected);
    }

    #[test]
    fn part_b() {
        let expected = 5;

        assert_eq!(Puzzle::part_b(INPUT), expected);
    }
}
