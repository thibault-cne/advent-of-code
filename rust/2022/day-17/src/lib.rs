// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/25 17:13:35 by Thibault Cheneviere                      \\
//   Updated: 2022/12/26 20:21:55 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use std::ops::{Add, AddAssign};
use std::collections::{HashSet, VecDeque, HashMap};
use utils::{
	files::read_file,
	parsing::parse_lines,
};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Coord {
	x: i32,
	y: i32,
}

impl Add for Coord {
	type Output = Coord;

	fn add(self, rhs: Self) -> Self {
		Coord{
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}	
	}
}

impl AddAssign for Coord {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

struct Shape {
	shape: Vec<Coord>,
}

impl Shape {
	fn new(index: usize) -> Shape {
		let shape = match index % 5 {
			0 => vec![Coord{x: 2, y: 0}, Coord{x: 3, y: 0}, Coord{x: 4, y: 0}, Coord{x: 5, y: 0}],
			1 => vec![Coord{x: 3, y: 2}, Coord{x: 2, y: 1}, Coord{x: 3, y: 1}, Coord{x: 4, y: 1}, Coord{x: 3, y: 0}],
			2 => vec![Coord{x: 2, y: 0}, Coord{x: 3, y: 0}, Coord{x: 4, y: 0}, Coord{x: 4, y: 1}, Coord{x: 4, y: 2}],
			3 => vec![Coord{x: 2, y: 0}, Coord{x: 2, y: 1}, Coord{x: 2, y: 2}, Coord{x: 2, y: 3}],
			4 => vec![Coord{x: 2, y: 1}, Coord{x: 3, y: 1}, Coord{x: 2, y: 0}, Coord{x: 3, y: 0}],
			_ => panic!("Unknown result"),
		};
		Shape{shape: shape}
	}

	fn add(&mut self, delta: Coord) {
		self.shape
			.iter_mut()
			.for_each(|c| *c += delta)
	}

	fn check(&self, delta: Coord, tower: &HashSet<Coord>) -> bool {
		for part in &self.shape {
			let next = *part + delta;
			if next.x < 0 || next.x > 6 || tower.contains(&next) {
				return false;
			}
		}

		true
	}
}

pub fn part_one(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_one_no(split)
}

pub fn part_one_no(split: Vec<String>) -> u64 {
	let rocks = 2022;
	let mut it = split[0].chars().cycle();
	let mut max: u64 = 0;
	let mut tower = HashSet::from([
		Coord{x: 0, y: 0},
		Coord{x: 1, y: 0},
		Coord{x: 2, y: 0},
		Coord{x: 3, y: 0},
		Coord{x: 4, y: 0},
		Coord{x: 5, y: 0},
		Coord{x: 6, y: 0},
	]);

	for i in 0..rocks {
		let mut rock = Shape::new(i);
		
		// Move rock to it's initial place
		rock.add(Coord{x: 0, y: (max as i32) + 4});

		loop {
			let delta = match it.next() {
				Some('>') => Coord{x: 1, y: 0},
				Some('<') => Coord{x: -1, y: 0},
				_ => panic!("Unknown wind dir"),
			};

			if rock.check(delta, &tower) {
				rock.add(delta);
			}

			if rock.check(Coord{x: 0, y: -1}, &tower) {
				rock.add(Coord{x: 0, y: -1});
			} else {
				let t_max = rock.shape.iter().map(|c| c.y).max().unwrap();
				max = max.max(t_max.try_into().unwrap());
				rock.shape.iter().for_each(|c| { tower.insert(*c); });
				break;
			}
		}
	}
	max
}

pub fn part_two(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_two_no(split)
}

pub fn part_two_no(split: Vec<String>) -> u64 {
	let rocks = 1_000_000_000_000;
	let mut it = split[0].chars().cycle();
	let mut max: u64 = 0;
	let mut tower = HashSet::from([
		Coord{x: 0, y: 0},
		Coord{x: 1, y: 0},
		Coord{x: 2, y: 0},
		Coord{x: 3, y: 0},
		Coord{x: 4, y: 0},
		Coord{x: 5, y: 0},
		Coord{x: 6, y: 0},
	]);
	let mut heights = vec![0; 10_000];
	let mut increments = VecDeque::with_capacity(1_000);
	increments.push_back(0);
	let mut past_increments = HashMap::new();

	let mut current_rock_index = 0;
	let mut cycle = 0;


	for i in 0..10_000 {
		if i > 1_000 {
			if past_increments.contains_key(&increments) {
				current_rock_index = i;
				cycle = i - past_increments.get(&increments).unwrap();
				break;
			} else {
				past_increments.insert(increments.clone(), i);
			}
			increments.pop_front();
		}

		let mut rock = Shape::new(i);
		
		// Move rock to it's initial place
		rock.add(Coord{x: 0, y: (max as i32) + 4});

		loop {
			let delta = match it.next() {
				Some('>') => Coord{x: 1, y: 0},
				Some('<') => Coord{x: -1, y: 0},
				_ => panic!("Unknown wind dir"),
			};

			if rock.check(delta, &tower) {
				rock.add(delta);
			}

			if rock.check(Coord{x: 0, y: -1}, &tower) {
				rock.add(Coord{x: 0, y: -1});
			} else {
				let previous_max = max;
				let t_max = rock.shape.iter().map(|c| c.y).max().unwrap();
				max = max.max(t_max.try_into().unwrap());
				rock.shape.iter().for_each(|c| { tower.insert(*c); });
				increments.push_back(max - previous_max);
				break;
			}
		}

		heights[i] = max;
	}

	let height_diff = heights[current_rock_index - 1] - heights[current_rock_index - cycle - 1];

	let n = (rocks - current_rock_index) / cycle;

	for i in current_rock_index..(rocks - n * cycle) {
		let mut rock = Shape::new(i);
		
		// Move rock to it's initial place
		rock.add(Coord{x: 0, y: (max as i32) + 4});

		loop {
			let delta = match it.next() {
				Some('>') => Coord{x: 1, y: 0},
				Some('<') => Coord{x: -1, y: 0},
				_ => panic!("Unknown wind dir"),
			};

			if rock.check(delta, &tower) {
				rock.add(delta);
			}

			if rock.check(Coord{x: 0, y: -1}, &tower) {
				rock.add(Coord{x: 0, y: -1});
			} else {
				let t_max = rock.shape.iter().map(|c| c.y).max().unwrap();
				max = max.max(t_max.try_into().unwrap());
				rock.shape.iter().for_each(|c| { tower.insert(*c); });
				break;
			}
		}
	}

	max += n as u64 * height_diff;
	max
}
