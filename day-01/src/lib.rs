// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/05 15:14:01 by Thibault Cheneviere                      \\
//   Updated: 2022/12/05 15:24:27 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;

pub fn part_one(file: &str) -> u32 {
	let split: Vec<String> = parse_lines(read_file(file));

	let mut max_cal: u32 = 0;
	let mut current_cal: u32 = 0;

	for x in split {
		match x.as_str() {
			"" => {
				if current_cal > max_cal {
					max_cal = current_cal;
				}
				current_cal = 0;
			}
			_ => current_cal += x.parse::<u32>().unwrap(),
		}

	}

	return max_cal;
}

pub fn part_two(file: &str) -> u32 {
	let split: Vec<String> = parse_lines(read_file(file));

	let mut max_cal: Vec<u32> = Vec::from([0, 0, 0]);
	let mut current_cal: u32 = 0;
	let mut is_insrt: bool = false;

	for x in split {
		match x.as_str() {
			"" => {
				if current_cal > max_cal[0] {
					max_cal.insert(0, current_cal);
					is_insrt = true;
				}
				if !is_insrt && current_cal > max_cal[1] {
					max_cal.insert(1, current_cal);
				}
				if !is_insrt &&  current_cal > max_cal[2] {
					max_cal[2] = current_cal;
				}

				is_insrt = false;
				current_cal = 0;
			}
			_ => current_cal += x.parse::<u32>().unwrap(),
		}

	}

	return max_cal[0] + max_cal[1] + max_cal[2];
}
