// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/16 20:28:42 by Thibault Cheneviere                      \\
//   Updated: 2022/12/17 00:10:56 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Eq, PartialEq)]
pub enum Item {
	Num(u8),
	List(Vec<Item>),
}

impl Ord for Item {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Self::List(a), Self::List(b)) => a.cmp(b),
			(Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
			(Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(&b),
			(Self::Num(a), Self::Num(b)) => a.cmp(b),
		}
	}
}

impl PartialOrd for Item {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub fn part_one(file: &str) -> u64 {
	let items: Vec<[Item; 2]> = parse(file);

	return part_one_no(items);
}

pub fn part_one_no(items: Vec<[Item; 2]>) -> u64 {
	items
		.iter()
		.positions(|[left, right]| left < right)
		.map(|idx| idx + 1)
		.sum::<usize>() as u64
}

pub fn part_two(file: &str) -> u64 {
	let items: Vec<[Item; 2]> = parse(file);

	return part_two_no(items);
}

pub fn part_two_no(items: Vec<[Item; 2]>) -> u64 {
	let mut items: Vec<_> = items.iter().flatten().collect();

	let div_one = parse_item("[[2]]");
	let div_two = parse_item("[[6]]");

	items.push(&div_one);
	items.push(&div_two);

	items.sort_unstable();

	items
		.into_iter()
		.positions(|item| item == &div_one || item == &div_two)
		.map(|idx| idx + 1)
		.product::<usize>() as u64
}

fn parse(file: &str) -> Vec<[Item; 2]> {
	let input = std::fs::read_to_string(file).unwrap();

	input
		.split("\n\n")
		.map(|pair| {
			let mut lines = pair.lines();
			let left = lines.next().unwrap();
			let right = lines.next().unwrap();

			[parse_item(left), parse_item(right)]
		})
		.collect()
}

fn parse_item(line: &str) -> Item {
	let chars: Vec<char> = line.chars().collect();

	let (item, _) = parse_list(&chars);
	item
}

fn parse_list(chars: &[char]) -> (Item, &[char]) {
	// We remove the first [
	let mut chars = &chars[1..];
	let mut items: Vec<Item> = Vec::new();

	loop {
		match chars[0] {
			// List ended
			']' => break,
			// We skip ,
			',' => chars = &chars[1..],
			// We call recursion if we reach a new list
			'[' => {
				let (item, rest) = parse_list(chars);
				items.push(item);
				chars = rest;
			}
			// Number
			_ => {
				let (n, rest) = parse_num(chars);
				items.push(n);
				chars = rest;
			}
		}
	}

	(Item::List(items), &chars[1..])
}

fn parse_num(chars: &[char]) -> (Item, &[char]) {
	// Find the index where the number end
	let mut i = 0;
	while i < chars.len() && chars[i].is_ascii_digit() {
		i += 1;
	}

	// Create the number with math
	let mut number = 0;
	let mut offset = 1;
	for c in chars[..i].iter().rev() {
		number += (*c as u8 - b'0') * offset;
		offset *= 10;
	}

	(Item::Num(number), &chars[i..])
}
