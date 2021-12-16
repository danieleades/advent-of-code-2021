use advent_of_code_2021::{Loader, Solution};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;

pub fn criterion_benchmark(c: &mut Criterion) {
    let loader = Loader::default();
    macro_rules! bench_day {
        ($d:literal) => {{
            paste! {
                use advent_of_code_2021::solutions::[<day $d>]::Puzzle;
            }

            let input = loader.load($d).expect("unable to load puzzle input");

            c.bench_function(concat!("day ", stringify!($d), ", part a"), |b| {
                b.iter(|| Puzzle::part_a(black_box(&input)))
            });

            c.bench_function(concat!("day ", stringify!($d), ", part b"), |b| {
                b.iter(|| Puzzle::part_b(black_box(&input)))
            });
        }};
    }

    bench_day!(1);
    bench_day!(2);
    bench_day!(3);
    bench_day!(6);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
