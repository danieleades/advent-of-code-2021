use advent_of_code_2021::{Loader, Solution};
use clap::Parser;
use paste::paste;

fn default_input_root() -> String {
    std::env::current_dir()
        .unwrap()
        .join("inputs")
        .to_string_lossy()
        .to_string()
}

#[derive(Parser)]
pub struct App {
    /// The directory containing the puzzle inputs
    ///
    /// The puzzle inputs should be in plain-text files, with names day1, day2,
    /// etc.
    #[clap(short, long, default_value_t = default_input_root())]
    root: String,

    /// A comma-separated list of solutions to run
    #[clap(short, default_value = "1,2,3,6", value_delimiter = ',')]
    days: Vec<u16>,
}

impl App {
    pub fn from_cli() -> Self {
        Self::parse()
    }

    pub fn run(self) {
        let loader = Loader::new(self.root.into());

        macro_rules! run {
            ($n:literal) => {{
                let input = loader.load($n).unwrap();
                paste! {
                    run_and_display::<advent_of_code_2021::solutions::[<day $n>]::Puzzle>($n, &input);
                }
            }};
        }

        for day in self.days {
            match day {
                1 => run!(1),
                2 => run!(2),
                3 => run!(3),
                6 => run!(6),
                _ => unimplemented!(),
            }
        }
    }
}

fn run_and_display<T>(day: u16, input: &str)
where
    T: Solution,
{
    println!("day {}", day);
    println!("  part a: {}", T::part_a(input));
    println!("  part b: {}", T::part_b(input));
}
