// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   main.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/02 10:42:14 by Thibault Cheneviere                      \\
//   Updated: 2022/12/02 17:53:43 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::fs;

fn main() {
	let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

	let split: Vec<&str> = contents.split('\n').collect();

	let mut result: u32 = 0;

	for x in split {
		let decisions: Vec<&str> = x.split(' ').collect();

		if decisions.len() != 2 {
			break;
		}

		match decisions[1] {
			"X" => result += decider(decisions[0], decisions[1]) + 0,
			"Y" => result += decider(decisions[0], decisions[1]) + 3,
			"Z" => result += decider(decisions[0], decisions[1]) + 6,
			_ => (),
		}
	}

	println!("Result {}", result);
}

fn decider(elf: &str, result: &str) -> u32 {
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
