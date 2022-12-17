// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/17 00:13:52 by Thibault Cheneviere                      \\
//   Updated: 2022/12/17 01:41:53 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::collections::HashSet;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
	fn neighbours(&self) -> [Coord; 3] {
		let down = Coord {
			x: self.x,
			y: self.y + 1,
		};
		let left = Coord {
			x: self.x - 1,
			y: self.y + 1,
		};
		let right = Coord {
			x: self.x + 1,
			y: self.y + 1,
		};

		[down, left, right]
	}

	fn next(&self, cave: &[Vec<Tile>]) -> Option<Coord> {
		self
			.neighbours()
			.into_iter()
			.find(|coord| cave[coord.y as usize][coord.x as usize] == Tile::Air)

	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Rock,
    Sand,
    Air,
}

pub fn part_one(file: &str) -> u64 {
	let input: Vec<Vec<Coord>> = parse(file);

	part_one_no(input)
}

pub fn part_one_no(input: Vec<Vec<Coord>>) -> u64 {
	let cave_rocks = cave_rocks(input);

	let max_y = cave_rocks.iter().map(|coord| coord.y).max().unwrap();

	let width = 500 + max_y + 2;

	let mut cave = vec![vec![Tile::Air; width as usize]; (max_y + 2) as usize];

	for pos in cave_rocks {
		cave[pos.y as usize][pos.x as usize] = Tile::Rock;
	}

	for i in 0.. {
		let mut sand = Coord{
			x: 500,
			y: 0,
		};

		while let Some(next) = sand.next(&cave) {
			sand = next;
			if sand.y > max_y {
				return i as u64
			}
		}

		cave[sand.y as usize][sand.x as usize] = Tile::Sand;
	}

	u64::MAX
}

pub fn part_two(file: &str) -> u64 {
	let input: Vec<Vec<Coord>> = parse(file);

	part_two_no(input)
}

pub fn part_two_no(input: Vec<Vec<Coord>>) -> u64 {
	let cave_rocks = cave_rocks(input);

	let max_y = cave_rocks.iter().map(|coord| coord.y).max().unwrap() + 2;

	let width = 500 + max_y + 2;

	let mut cave = vec![vec![Tile::Air; width as usize]; (max_y + 1) as usize];
	cave[max_y as usize] = vec![Tile::Rock; width as usize];

	for pos in cave_rocks {
		cave[pos.y as usize][pos.x as usize] = Tile::Rock;
	}

	for i in 0.. {
		let mut sand = Coord{
			x: 500,
			y: 0,
		};

		while let Some(next) = sand.next(&cave) {
			sand = next;
		}

		cave[sand.y as usize][sand.x as usize] = Tile::Sand;

		if sand == (Coord{ x: 500, y: 0 }) {
			return (i + 1) as u64;
		}
	}

	u64::MAX
}

fn parse(file: &str) -> Vec<Vec<Coord>> {
	let input = std::fs::read_to_string(file).unwrap();

	input
		.lines()
		.map(|line| {
			line.split(" -> ")
				.map(|coord| {
					let (x, y) = coord.split_once(',').unwrap();
					let x: i32 = x.parse().unwrap();
					let y: i32 = y.parse().unwrap();
					Coord{x, y}
				})
				.collect()
		})
		.collect()
}

fn cave_rocks(input: Vec<Vec<Coord>>) -> HashSet<Coord> {
	input
		.iter()
		.flat_map(|path| {
			path.
				iter()
				.tuple_windows()
				.flat_map(|(start, end)| {
					let diff_x = end.x - start.x;
					let diff_y = end.y - start.y;
					let dir = (diff_x.signum(), diff_y.signum());

					let n = diff_x.abs().max(diff_y.abs());

					(0..=n).map(move |amount| {
						let offset_x = amount as i32 * dir.0;
						let offset_y = amount as i32 * dir.1;

						Coord {
							x: start.x + offset_x,
							y: start.y + offset_y,
						}
					})
				})
		})
		.collect()
}
