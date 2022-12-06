// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/06 17:07:07 by Thibault Cheneviere                      \\
//   Updated: 2022/12/06 18:03:05 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;

pub fn part_one(file: &str) -> u32 {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_one_no(split);
}

pub fn part_two(file: &str) -> u32 {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_two_no(split);
}

pub fn part_one_no(split: Vec<String>) -> u32 {
	let mut res: u32 = 4;

	let win = split[0].as_bytes().windows(4);
	let _res = win.map(|val| has_unique(val)); 
	for val in _res {
		if !val {
			res+= 1;
		} else {
			break;
		}
	}

	return res;
}

pub fn part_two_no(split: Vec<String>) -> u32 {
	let mut res: u32 = 14;

	let win = split[0].as_bytes().windows(14);
	let _res = win.map(|val| has_unique(val)); 
	for i in _res {
		if !i {
			res += 1
		} else {
			break;
		}
	}

	return res;
}

fn has_unique(slice: &[u8]) -> bool {
	for i in slice.iter() {
		if slice.iter().filter(|&x| *x == *i).count() > 1 {
			return false;
		}
	}

	return true;
}
