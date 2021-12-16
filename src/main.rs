#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use cli::App;

mod cli;

fn main() {
    let app = App::from_cli();

    app.run();

    // let config = App::parse();
    // let loader = Loader::new(config.root);

    // let run_and_display<T> = |solution: &dyn Solution<Output = T>| {
    //     let input = loader.load(so).unwrap();

    //     println!("day {}", solution::DAY);
    //     println!("  part a: {}", solution::part_a(&input));
    //     println!("  part b: {}", solution::part_b(&input));
    // };
    // advent_of_code_2021::day1::Puzzle::run_and_display();
    // advent_of_code_2021::day2::Puzzle::run_and_display();
    // advent_of_code_2021::day3::Puzzle::run_and_display();
    // advent_of_code_2021::day4::Puzzle::run_and_display();
}

// trait SolutionExt {
//     fn run_and_display();
// }

// impl<T> SolutionExt for T
// where
//     T: Solution,
// {
//     fn run_and_display() {
//         println!("day {}", Self::DAY);
//         println!("  part a: {}", Self::part_a(Self::INPUT));
//         println!("  part b: {}", Self::part_b(Self::INPUT));
//     }
// }
