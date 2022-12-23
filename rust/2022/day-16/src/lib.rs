// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/17 22:42:47 by Thibault Cheneviere                      \\
//   Updated: 2022/12/23 10:26:22 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{VecDeque, HashSet};

#[derive(Default, Debug)]
struct Network {
	start: usize,
	valves: Vec<Valve>,
	distance_map: Vec<Vec<Option<u32>>>,
}

#[derive(Default, Debug)]
struct Valve {
	_id: usize,
	name: String,
	flow_rate: i32,
	tunnels: Vec<usize>,
	tunnel_names: Vec<String>,
}

#[derive(Clone)]
enum AgentTask {
	MoveTo(usize, u32),
	Ready(usize),
}

pub fn part_one(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_one_no(split)
}

pub fn part_one_no(split: Vec<String>) -> u64 {
	let network = parse(split).unwrap();
	let mut open_valves = vec![false; network.valves.len()];

	let (tp, _) = release_pressure(
		&network,
		network.start,
		&mut open_valves,
		30,
		0,
		0,
		0,
	);
	
	tp
}

pub fn part_two(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_two_no(split)
}

pub fn part_two_no(split: Vec<String>) -> u64 {
	let network = parse(split).unwrap();
	let mut open_valves = vec![false; network.valves.len()];
	let tasks = vec![
		AgentTask::Ready(network.start),
		AgentTask::Ready(network.start)
	];

	let tp = release_pressure_2(
		&network,
		tasks,
		&mut open_valves,
		26,
		0,
		0,
	);

	tp
		
}

fn release_pressure(network: &Network, current: usize, open_valves: &mut Vec<bool>, time: i32, total_pressure: u64, ppm: u64, level: usize) -> (u64, Vec<bool>) {
	if time <= 0 {
		return (total_pressure, open_valves.clone());
	}

	let mut max_tp = total_pressure + ppm * time as u64;
	let mut max_state: Option<Vec<bool>> = None;

	let should_open = !open_valves[current] && network.valves[current].flow_rate > 0;

	if should_open {
		open_valves[current] = true;
		let (tp, state) = release_pressure(
			network,
			current,
			open_valves,
			time - 1,
			total_pressure + 1 * ppm,
			ppm + network.valves[current].flow_rate as u64,
			level + 1,
		);

		if tp > max_tp {
			max_tp = tp;
			max_state = Some(state);
		}
		open_valves[current] = false;
	}

	if !should_open || level == 0 {
		for (to, dist) in network.distance_map[current]
			.iter()
			.enumerate()
			.filter(|(_, d)| d.is_some())
		{
			let dist = dist.unwrap();
			if dist > time as u32 || open_valves[to] || network.valves[to].flow_rate <= 0 {
				continue;
			}


			let (tp, state) = release_pressure(
				network,
				to,
				open_valves,
				time - dist as i32,
				total_pressure + (dist as u64) * ppm,
				ppm,
				level + 1,
			);

			if tp > max_tp {
				max_tp = tp;
				max_state = Some(state);
			}
		}
	}

	let max_state = match max_state {
		Some(state) => state,
		None => open_valves.clone(),
	};

	(max_tp, max_state)
}

