// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/05 15:37:27 by Thibault Cheneviere                      \\
//   Updated: 2022/12/06 16:29:52 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;

pub fn part_two(file: &str) -> u32 {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_two_no(split);
}

pub fn part_two_no(split: Vec<String>) -> u32 {
	let mut result: u32 = 0;

	for x in split {
		let decisions: Vec<&str> = x.as_str().split(' ').collect();

		if decisions.len() != 2 {
			break;
		}

		match decisions[1] {
			"X" => result += decider_two(decisions[0], decisions[1]) + 0,
			"Y" => result += decider_two(decisions[0], decisions[1]) + 3,
			"Z" => result += decider_two(decisions[0], decisions[1]) + 6,
			_ => (),
		}
	}

	return result;
}

fn decider_two(elf: &str, result: &str) -> u32 {
	match result {
		"X" => {
			match elf {
				"A" => 3,
				"B" => 1,
				"C" => 2,
				_ => 0,
			}
		}
		"Y" => {
			match elf {
				"A" => 1,
				"B" => 2,
				"C" => 3,
				_ => 0,
			}
		}
		"Z" => {
			match elf {
				"A" => 2,
				"B" => 3,
				"C" => 1,
				_ => 0,
			}
		}
		_ => 0,
	}
}
