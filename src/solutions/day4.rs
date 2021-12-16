use std::str::FromStr;

use ndarray::{Array2, Axis};

use crate::Solution;

pub struct Puzzle;

impl Solution for Puzzle {
    type Output = u32;

    fn part_a(input: &str) -> Self::Output {
        let bingo = Bingo::from_str(input).unwrap();
        let Output { last_called, score } = bingo.win();

        last_called * score
    }

    fn part_b(input: &str) -> Self::Output {
        let bingo = Bingo::from_str(input).unwrap();
        let Output { last_called, score } = bingo.lose();

        last_called * score
    }
}

struct Bingo {
    input: Vec<u32>,
    boards: Vec<Board>,
}

impl Bingo {
    fn has_won(&self) -> Option<u32> {
        self.boards
            .iter()
            .find(|board| board.has_won())
            .map(Board::score)
    }

    fn win(mut self) -> Output {
        for n in &self.input {
            for b in &mut self.boards {
                b.call_number(*n);
            }
            if let Some(score) = self.has_won() {
                return Output {
                    last_called: *n,
                    score,
                };
            }
        }

        unreachable!()
    }

    fn lose(mut self) -> Output {
        for n in &self.input {
            let remaining_boards: Vec<_> = self
                .boards
                .iter_mut()
                .filter(|board| !board.has_won())
                .map(|board| {
                    board.call_number(*n);
                    board
                })
                .filter(|board| !board.has_won())
                .collect();

            if remaining_boards.len() == 1 {
                let score = remaining_boards[0].score();
                return Output {
                    last_called: *n,
                    score,
                };
            }
        }

        unreachable!()
    }
}

struct Output {
    pub last_called: u32,
    pub score: u32,
}

impl FromStr for Bingo {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|c| u32::from_str(c).unwrap())
            .collect();

        let boards = s
            .split("\n\n")
            .skip(1)
            .map(|s| Board::from_str(s).unwrap())
            .collect();

        Ok(Self { input, boards })
    }
}

#[derive(Debug)]
struct Board {
    values: Array2<u32>,
    called: Array2<bool>,
}

impl Board {
    fn new(values: Array2<u32>) -> Self {
        let shape = values.raw_dim();
        let called = Array2::default(shape);

        Self { values, called }
    }

    fn call_number(&mut self, n: u32) {
        let index = self
            .values
            .iter()
            .enumerate()
            .find_map(|(i, element)| if *element == n { Some(i) } else { None });

        if let Some(i) = index {
            self.called.as_slice_mut().unwrap()[i] = true;
        }
    }

    fn has_won(&self) -> bool {
        let (n_rows, n_cols) = self.called.dim();
        self.called
            .axis_iter(Axis(0))
            .any(|array| usize::from(array.map(|&b| u8::from(b)).sum()) == n_rows)
            || self
                .called
                .axis_iter(Axis(1))
                .any(|array| usize::from(array.map(|&b| u8::from(b)).sum()) == n_cols)
    }

    fn score(&self) -> u32 {
        self.values
            .iter()
            .zip(&self.called)
            .filter_map(|(value, called)| if *called { None } else { Some(value) })
            .sum()
    }
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<&str> = s.lines().collect();

        let n_rows = rows.len();
        let n_columns = rows[0].split_ascii_whitespace().count();

        let elements: Vec<u32> = rows
            .into_iter()
            .flat_map(str::split_ascii_whitespace)
            .map(|c| u32::from_str(c).unwrap())
            .collect();

        let values = Array2::<u32>::from_shape_vec((n_rows, n_columns), elements).unwrap();

        Ok(Self::new(values))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Bingo, Board, Puzzle};
    use crate::Solution;

    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn board_from_str() {
        let input = r#"22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
"#;

        let _board = Board::from_str(input).unwrap();
    }

    #[test]
    fn bingo_from_str() {
        let _bingo = Bingo::from_str(INPUT).unwrap();
    }

    #[test]
    fn part_a() {
        let expected = 4512;

        assert_eq!(Puzzle::part_a(INPUT), expected);
    }

    #[test]
    fn part_b() {
        let expected = 1924;

        assert_eq!(Puzzle::part_b(INPUT), expected);
    }
}
