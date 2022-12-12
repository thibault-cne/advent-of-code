// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/09 13:44:14 by Thibault Cheneviere                      \\
//   Updated: 2022/12/09 21:10:57 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;

pub fn part_one(file: &str) -> usize {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_one_no(split);
}

pub fn part_one_no(split: Vec<String>) -> usize {
	let mut res: usize = 0;

	let grid = split
		.iter()
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
		.collect::<Vec<Vec<u8>>>();

	for (i, row) in grid.iter().enumerate() {
		for (j, _col) in row.iter().enumerate() {
			if check_visibility_column(&grid.clone(), i, j) || check_visibility_row(&grid.clone(), i, j) {
				res += 1;
			}
		}
	}

	return res;
}

pub fn part_two(file: &str) -> usize {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_two_no(split);
}

pub fn part_two_no(split: Vec<String>) -> usize {
	let mut res: usize = 0;

	let grid = split
		.iter()
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
		.collect::<Vec<Vec<u8>>>();

	for (i, row) in grid.iter().enumerate() {
		for (j, _col) in row.iter().enumerate() {
			if j == 0 || j == row.len() - 1 || i == 0 || i == grid.len() - 1 {
				continue;
			}

			let temp = calculate_row_score(&grid.clone(), i, j);
			let temp_2 = calculate_column_score(&grid.clone(), i, j);

			if temp * temp_2 > res {
				res = temp * temp_2;
			}
		}
	}

	return res;
}

fn check_visibility_column(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
	let mut res: bool = true;

	for (k, row) in grid.iter().enumerate() {
		if k == i {
			break;
		}

		if row[j] >= grid[i][j] {
			res = false;
		}
	}

	if res {
		return res;
	}

	res = true;

	for (k, row) in grid.iter().enumerate() {
		if k <= i {
			continue;
		}

		if row[j] >= grid[i][j] {
			res = false;
		}
	}

	return res;
}

fn check_visibility_row(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
	let mut res: bool = true;

	for (k, col) in grid[i].iter().enumerate() {
		if k == j {
			break;
		}

		if *col >= grid[i][j] {
			res = false;
		}
	}

	if res {
		return res;
	}

	res = true;

	for (k, col) in grid[i].iter().enumerate() {
		if k <= j {
			continue;
		}

		if *col >= grid[i][j] {
			res = false;
		}
	}

	return res;
}

fn calculate_row_score(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
	let mut res_left: usize = 0;
	let mut res_right: usize = 0;

	for _j in 1..(j+1) {
		res_left += 1;

		if grid[i][j - _j] >= grid[i][j] {
			break;
		}
	}

	for _j in (j+1)..grid[i].len() {
		res_right += 1;

		if grid[i][_j] >= grid[i][j] {
			break;
		}
	}

	return res_left * res_right;
}

fn calculate_column_score(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
	let mut res_up: usize = 0;
	let mut res_down: usize = 0;

	for _i in 1..(i+1) {
		res_up += 1;

		if grid[i - _i][j] >= grid[i][j] {
			break;
		}
	}

	for _i in (i+1)..grid.len() {
		res_down += 1;

		if grid[_i][j] >= grid[i][j] {
			break;
		}
	}

	return res_up * res_down;
}
