// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   benchmark.rs                                                             \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/05 15:25:53 by Thibault Cheneviere                      \\
//   Updated: 2022/12/05 15:29:01 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_01::{part_one, part_two};

criterion_group!(benches, benchmark_part1, benchmark_part2);
criterion_main!(benches);

fn benchmark_part1(c: &mut Criterion) {
    c.bench_function("part one", |b| {
        b.iter(|| black_box(part_one(black_box("./input.txt"))));
    });
}

fn benchmark_part2(c: &mut Criterion) {
    c.bench_function("part two", |b| {
        b.iter(|| black_box(part_two(black_box("./input.txt"))));
    });
}
