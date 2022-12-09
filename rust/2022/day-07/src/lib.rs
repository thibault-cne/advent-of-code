// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/07 17:36:06 by Thibault Cheneviere                      \\
//   Updated: 2022/12/09 13:29:12 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

use utils::files::read_file;
use utils::parsing::parse_lines;
use std::rc::Rc;
use std::cell::RefCell;

struct File {
	name: String,
	size: usize,
}

impl File {
	pub fn new(name: String, size: usize) -> File {
		return File{name: name, size: size};
	}

	pub fn get_size(&self) -> usize {
		return self.size;
	}
}

struct Dir {
	name: String,
	sub_dir: Vec<Rc<RefCell<Dir>>>,
	sub_files: Vec<Rc<RefCell<File>>>,
	parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
	pub fn new(name: String, parent: Option<Rc<RefCell<Dir>>>) -> Dir {
		return Dir{name: name, sub_dir: Vec::new(), sub_files: Vec::new(), parent: parent};
	}

	pub fn get_parent(&mut self) -> Option<Rc<RefCell<Dir>>> {
		if let Some(parent) = &self.parent {
			return Some(Rc::clone(&parent));
		}

		return None;
	}

	pub fn get_name(&self) -> String {
		return self.name.clone();
	}

	pub fn get_sub_dir(&self) -> Vec<Rc<RefCell<Dir>>> {
		return self.sub_dir
			.iter()
			.map(|dir| Rc::clone(&dir))
			.collect::<Vec<Rc<RefCell<Dir>>>>();
	}

	pub fn cd(&mut self, name: String) -> Option<Rc<RefCell<Dir>>> {
		let temp: Vec<Rc<RefCell<Dir>>> = self.sub_dir
			.iter()
			.map(|dir| Rc::clone(dir))
			.filter(|dir| dir.borrow().name.as_str() == name)
			.collect();
			

		if temp.len() == 0 {
			return None;
		}

		return Some(Rc::clone(&temp[0]));
	}

	pub fn cd_back(&mut self) -> Option<Rc<RefCell<Dir>>> {
		return self.get_parent();
	}

	pub fn cd_root(&mut self) -> Option<Rc<RefCell<Dir>>> {
		let _back = self.cd_back();

		if _back.is_none() {
			return None;
		}

		let mut back = _back.unwrap();

		loop {
			let name = back
				.borrow()
				.get_name();

			if name.as_str() == "/" {
				break;
			}
			
			let temp = back
				.borrow_mut()
				.cd_back()
				.unwrap();

			back = temp;
		}

		return Some(Rc::clone(&back));
	}

	pub fn add_file(&mut self, name: String, size: usize) {
		let file: File = File::new(name, size);

		self.sub_files.push(Rc::new(RefCell::new(file)));
	}

	pub fn add_dir(&mut self, name: String, _self: Option<Rc<RefCell<Dir>>>) {
		let dir: Dir = Dir::new(name, Some(Rc::clone(&_self.unwrap())));

		self.sub_dir.push(Rc::new(RefCell::new(dir)));
	}

	pub fn size(&self) -> usize {
		let mut res: usize = 0;

		for file in self.sub_files.iter() {
			res += file.borrow().get_size();
		}

		for dir in self.sub_dir.iter() {
			res += dir.borrow().size();
		}

		return res;
	}
}

pub fn part_one(file: &str) -> usize {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_one_no(split);
}

pub fn part_one_no(split: Vec<String>) -> usize {
	let mut root: Rc<RefCell<Dir>> = Rc::new(RefCell::new(Dir::new(String::from("/"), None)));

	for line in split {
		if line.as_str() != "" {
			let temp = exec_line(line, Rc::clone(&root));

			if temp.is_some() {
				root = temp.unwrap();
			}
		}
	}

	let temp = root.borrow_mut().cd_root();

	if temp.is_some() {
		root = temp.unwrap();
	}

	return get_result_one(Rc::clone(&root), 100000);
}

pub fn part_two(file: &str) -> usize {
	let split: Vec<String> = parse_lines(read_file(file));

	return part_two_no(split);
}

pub fn part_two_no(split: Vec<String>) -> usize {
	let mut root: Rc<RefCell<Dir>> = Rc::new(RefCell::new(Dir::new(String::from("/"), None)));

	for line in split {
		if line.as_str() != "" {
			let temp = exec_line(line, Rc::clone(&root));

			if temp.is_some() {
				root = temp.unwrap();
			}
		}
	}

	let temp = root.borrow_mut().cd_root();

	if temp.is_some() {
		root = temp.unwrap();
	}

	return get_result_two(Rc::clone(&root));
}

fn exec_line(line: String, root: Rc<RefCell<Dir>>) -> Option<Rc<RefCell<Dir>>> {
	let temp: Vec<String> = line
		.split(' ')
		.map(|_str| _str.to_string())
		.collect();

	let mut clone = root.borrow_mut();

	if temp[0].as_str() == "$" && temp[1].as_str() == "cd" {
		if temp[2].as_str() == ".." {
			if let Some(temp) = clone.cd_back() {
				return Some(Rc::clone(&temp));
			}

		} else {
			if let Some(temp) = clone.cd(temp[2].clone()) {
				return Some(Rc::clone(&temp));
			}
		}

		return None;
	}

	if temp[0].as_str() == "$" {
		return None;
	}

	if temp[0].as_str() == "dir" {
		clone.add_dir(temp[1].clone(), Some(Rc::clone(&root)));		
	} else {
		clone.add_file(temp[1].clone(), temp[0].parse::<usize>().unwrap());
	}

	return None;
}

fn get_result_one(root: Rc<RefCell<Dir>>, threshold: usize) -> usize {
	let mut wait = root.borrow().get_sub_dir();
	let mut res: usize = 0;

	loop {
		if wait.len() == 0 {
			break;
		}

		let temp = wait.pop().unwrap();

		let _size = temp.borrow().size();

		if _size <= threshold {
			res += _size;
		}

		wait.append(&mut temp.borrow().get_sub_dir());
	}


	return res;
}

fn get_result_two(root: Rc<RefCell<Dir>>) -> usize {
	let root_size: usize = root.borrow().size();

	let threshold: usize = root_size - 40000000;

	let mut wait = root.borrow().get_sub_dir();
	let mut res: usize = root_size;

	loop {
		if wait.len() == 0 {
			break;
		}

		let temp = wait.pop().unwrap();
		let _size = temp.borrow().size();

		if _size >= threshold && _size < res {
			res = _size;
		}

		wait.append(&mut temp.borrow().get_sub_dir());
	}

	return res;
}
