// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   main.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/01 13:09:24 by Thibault Cheneviere                      \\
//   Updated: 2022/12/02 00:28:17 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::fs;

fn main() {
	let file_path = "./input.txt";

	println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

	let split: Vec<&str> = contents.split('\n').collect();
	let mut max_cal: Vec<u32> = Vec::new();
	max_cal.push(0);
	max_cal.push(0);
	max_cal.push(0);
	let mut current_cal: u32 = 0;
	let mut is_insrt: bool = false;

	for x in split {
		match x {
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

    println!("With text:\n{}", max_cal[0] + max_cal[1] + max_cal[2]);
}
