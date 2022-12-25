use utils::{
	files::read_file,
	parsing::parse_lines,
};

pub fn part_one(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_one_no(split)
}

pub fn part_one_no(split: Vec<String>) -> u64 {
	0
}

pub fn part_two(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_two_no(split)
}

pub fn part_two_no(split: Vec<String>) -> u64 {
	0
}
