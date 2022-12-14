// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/14 11:31:11 by Thibault Cheneviere                      \\
//   Updated: 2022/12/14 12:22:14 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;

pub fn part_one(file: &str) -> isize {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_one_no(split);
}

pub fn part_one_no(split: Vec<String>) -> isize {
	let mut cycle: usize = 1;
	let mut x: isize = 1;
	let mut res: isize = 0;

	for line in split {
		let temp: Vec<&str> = line.split(' ').collect::<Vec<&str>>();

		match temp[0] {
			"noop" => cycle += 1,
			"addx" => {
				cycle += 1;
				res += check_cycle(x, cycle);
				cycle += 1;
				x += temp[1].parse::<isize>().unwrap();
			}
			_ => (),
		}
		res += check_cycle(x, cycle);
	}

	return res;
}

fn check_cycle(x: isize, cycle: usize) -> isize {
	return match cycle {
		20 => 20 * x,
		60 => 60 * x,
		100 => 100 * x,
		140 => 140 * x,
		180 => 180 * x,
		220 => 220 * x,
		_ => 0,
	}
}

pub fn part_two(file: &str) -> usize {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_two_no(split);
}

pub fn part_two_no(split: Vec<String>) -> usize {
	let mut cycle: usize = 0;
	let mut x: isize = 1;

	for line in split {
		let temp: Vec<&str> = line.split(' ').collect::<Vec<&str>>();

		match temp[0] {
			"noop" => {
				cycle += 1;
				draw(x, cycle);
			}
			"addx" => {
				cycle += 1;
				draw(x, cycle);
				cycle += 1;
				draw(x, cycle);
				x += temp[1].parse::<isize>().unwrap();
			}
			_ => (),
		}
	}

	return 0;
}

fn draw(x: isize, cycle: usize) {
	let j: isize = ((cycle - 1) % 40).try_into().unwrap();

	if j == (x - 1) || j == x || j == (x + 1) {
		print!("#");
	} else {
		print!(".");
	}

	if cycle == 40 || cycle == 80 || cycle == 120 || cycle == 160 || cycle == 200 || cycle == 240 {
		println!("");
	}
}
