// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   main.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/03 08:37:00 by Thibault Cheneviere                      \\
//   Updated: 2022/12/03 09:37:21 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::fs;

fn main() {
	let file_path = "./input.txt";

	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	let split: Vec<&str> = contents.split('\n').collect();

	let mut result: u32 = 0;
	let mut done: u8 = 0;
	let mut containers: Vec<&str> = Vec::new();

	for line in split {
		containers.insert(0, line);

		done += 1;

		if done == 3 {
			done = 0;

			result += get_priority_2(containers.clone());
			containers.remove(2);
			containers.remove(1);
			containers.remove(0);
		}
	}

	println!("Result {}", result);
}

fn get_priority(first: &str, second: &str) -> u32 {
	for i in first.chars() {
		if second.contains(i) {
			if (i as u32) < 91 {
				return (i as u32) - 65 + 27;
			}
			return (i as u32) - 96;
		}
	}

	0
}

fn get_priority_2(con: Vec<&str>) -> u32 {
	for i in con[0].chars() {
		if con[1].contains(i) && con[2].contains(i) {
			if (i as u32) < 91 {
				return (i as u32) - 65 + 27;
			}
			return (i as u32) - 96;
		}
	}

	0
}
