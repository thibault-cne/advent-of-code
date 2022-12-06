// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   benchmark.rs                                                             \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/05 15:43:46 by Thibault Cheneviere                      \\
//   Updated: 2022/12/06 16:43:09 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_02::{part_two, part_two_no};
use utils::files::read_file;
use utils::parsing::parse_lines;

criterion_group!(benches, benchmark_part_two, benchmark_part_two_no);
criterion_main!(benches);

/*

fn benchmark_part1(c: &mut Criterion) {
    c.bench_function("part one", |b| {
        b.iter(|| black_box(part_one(black_box("./input.txt"))));
    });
}

*/

fn benchmark_part_two(c: &mut Criterion) {
    c.bench_function("part two", |b| {
        b.iter(|| black_box(part_two(black_box("./input.txt"))));
    });
}

fn benchmark_part_two_no(c: &mut Criterion) {
	let split: Vec<String> = parse_lines(read_file("./input.txt"));

    c.bench_function("part two no", |b| {
        b.iter(|| black_box(part_two_no(black_box(split.clone()))));
    });
}
