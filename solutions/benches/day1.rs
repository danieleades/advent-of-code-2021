use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;
use solutions::Solution;

pub fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! bench_day {
        ($d:literal) => {{
            paste! {
                use solutions::[<day $d>]::Puzzle;
            }

            c.bench_function(concat!("day ", stringify!($d), ", part a"), |b| {
                b.iter(|| Puzzle::part_a(black_box(Puzzle::INPUT)))
            });

            c.bench_function(concat!("day ", stringify!($d), ", part b"), |b| {
                b.iter(|| Puzzle::part_b(black_box(Puzzle::INPUT)))
            });
        }};
    }

    bench_day!(1);
    bench_day!(2);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
