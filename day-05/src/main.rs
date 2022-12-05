// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   main.rs                                                                  \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/05 08:20:12 by Thibault Cheneviere                      \\
//   Updated: 2022/12/05 11:07:44 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::fs;
use std::collections::LinkedList;

fn main() {
	part_one("./input.txt");
	part_two("./input.txt");
}

fn part_one(file_path: &str) {
	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	let split: Vec<&str> = contents.split('\n').collect();
	let stack_number: usize = (split[0].len() + 1) / 4;

	let mut lists: Vec<LinkedList<char>> = Vec::new();

	for _i in 0..stack_number {
		lists.push(LinkedList::new() as LinkedList<char>);
	}

	for i in 0..8 {
		let line: &str = split[i];
		if line == "" {
			break;
		}

		fill_stacks(line, lists.as_mut_slice());
	}

	for i in 10..split.len() {
		if split[i] != "" {
			r_move_one(split[i], lists.as_mut_slice());
		}
	}

	display_stacks(lists.as_slice());
}

fn part_two(file_path: &str) {
	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	let split: Vec<&str> = contents.split('\n').collect();
	let stack_number: usize = (split[0].len() + 1) / 4;

	let mut lists: Vec<LinkedList<char>> = Vec::new();

	for _i in 0..stack_number {
		lists.push(LinkedList::new() as LinkedList<char>);
	}

	for i in 0..8 {
		let line: &str = split[i];
		if line == "" {
			break;
		}

		fill_stacks(line, lists.as_mut_slice());
	}

	for i in 10..split.len() {
		if split[i] != "" {
			r_move_two(split[i], lists.as_mut_slice());
		}
	}

	display_stacks(lists.as_slice());
}

fn display_stacks(lists:  &[LinkedList<char>]) {
	for i in lists {
		print!("{}", i.front().unwrap());
	}

	println!("");
}

fn r_move_one(line: &str, lists: &mut [LinkedList<char>]) {
	let temp: Vec<&str> = line.split(' ').collect();

	let mov: u32 = temp[1].parse::<u32>().unwrap();
	let from: usize = temp[3].parse::<usize>().unwrap();
	let to: usize = temp[5].parse::<usize>().unwrap();

	for _i in 0..mov {
		let temp_c: Option<char> = lists[from - 1].pop_front();

		if temp_c.is_some() {
			let chr: char = temp_c.unwrap();
			lists[to -1].push_front(chr);
		} else {
			break;
		} 
	}
}

fn r_move_two(line: &str, lists: &mut [LinkedList<char>]) {
	let temp: Vec<&str> = line.split(' ').collect();

	let mov: u32 = temp[1].parse::<u32>().unwrap();
	let from: usize = temp[3].parse::<usize>().unwrap();
	let to: usize = temp[5].parse::<usize>().unwrap();
	let mut crates: Vec<char> = Vec::new();

	for _i in 0..mov {
		let temp_c: Option<char> = lists[from - 1].pop_front();

		if temp_c.is_some() {
			let chr: char = temp_c.unwrap();
			crates.insert(0, chr);
		} else {
			break;
		} 
	}

	for i in crates {
		lists[to - 1].push_front(i);
	}
}

fn fill_stacks(line: &str, lists: &mut [LinkedList<char>]) {
	let mut i: usize = 1;
	let mut stack_ptr: usize = 0;

	while i < line.len() {
		let chr: char = line.as_bytes()[i] as char; 

		if chr != ' ' {
			lists[stack_ptr].push_back(chr);
		}

		i += 4;
		stack_ptr += 1;
	}
}
