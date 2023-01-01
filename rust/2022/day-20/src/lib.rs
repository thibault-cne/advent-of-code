// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/31 17:23:27 by Thibault Cheneviere                      \\
//   Updated: 2023/01/01 13:05:18 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::{
	files::read_file,
	parsing::parse_lines,
};

fn mix(mixed: &mut Vec<(usize, i64)>, nums: &Vec<(usize, i64)>) {
	for n in nums {
		let i = mixed.iter().position(|y| y == n).unwrap();
		let i_mixed = (i as i64 + n.1).rem_euclid((mixed.len() - 1) as i64) as usize;

		mixed.remove(i);
		mixed.insert(i_mixed, *n);
	}
}

pub fn part_one(file: &str) -> i64 {
	let split = parse_lines(read_file(file));

	part_one_no(split)
}

pub fn part_one_no(split: Vec<String>) -> i64 {
	let nums = split.iter().map(|line| line.parse::<i64>().unwrap()).enumerate().collect::<Vec<(usize, i64)>>();
	let mut mixed = nums.clone();

	mix(&mut mixed, &nums);

	let zero = mixed
		.iter()
		.position(|x| (*x).1 == 0)
		.expect("Expect at least one 0");

	[1_000, 2_000, 3_000]
		.iter()
		.fold(0, |acc, i| acc + mixed[(zero + i).rem_euclid(mixed.len())].1)
}

pub fn part_two(file: &str) -> i64 {
	let split = parse_lines(read_file(file));

	part_two_no(split)
}

pub fn part_two_no(split: Vec<String>) -> i64 {
	let key = 811_589_153;
	let nums = split.iter().map(|line| line.parse::<i64>().unwrap() * key).enumerate().collect::<Vec<(usize, i64)>>();
	let mut mixed = nums.clone();

	for _ in 0..10 {
		mix(&mut mixed, &nums);
	}

	let zero = mixed
		.iter()
		.position(|x| (*x).1 == 0)
		.expect("Expect at least one 0");

	[1_000, 2_000, 3_000]
		.iter()
		.fold(0, |acc, i| acc + mixed[(zero + i).rem_euclid(mixed.len())].1)
}
