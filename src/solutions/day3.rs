use crate::Solution;
pub struct Puzzle;

impl Solution for Puzzle {
    type Output = u64;

    fn part_a(input: &str) -> Self::Output {
        let mut accumulator = Accumulator::default();
        let mut length = 0;

        for line in input.lines() {
            length += 1;
            for (i, c) in line.chars().enumerate() {
                accumulator.push(i, c == '1');
            }
        }

        let gamma_rate = gamma_rate(accumulator.most_common(length));
        let epsilon_rate = epsilon_rate(accumulator.most_common(length));

        gamma_rate * epsilon_rate
    }

    fn part_b(input: &str) -> Self::Output {
        let o2 = oxygen_rating(input);
        let co2 = co2_rating(input);

        o2 * co2
    }
}

#[derive(Debug, Default)]
struct Accumulator {
    length: usize,
    counts: Vec<usize>,
}

impl Accumulator {
    fn push(&mut self, index: usize, value: bool) {
        while index >= self.counts.len() {
            self.counts.push(0);
        }

        self.counts[index] += usize::from(value);
        self.length += 1;
    }

    fn most_common(&self, length: usize) -> impl Iterator<Item = bool> + '_ {
        self.counts.iter().map(move |&x| x > (length / 2))
    }
}

fn gamma_rate(bits: impl Iterator<Item = bool>) -> u64 {
    bits.fold(0, |result, bit| (result << 1) ^ u64::from(bit))
}

fn epsilon_rate(bits: impl Iterator<Item = bool>) -> u64 {
    let neg = bits.map(|b| !b);
    gamma_rate(neg)
}

fn partition<'a, I>(values: I, index: usize) -> (Vec<&'a str>, Vec<&'a str>)
where
    I: IntoIterator<Item = &'a str>,
{
    let (ones, zeros) = values
        .into_iter()
        .partition(|s| s.chars().nth(index) == Some('1'));

    (ones, zeros)
}

fn filter_oxygen<'a, I>(values: I, index: usize) -> Vec<&'a str>
where
    I: IntoIterator<Item = &'a str>,
{
    let (ones, zeros) = partition(values, index);

    if ones.len() >= zeros.len() {
        ones
    } else {
        zeros
    }
}

fn oxygen_rating(input: &str) -> u64 {
    let mut values = filter_oxygen(input.lines(), 0);
    let mut i = 1_usize;

    while values.len() > 1 {
        values = filter_oxygen(values, i);
        i += 1;
    }

    u64::from_str_radix(values[0], 2).unwrap()
}

fn filter_co2<'a, I>(values: I, index: usize) -> Vec<&'a str>
where
    I: IntoIterator<Item = &'a str>,
{
    let (ones, zeros) = partition(values, index);

    if ones.len() < zeros.len() {
        ones
    } else {
        zeros
    }
}

fn co2_rating(input: &str) -> u64 {
    let mut values = filter_co2(input.lines(), 0);
    let mut i = 1_usize;
    while values.len() > 1 {
        values = filter_co2(values, i);
        i += 1;
    }

    u64::from_str_radix(values[0], 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use crate::Solution;

    const INPUT: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    #[test]
    fn part_a() {
        let expected = 198;

        assert_eq!(Puzzle::part_a(INPUT), expected);
    }

    #[test]
    fn part_b() {
        let expected = 230;

        assert_eq!(Puzzle::part_b(INPUT), expected);
    }
}
