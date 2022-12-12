// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/09 22:05:40 by Thibault Cheneviere                      \\
//   Updated: 2022/12/12 09:05:11 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;

pub fn part_one(file: &str) -> usize {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_one_no(split);
}

pub fn part_one_no(split: Vec<String>) -> usize {
	let mut t_coord: (isize, isize) = (0, 0);
	let mut h_coord: (isize, isize) = (0, 0);
	let mut hashtag: Vec<(isize, isize)> = vec![(0, 0)];

	for i in split.iter() {
		let temp: Vec<&str> = i.split(' ').collect::<Vec<&str>>();

		for _ in 0..temp[1].parse::<u32>().unwrap() {
			match temp[0] {
				"U" => h_coord.1 += 1,
				"L" => h_coord.0 -= 1,
				"R" => h_coord.0 += 1,
				"D" => h_coord.1 -= 1,
                _ => panic!("Invalid direction"),
			}
			
			t_coord = new_tail_position(h_coord, t_coord);

			if !hashtag.contains(&t_coord) {
				hashtag.push(t_coord);
			}
		}
	}

    return hashtag.len();
}

pub fn part_two(file: &str) -> usize {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_two_no(split);
}

pub fn part_two_no(split: Vec<String>) -> usize {
	let mut snake: Vec<(isize, isize)> = vec![(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
	let mut hashtag: Vec<(isize, isize)> = vec![(0, 0)];

	for i in split.iter() {
		let temp: Vec<&str> = i.split(' ').collect::<Vec<&str>>();

		for _ in 0..temp[1].parse::<u32>().unwrap() {
			match temp[0] {
				"U" => snake[0].1 += 1,
				"L" => snake[0].0 -= 1,
				"R" => snake[0].0 += 1,
				"D" => snake[0].1 -= 1,
                _ => panic!("Invalid direction"),
			}
			
			for i in 1..10 {
				let new_tail = new_tail_position(snake[i - 1], snake[i]);
				snake[i] = new_tail;
			}

			if !hashtag.contains(&snake[9]) {
				hashtag.push(snake[9]);
			}
		}
	}

	return hashtag.len();
}

fn new_tail_position(h_coord: (isize, isize), t_coord: (isize, isize)) -> (isize, isize) {
	if check_position(h_coord, t_coord) {
		return t_coord;
	}

	let mut _t_coord = (t_coord.0, t_coord.1);

	if h_coord.0 > _t_coord.0 {
		_t_coord.0 += 1;
	}
	if h_coord.0 < _t_coord.0 {
		_t_coord.0 -= 1;
	}
	if h_coord.1 > _t_coord.1 {
		_t_coord.1 += 1;
	}
	if h_coord.1 < _t_coord.1 {
		_t_coord.1 -= 1;
	}

	return _t_coord;
}

fn check_position(h_coord: (isize, isize), t_coord: (isize, isize)) -> bool {
	let x: i64 = (h_coord.0 - t_coord.0).try_into().unwrap();
	let y: i64 = (h_coord.1 - t_coord.1).try_into().unwrap();
	let dist: f64 = (x.pow(2) + y.pow(2)) as f64;

	return dist.sqrt() <= 2_f64.sqrt();
}
