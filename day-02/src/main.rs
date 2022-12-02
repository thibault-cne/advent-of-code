// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   main.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/02 10:42:14 by Thibault Cheneviere                      \\
//   Updated: 2022/12/02 10:52:19 by Thibault Cheneviere                      \\
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

		match decisions[0] {
			"A" => {
				match decisions[1] {
					"X" => result += 1 + 3,
					"Y" => result += 2 + 6,
					"Z" => result += 3,
					_ => (),
				}
			}
			"B" => {
				match decisions[1] {
					"X" => result += 1 + 0,
					"Y" => result += 2 + 3,
					"Z" => result += 3 + 6,
					_ => (),
				}
			}
			"C" => {
				match decisions[1] {
					"X" => result += 1 + 6,
					"Y" => result += 2 + 0,
					"Z" => result += 3 + 3,
					_ => (),
				}
			}
			_ => (),
		}
	}

	println!("Result {}", result);
}
