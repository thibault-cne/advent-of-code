// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/17 12:49:54 by Thibault Cheneviere                      \\
//   Updated: 2022/12/17 22:10:03 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;
use std::ops::RangeInclusive;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
	x: i32,
	y: i32,
}

impl Coord {
	fn dist(&self, other: &Self) -> i32 {
		(self.x - other.x).abs() + (self.y - other.y).abs()
	}
}

pub fn part_one(file: &str) -> u64 {
	let split: Vec<String> = parse_lines(read_file(file));

	part_one_no(split)
}

pub fn part_one_no(split: Vec<String>) -> u64 {
	let input = parse(split);
	let row: i32 = 2_000_000;

	let covered = ranges(row, &input)
		.iter()
		.map(|range| range.end() - range.start() + 1)
		.sum::<i32>() as u64;

	let beacon: u64 = input
		.iter()
		.map(|pair| pair[1])
		.filter(|beacon| beacon.y == row)
		.map(|beacon| beacon.x)
		.dedup()
		.count().try_into().unwrap();

	covered - beacon
}

pub fn part_two(file: &str) -> u64 {
	let split: Vec<String> = parse_lines(read_file(file));

	part_two_no(split)
}

pub fn part_two_no(split: Vec<String>) -> u64 {
	let input = parse(split);
	let size: u64 = 4_000_000;

	let (row, col_ranges) = (0..size)
		.rev()
		.map(|row| (row, ranges(row.try_into().unwrap(), &input)))
		.find(|(_, col_ranges)| col_ranges.len() > 1)
		.unwrap();

	let col = col_ranges.first().unwrap().end() + 1;

	let row = row as u64;
	let col = col as u64;

	col * size + row
}

fn beacon_row_ranges(sensor: &Coord, beacon: &Coord, row: i32) -> Option<RangeInclusive<i32>> {
	let radius = sensor.dist(beacon);
	let offset = radius - (sensor.y - row).abs();
	if offset < 0 {
		None
	} else {
		Some((sensor.x - offset)..=(sensor.x + offset))
	}
}

fn ranges(row: i32, input: &[[Coord; 2]]) -> Vec<RangeInclusive<i32>> {
	let mut range: Vec<RangeInclusive<i32>> = input
		.iter()
		.flat_map(|pairs| beacon_row_ranges(&pairs[0], &pairs[1], row))
		.collect();
	range.sort_unstable_by_key(|range| *range.start());

	let mut merged_ranges = vec![range[0].clone()];
	for next in &range[1..] {
		let last_idx = merged_ranges.len() - 1;
		let last = &merged_ranges[last_idx];

		if next.start() <= last.end() || last.end() + 1 == *next.start() {
			if next.end() > last.end() {
				let new = *last.start()..=*next.end();
				merged_ranges[last_idx] = new;
			}
		} else {
			merged_ranges.push(next.clone());
		}
	}

	merged_ranges
}

fn parse(split: Vec<String>) -> Vec<[Coord; 2]> {
	split
		.iter()
		.map(|line| {
			let (sensor, beacon) = line.split_once(": ").unwrap();
			let sensor = sensor.strip_prefix("Sensor at ").unwrap();
			let beacon = beacon.strip_prefix("closest beacon is at ").unwrap();
			let (sx, sy) = sensor.split_once(", ").unwrap();
			let (bx, by) = beacon.split_once(", ").unwrap();

			let sx = sx.strip_prefix("x=").unwrap();
			let sy = sy.strip_prefix("y=").unwrap();
			let bx = bx.strip_prefix("x=").unwrap();
			let by = by.strip_prefix("y=").unwrap();

			let c_b = Coord{ x: bx.parse::<i32>().unwrap(), y: by.parse::<i32>().unwrap() };
			let c_s = Coord{ x: sx.parse::<i32>().unwrap(), y: sy.parse::<i32>().unwrap() };

			[c_s, c_b]
		})
		.collect::<Vec<[Coord; 2]>>()
}
