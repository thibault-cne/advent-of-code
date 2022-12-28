// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/27 15:06:40 by Thibault Cheneviere                      \\
//   Updated: 2022/12/27 17:06:10 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::{
	files::read_file,
	parsing::parse_lines,
};
use std::collections::HashSet;

#[derive(Hash, Eq, Default, PartialEq, Clone, Copy)]
struct Cube {
	x: i32,
	y: i32,
	z: i32,
}

impl Cube {
	fn new(x: i32, y: i32, z: i32) -> Cube {
		Cube{x, y, z}
	}

	fn neighbours(&self) -> Vec<Cube> {
		let n = vec![
			Cube{x: self.x - 1, y: self.y, z: self.z},
			Cube{x: self.x + 1, y: self.y, z: self.z},
			Cube{x: self.x, y: self.y - 1, z: self.z},
			Cube{x: self.x, y: self.y + 1, z: self.z},
			Cube{x: self.x, y: self.y, z: self.z - 1},
			Cube{x: self.x, y: self.y, z: self.z + 1},
		];

		n
	}

	fn in_bounds(&self, bounds: &[Self; 2]) -> bool {
		let [min, max] = bounds;
		self.x >= min.x - 1
			&& self.y >= min.y - 1
			&& self.z >= min.z - 1
			&& self.x <= max.x + 1
			&& self.y <= max.y + 1
			&& self.z <= max.z + 1
	}
}

pub fn part_one(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_one_no(split)
}

pub fn part_one_no(split: Vec<String>) -> u64 {
	let mut set: HashSet<Cube> = HashSet::new();

	split.iter().for_each(|line| {
		let s: Vec<&str> = line.split(",").collect();
		let c = Cube::new(s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap(), s[2].parse::<i32>().unwrap());
		set.insert(c);
	});


	set.iter().map(|c| {
		let n = c.neighbours();
		6 - n.iter().map(|n| {
			if set.contains(&n) {
				return 1;
			}

			0
		})
		.sum::<u64>()
	})
	.sum::<u64>()
}

pub fn part_two(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_two_no(split)
}

pub fn part_two_no(split: Vec<String>) -> u64 {
	let mut set: HashSet<Cube> = HashSet::new();

	split.iter().for_each(|line| {
		let s: Vec<&str> = line.split(",").collect();
		let c = Cube::new(s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap(), s[2].parse::<i32>().unwrap());
		set.insert(c);
	});

	let exposed = exposed(&set);

	set.iter()
		.flat_map(|c| c.neighbours())
		.filter(|n| exposed.contains(&n))
		.count() as u64

}

fn bounds(set: &HashSet<Cube>) -> [Cube; 2] {
	set.iter().fold([Cube::default(), Cube::default()], |[mut min, mut max], c| {
		min.x = min.x.min(c.x);
		min.y = min.y.min(c.y);
		min.z = min.z.min(c.z);
		max.x = max.x.max(c.x);
		max.y = max.y.max(c.y);
		max.z = max.z.max(c.z);
		[min, max]
	})
}

fn exposed(set: &HashSet<Cube>) -> HashSet<Cube> {
	let bounds = bounds(set);

	let mut exposed: HashSet<Cube> = HashSet::new();

	let start = Cube::default();
	let mut stack: Vec<Cube> = Vec::new();
	let mut seen: HashSet<Cube> = HashSet::new();

	stack.push(start);
	seen.insert(start);

	while let Some(c) = stack.pop() {
		for n in c.neighbours() {
			if set.contains(&n) || !n.in_bounds(&bounds) {
				continue;
			}

			if seen.insert(n) {
				stack.push(n);
				exposed.insert(n);
			}
		}
	}

	exposed
}
