// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/15 11:53:42 by Thibault Cheneviere                      \\
//   Updated: 2022/12/15 13:16:10 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
		let mut result = Vec::new();

		// up
		if self.y > 0 {
			result.push(Self {
				x: self.x,
				y: self.y - 1,
			});
		}
		// down
		if self.y < rows - 1 {
			result.push(Self {
				x: self.x,
				y: self.y + 1,
			});
		}
		// left
		if self.x > 0 {
			result.push(Self {
				x: self.x - 1,
				y: self.y,
			});
		}
		// right
		if self.x < cols - 1 {
			result.push(Self {
				x: self.x + 1,
				y: self.y,
			});
		}

		result
	}
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
	cost: u32,
	coord: Coordinate,
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		other.cost.cmp(&self.cost)
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub fn part_one(file: &str) -> u64 {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_one_no(split);
}

fn parse(split: Vec<String>) -> (Coordinate, Coordinate, Vec<Vec<u8>>, usize, usize) {
	let rows = split.len();
	let cols = split[0].len();
	let mut grid = vec![vec![0; cols]; rows];
	let mut start = Coordinate{x: 0, y: 0};
	let mut end = Coordinate{x: 0, y: 0};

	for (row, line) in split.iter().enumerate() {
		for (col, c) in line.chars().enumerate() {
			let letter = match c {
				'S' => {
					start.x = col;
					start.y = row;
					'a'
				}
				'E' => {
					end.x = col;
					end.y = row;
					'z'
				}
				'a'..='z' => c,
				_ => panic!("Invalid input"),
			};

			let val = letter as u8 - b'a';
			grid[row][col] = val;
		}
	}

	(start, end, grid, rows, cols)
}

pub fn part_one_no(split: Vec<String>) -> u64 {
	let (start, end, grid, rows, cols) = parse(split);
	let mut pq = BinaryHeap::new();
	let mut visited = HashSet::new();

	pq.push(Node {
		cost: 0,
		coord: start,
	});
	visited.insert(start);

	while let Some(Node {cost, coord}) = pq.pop() {
		if coord == end {
			return cost as u64;
		}

		let curr_height = grid[coord.y][coord.x];
		let neighbours = coord.neighbours(rows, cols);
		let candidates: Vec<_> = neighbours
			.iter()
			.filter(|coord| {
				let height = grid[coord.y][coord.x];
				height <= (curr_height + 1)
			})
			.collect();

		for candidate in candidates {
			if visited.insert(*candidate) {
				pq.push(Node {
					cost: cost + 1,
					coord: *candidate,
				})
			}
		}
	}

	u64::MAX
}

pub fn part_two(file: &str) -> u64 {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_two_no(split);
}

pub fn part_two_no(split: Vec<String>) -> u64 {
	let (_, end, grid, rows, cols) = parse(split);
	let mut pq = BinaryHeap::new();
	let mut visited = HashSet::new();

	pq.push(Node {
		cost: 0,
		coord: end,
	});

	while let Some(Node {cost, coord}) = pq.pop() {
		let curr_height = grid[coord.y][coord.x];

		if curr_height == 0 {
			return cost as u64;
		}

		let neighbours = coord.neighbours(rows, cols);
		let candidates: Vec<_> = neighbours
			.iter()
			.filter(|coord| {
				let height = grid[coord.y][coord.x];
				height >= (curr_height - 1)
			})
			.collect();

		for candidate in candidates {
			if visited.insert(*candidate) {
				pq.push(Node {
					cost: cost + 1,
					coord: *candidate,
				})
			}
		}
	}

	u64::MAX
}
