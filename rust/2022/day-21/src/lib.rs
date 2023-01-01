// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2023/01/01 13:08:37 by Thibault Cheneviere                      \\
//   Updated: 2023/01/01 15:31:44 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::{
	files::read_file,
	parsing::parse_lines,
};
use std::collections::HashMap;

#[derive(Clone)]
enum Op {
	Num(isize),
	Mul(String, String),
	Add(String, String),
	Div(String, String),
	Sub(String, String),
}

#[derive(Clone)]
struct Monkey {
	op: Op,
	name: String,
}

impl Monkey {
	fn new(line: String) -> Monkey {
		let (l, r) = line.split_once(": ").unwrap();
		let op: Op;
		if let Some(num) = r.parse::<isize>().ok() {
			op = Op::Num(num);
		} else if let Some((lhs, rhs)) = r.split_once(" + ") {
			op = Op::Add(lhs.to_string(), rhs.to_string());
		} else if let Some((lhs, rhs)) = r.split_once(" / ") {
			op = Op::Div(lhs.to_string(), rhs.to_string());
		} else if let Some((lhs, rhs)) = r.split_once(" * ") {
			op = Op::Mul(lhs.to_string(), rhs.to_string());
		} else if let Some((lhs, rhs)) = r.split_once(" - ") {
			op = Op::Sub(lhs.to_string(), rhs.to_string());
		} else {
			unreachable!();
		}
		Monkey{op, name: l.to_string()}
	}

	fn compute(&self, map: &HashMap<String, Self>) -> isize {
		match &self.op {
			Op::Num(num) => *num,
			Op::Mul(n1, n2) => {
				let m1 = map.get(n1).unwrap();
				let m2 = map.get(n2).unwrap();

				m1.compute(map) * m2.compute(map)
			}
			Op::Add(n1, n2) => {
				let m1 = map.get(n1).unwrap();
				let m2 = map.get(n2).unwrap();

				m1.compute(map) + m2.compute(map)
			}
			Op::Div(n1, n2) => {
				let m1 = map.get(n1).unwrap();
				let m2 = map.get(n2).unwrap();

				m1.compute(map) / m2.compute(map)
			}
			Op::Sub(n1, n2) => {
				let m1 = map.get(n1).unwrap();
				let m2 = map.get(n2).unwrap();

				m1.compute(map) - m2.compute(map)
			}
		}
	}

	fn reverse(&self, answer: isize, map: &HashMap<String, Self>) -> Option<isize> {
		if self.name == "humn" {
			return Some(answer);
		}

		let (lhs, rhs) = match &self.op {
			Op::Num(_) => return None,
			Op::Mul(lhs, rhs) => (lhs, rhs),
			Op::Add(lhs, rhs) => (lhs, rhs),
			Op::Div(lhs, rhs) => (lhs, rhs),
			Op::Sub(lhs, rhs) => (lhs, rhs),
		};

		let rhs_comp = map.get(rhs).unwrap().compute(map);
		let lhs_humn = match &self.op {
			Op::Num(_) => unreachable!(),
			Op::Mul(lhs, _) => map.get(lhs).unwrap().reverse(answer / rhs_comp, map),
			Op::Add(lhs, _) => map.get(lhs).unwrap().reverse(answer - rhs_comp, map),
			Op::Div(lhs, _) => map.get(lhs).unwrap().reverse(answer * rhs_comp, map),
			Op::Sub(lhs, _) => map.get(lhs).unwrap().reverse(answer + rhs_comp, map),
		};


		let lhs_comp = map.get(lhs).unwrap().compute(map);
		let rhs_humn = match &self.op {
			Op::Num(_) => unreachable!(),
			Op::Mul(_, rhs) => map.get(rhs).unwrap().reverse(answer / lhs_comp, map),
			Op::Add(_, rhs) => map.get(rhs).unwrap().reverse(answer - lhs_comp, map),
			Op::Div(_, rhs) => map.get(rhs).unwrap().reverse(lhs_comp / answer, map),
			Op::Sub(_, rhs) => map.get(rhs).unwrap().reverse(lhs_comp - answer, map),
		};

		lhs_humn.or(rhs_humn)
	}
}

pub fn part_one(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_one_no(split)
}

pub fn part_one_no(split: Vec<String>) -> u64 {
	let mut map = HashMap::new();
	split
		.iter()
		.for_each(|line| {
			let m = Monkey::new(line.to_string());
			map.insert(m.name.clone(), m.clone());
		});

	let root = map.get("root").unwrap();

	root.compute(&map) as u64
}

pub fn part_two(file: &str) -> u64 {
	let split = parse_lines(read_file(file));

	part_two_no(split)
}

pub fn part_two_no(split: Vec<String>) -> u64 {
	let mut map = HashMap::new();
	split
		.iter()
		.for_each(|line| {
			let m = Monkey::new(line.to_string());
			map.insert(m.name.clone(), m.clone());
		});

	let root = map.get("root").unwrap();

	let (lhs, rhs) = match &root.op {
		Op::Num(_) => unreachable!(),
		Op::Mul(lhs, rhs) => (lhs, rhs),
		Op::Add(lhs, rhs) => (lhs, rhs),
		Op::Div(lhs, rhs) => (lhs, rhs),
		Op::Sub(lhs, rhs) => (lhs, rhs),
	};

	let lhs = map.get(lhs).unwrap();
	let rhs = map.get(rhs).unwrap();

	let lhs_comp = lhs.compute(&map);
	let rhs_comp = rhs.compute(&map);

	lhs.reverse(rhs_comp, &map)
		.or(rhs.reverse(lhs_comp, &map))
		.expect("One should work") as u64
}