fn release_pressure_2(
	network: &Network,
	tasks: Vec<AgentTask>,
	open_valves: &mut Vec<bool>,
	time: i32,
	total_pressure: u64,
	mut ppm: u64,
) -> u64 {
	let mut restore_valves = Vec::new();
	let tasks = tasks
		.into_iter()
		.map(|t| match t {
			AgentTask::Ready(_) => t,
			AgentTask::MoveTo(to, 0) => {
				open_valves[to] = true;
				restore_valves.push(to);
				ppm += network.valves[to].flow_rate as u64;
				AgentTask::Ready(to)
			}
			AgentTask::MoveTo(to, dist) => AgentTask::MoveTo(to, dist - 1),
		})
		.collect_vec();

	
	let mut new_tasks: Vec<Vec<AgentTask>> = vec![vec![]; tasks.len()];
	for i in 0..tasks.len() {
		let task = &tasks[i];
		match task {
			AgentTask::MoveTo(to, dist) => new_tasks[i].push(AgentTask::MoveTo(*to, *dist)),
			AgentTask::Ready(curr) => new_tasks[i].append(&mut next_actions(network, open_valves, *curr)),
		}
	}

	let mut tasks = Vec::new();
	if time > 1 {
		for a1 in &new_tasks[0] {
			for a2 in &new_tasks[1] {
				match (a1, a2) {
					(AgentTask::Ready(_), AgentTask::Ready(_)) => continue,
					(AgentTask::MoveTo(v1, _), AgentTask::MoveTo(v2, _)) if v1 == v2 => continue,
					(AgentTask::MoveTo(_, _), AgentTask::Ready(_)) if (new_tasks[1].len() > 0 && tasks.len() > 0) => continue, 
					(AgentTask::Ready(_), AgentTask::MoveTo(_, _)) if (new_tasks[0].len() > 0 && tasks.len() > 0) => continue, 
					_ => tasks.push(vec![a1.clone(), a2.clone()]),
				}
			}	
		}
	}

	let mut max_tp = total_pressure + (time as u64) * ppm;
	let tp = tasks
		.into_iter()
		.enumerate()
		.par_bridge()
		.map(|(_, task)| release_pressure_2(
				network,
				task,
				&mut open_valves.clone(),
				time - 1,
				total_pressure + ppm,
				ppm,
			)
		)
		.max();
	tp.map(|x| max_tp = max_tp.max(x));

	restore_valves.iter().for_each(|&v| open_valves[v] = false);
	return max_tp;
}

fn next_actions(network: &Network, open_valves: &mut Vec<bool>, current: usize) -> Vec<AgentTask> {
	let mut tasks: Vec<AgentTask> = Vec::new();

	for valve in (0..network.valves.len()).filter(|&v| !open_valves[v] && network.valves[v].flow_rate > 0) {
		if let Some(dist) = network.distance_map[current][valve] {
			tasks.push(AgentTask::MoveTo(valve, dist));
		}
	}

	tasks.push(AgentTask::Ready(current));

	tasks
}

fn parse(split: Vec<String>) -> Option<Network> {
	let mut network = Network::default();
	for line in split.iter() {
		let (left, right) = line.split_once("; ").unwrap();
		let left = left.split(' ').map(|item| item.to_string()).collect::<Vec<String>>();
		let name = left[1].clone();
		let sr = left[4].strip_prefix("rate=").unwrap();
		let r = sr.parse::<i32>().unwrap();
		let names: Vec<String>;
		if right.contains("tunnels lead to valves ") {
			let right = right.strip_prefix("tunnels lead to valves ").unwrap();
			names = right.split(", ").map(|name| name.to_string()).collect::<Vec<String>>();
		} else {
			let right = right.strip_prefix("tunnel leads to valve ").unwrap();
			names = vec![right.to_string()];
		}

		network.valves.push(Valve{
			_id: network.valves.len(),
			name: name,
			flow_rate: r,
			tunnel_names: names,
			..Default::default()
		});
	}

	network.start = network.valves.iter().position(|valve| valve.name == "AA").unwrap();

	for index in 0..network.valves.len() {
		let mut idx = network.valves[index]
			.tunnel_names
			.iter()
			.map(|name| network.valves.iter().position(|valve| &valve.name == name).unwrap())
			.collect::<Vec<usize>>();

		network.valves[index].tunnels.append(&mut idx);
	}

	network.distance_map = create_distance_map(&network).unwrap();

	Some(network)
}

fn create_distance_map(network: &Network) -> Option<Vec<Vec<Option<u32>>>> {
	let mut distance_map: Vec<Vec<Option<u32>>> = vec![vec![None; network.valves.len()]; network.valves.len()];

	for (from, to) in (0..network.valves.len())
		.combinations(2)
		.map(|x| (x[0], x[1]))
	{
		let mut visited = HashSet::from([from]);
		let mut queue = VecDeque::from([(from, 0)]);
		while let Some((curr, dist)) = queue.pop_front() {
			if curr == to {
				distance_map[from][to] = Some(dist);
				distance_map[to][from] = Some(dist);
				break;
			}

			for &next in &network.valves[curr].tunnels {
				if visited.insert(next) {
					queue.push_back((next, dist + 1));
				}
			}
		}
	}

	Some(distance_map)
}
