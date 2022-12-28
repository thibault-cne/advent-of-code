// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/27 17:07:27 by Thibault Cheneviere                      \\
//   Updated: 2022/12/28 02:45:44 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::{
	files::read_file,
	parsing::parse_lines,
};
use std::collections::VecDeque;

struct State {
	time: u16,
	robots: [u16; 4],
	ressources: [u16; 4],
}

struct Blueprint {
	costs: [[u16; 4]; 4],
}

impl Blueprint {
	fn new(line: &String) -> Blueprint {
		let (_, r) = line.split_once(": ").unwrap();
		let split: Vec<&str> = r.split(". ").collect();

		// Verifiy everithing is good
		assert_eq!(split.len(), 4);

		let robot_ore_cost = split[0].strip_prefix("Each ore robot costs ").unwrap();
		let robot_ore_cost = robot_ore_cost.strip_suffix(" ore").unwrap();
		let robot_ore_cost = robot_ore_cost.parse::<u16>().unwrap();
		let robot_ore_cost = [robot_ore_cost, 0, 0, 0];

		let robot_clay_cost = split[1].strip_prefix("Each clay robot costs ").unwrap();
		let robot_clay_cost = robot_clay_cost.strip_suffix(" ore").unwrap();
		let robot_clay_cost = robot_clay_cost.parse::<u16>().unwrap();
		let robot_clay_cost = [robot_clay_cost, 0, 0, 0];

		let (l, r) = split[2].split_once(" ore and ").unwrap();
		let l = l.strip_prefix("Each obsidian robot costs ").unwrap();
		let r = r.strip_suffix(" clay").unwrap();
		let robot_obsidian_cost = [l.parse::<u16>().unwrap(), r.parse::<u16>().unwrap(), 0, 0];

		let (l, r) = split[3].split_once(" ore and ").unwrap();
		let l = l.strip_prefix("Each geode robot costs ").unwrap();
		let r = r.strip_suffix(" obsidian.").unwrap();
		let robot_geode_cost = [l.parse::<u16>().unwrap(), 0, r.parse::<u16>().unwrap(), 0];
		
		Blueprint{costs: [robot_ore_cost, robot_clay_cost, robot_obsidian_cost, robot_geode_cost]}
	}
}

pub fn part_one(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_one_no(split)
}

pub fn part_one_no(split: Vec<String>) -> u64 {
	let bp = split.iter().map(|line| Blueprint::new(line)).collect::<Vec<Blueprint>>();

	bp.iter()
		.map(|b| max_geodes(&b.costs, 24))
		.enumerate()
		.map(|(idx, res)| (idx + 1) * res as usize)
		.sum::<usize>() as u64
}

pub fn part_two(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_two_no(split)
}

pub fn part_two_no(split: Vec<String>) -> u64 {
	let bp = split.iter().map(|line| Blueprint::new(line)).collect::<Vec<Blueprint>>();

	bp.iter()
		.take(3)
		.map(|b| max_geodes(&b.costs, 32))
		.product::<u64>()
}

fn max_geodes(bp: &[[u16; 4]; 4], max_time: u16) -> u64 {
	let mut max_geodes: u64 = 0;
	let max_ressources = bp.iter()
		.fold([0, 0, 0, u16::MAX], |mut acc, x| {
			acc[0] = acc[0].max(x[0]);
			acc[1] = acc[1].max(x[1]);
			acc[2] = acc[2].max(x[2]);
			acc
		});

	let mut q = VecDeque::new();
	q.push_back(State{
		time: 1,
		robots: [1, 0, 0, 0],
		ressources: [1, 0, 0, 0],
	});

	while let Some(State{
		time,
		robots,
		ressources,
	}) = q.pop_front() {
		for i in 0..bp.len() {
			if robots[i] == max_ressources[i] {
				continue;
			}

			let costs = &bp[i];

			let wait_time = (0..3)
				.map(|idx| {
					match costs[idx] {
						cost if cost <= ressources[idx] => 0,
						_ if robots[idx] == 0 => max_time + 1,
						_ => (costs[idx] - ressources[idx] + robots[idx] - 1) / robots[idx],
					}
				})
				.max()
				.unwrap();

			let new_time = time + wait_time + 1;
			if new_time >= max_time {
				continue;
			}

			let mut new_ressources = [0; 4];
			for idx in 0..robots.len() {
				new_ressources[idx] = ressources[idx] + robots[idx] * (wait_time + 1) - costs[idx];
			}

			let mut new_robots = robots;
			new_robots[i] += 1;

			let remaining_time = max_time - new_time;
			if ((remaining_time - 1) * remaining_time) / 2
				+ new_ressources[3]
				+ remaining_time * new_robots[3]
				< max_geodes as u16 {
					continue;
			}

			q.push_back(State{
				time: new_time,
				robots: new_robots,
				ressources: new_ressources,
			});

		}

		let geodes = ressources[3] + robots[3] * (max_time - time);
		let geodes = geodes as u64;
		max_geodes = geodes.max(max_geodes);
	}

	max_geodes
}

