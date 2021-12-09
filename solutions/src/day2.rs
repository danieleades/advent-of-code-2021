use crate::Solution;

pub struct Puzzle;

impl Solution for Puzzle {
    type Output = i32;

    const DAY: u16 = 2;
    const INPUT: &'static str = include_str!("inputs/day2");

    fn part_a(input: &str) -> Self::Output {
        let position = parse(input).fold((0, 0), |mut position, (command, x)| {
            match command {
                "up" => position.1 -= x,
                "down" => position.1 += x,
                "forward" => position.0 += x,
                _ => unreachable!(),
            }

            position
        });

        position.0 * position.1
    }

    fn part_b(input: &str) -> Self::Output {
        #[derive(Default)]
        struct State {
            x: i32,
            y: i32,
            aim: i32,
        }

        impl State {
            fn down(&mut self, i: i32) {
                self.aim += i;
            }

            fn forward(&mut self, i: i32) {
                self.x += i;
                self.y += self.aim * i;
            }
        }

        let position = parse(input).fold(State::default(), |mut position, (command, x)| {
            match command {
                "up" => position.down(-x),
                "down" => position.down(x),
                "forward" => position.forward(x),
                _ => unreachable!(),
            }

            position
        });

        position.x * position.y
    }
}

fn parse(input: &str) -> impl Iterator<Item = (&str, i32)> + '_ {
    input
        .lines()
        .filter_map(|l| l.split_once(' '))
        .map(|(command, x)| (command, x.parse::<i32>().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use crate::Solution;

    const INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

    #[test]
    fn part_a() {
        let expected = 150;

        assert_eq!(Puzzle::part_a(INPUT), expected);
    }

    #[test]
    fn part_b() {
        let expected = 900;

        assert_eq!(Puzzle::part_b(INPUT), expected);
    }
}
