// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   main.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/04 13:11:41 by Thibault Cheneviere                      \\
//   Updated: 2022/12/04 14:18:22 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::fs;

fn main() {
	// part_one();
	part_two();
}

fn part_one() {
	let file_path = "./input.txt";

	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	let split: Vec<&str> = contents.split('\n').collect();

	let mut result: u32 = 0;

	for line in split {
		if line == "" {
			break;
		}

		let temp: Vec<&str> = line.split(',').collect();
		
		if check(temp[0], temp[1]) {
			result += 1;
		}
	}

	println!("Result part one : {}", result);
}

fn part_two() {
	let file_path = "./input.txt";

	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	let split: Vec<&str> = contents.split('\n').collect();

	let mut result: u32 = 0;

	for line in split {
		if line == "" {
			break;
		}

		let temp: Vec<&str> = line.split(',').collect();
		
		if check_two(temp[0], temp[1]) {
			result += 1;
		}
	}

	println!("Result part two : {}", result);
}

fn check(f: &str, s: &str) -> bool {
	let split_f: Vec<&str> = f.split('-').collect();
	let split_s: Vec<&str> = s.split('-').collect();

	let beg_one = split_f[0].parse::<u32>().unwrap();
	let end_one = split_f[1].parse::<u32>().unwrap();
	let beg_two = split_s[0].parse::<u32>().unwrap();
	let end_two = split_s[1].parse::<u32>().unwrap();

	let size_one = end_one - beg_one + 1;
	let size_two = end_two - beg_two + 1;

	if size_one > size_two && (beg_one <= beg_two && end_one >= end_two) {
		return true;
	}

	if size_two >= size_one && (beg_two <= beg_one && end_two >= end_one) {
		return true;
	}

	return false;
}

fn check_two(f: &str, s: &str) -> bool {
	let split_f: Vec<&str> = f.split('-').collect();
	let split_s: Vec<&str> = s.split('-').collect();

	let beg_one = split_f[0].parse::<u32>().unwrap();
	let end_one = split_f[1].parse::<u32>().unwrap();
	let beg_two = split_s[0].parse::<u32>().unwrap();
	let end_two = split_s[1].parse::<u32>().unwrap();

	for i in beg_one..(end_one + 1) {
		if i >= beg_two && i <= end_two {
			return true;
		}
	}

	return false;
}
