// ************************************************************************** \\
//                                                                            \\
//                                                                            \\
//   lib.rs                                                                   \\
//                                                                            \\
//   By: Thibault Cheneviere <thibault.cheneviere@telecomnancy.eu>            \\
//                                                                            \\
//   Created: 2022/12/14 14:05:09 by Thibault Cheneviere                      \\
//   Updated: 2022/12/15 11:51:33 by Thibault Cheneviere                      \\
//                                                                            \\
// ************************************************************************** \\

#[derive(Clone)]
enum OpType {
	Mul(Value),
	Add(Value),
}

impl OpType {
	fn eval(&self, old: u64) -> u64 {
		match self {
			OpType::Mul(val) => old * val.number(old),
			OpType::Add(val) => old + val.number(old), 
		}
	}
}

#[derive(Clone)]
enum Value {
	Old,
	Num(u64),
}

impl Value {
	fn number(&self, old: u64) -> u64 {
		match self {
			Value::Old => old,
			Value::Num(n) => *n,
		}
	}
}

#[derive(Clone)]
pub struct Monkey {
	items: Vec<u64>,
	op: OpType,
	test: Test,
}

#[derive(Clone)]
struct Test {
	test_value: u64,
	positive_test: usize,
	negative_test: usize,
}

pub fn part_one(file: &str) -> usize {
	let monkeys: Option<Vec<Monkey>> = parse(file);

	return part_one_no(monkeys);
}

pub fn part_one_no(monkeys: Option<Vec<Monkey>>) -> usize {
	let mut monkeys: Vec<Monkey> = monkeys.unwrap();
	let mut inspections: Vec<u64> = vec![0; monkeys.len()];

	for _ in 0..20 {
		for i in 0..monkeys.len() {
			let items: Vec<u64> = monkeys[i].items.drain(..).collect();
			let monkey = monkeys[i].clone();

			for old in items {
				// Incr inspections
				inspections[i] += 1;

				// Operation
				let new = monkey.op.eval(old);

				// Divide by 3
				let new  = new / 3;

				// Test
				let id = if new % monkey.test.test_value == 0 {
					monkey.test.positive_test
				} else {
					monkey.test.negative_test
				};

				let receiver = &mut monkeys[id];
				receiver.items.push(new);
			}
		}
	}

	inspections.sort_unstable();
	return inspections.iter().rev().take(2).product::<u64>() as usize;
}

pub fn part_two(file: &str) -> usize {
	let monkeys: Option<Vec<Monkey>> = parse(file);

	return part_two_no(monkeys);
}

pub fn part_two_no(monkeys: Option<Vec<Monkey>>) -> usize {
	let mut monkeys: Vec<Monkey> = monkeys.unwrap();
	let mut inspections: Vec<u64> = vec![0; monkeys.len()];
	let common_multiple: u64 = monkeys.iter().map(|m| m.test.test_value).product::<u64>();

	for _ in 0..10_000 {
		for i in 0..monkeys.len() {
			let items: Vec<u64> = monkeys[i].items.drain(..).collect();
			let monkey = monkeys[i].clone();

			for old in items {
				// Incr inspections
				inspections[i] += 1;

				// Operation
				let new = monkey.op.eval(old);

				// Keep the new value low
				let new  = new % common_multiple;

				// Test
				let id = if new % monkey.test.test_value == 0 {
					monkey.test.positive_test
				} else {
					monkey.test.negative_test
				};

				let receiver = &mut monkeys[id];
				receiver.items.push(new);
			}
		}
	}

	inspections.sort_unstable();
	return inspections.iter().rev().take(2).product::<u64>() as usize;
}

fn parse(file: &str) -> Option<Vec<Monkey>> {
    let input = std::fs::read_to_string(file).ok()?;

    let mut monkeys = Vec::new();
    for block in input.split("\n\n") {
        let mut lines = block.lines().skip(1);

        let (_, items) = lines.next()?.split_once(": ")?;
        let items = items
            .split_terminator(", ")
            .filter_map(|s| s.parse().ok())
            .collect();

        let (_, operation) = lines.next()?.split_once("= old ")?;
        let (operator, operand) = operation.split_once(" ")?;
        let operand = match operand {
            "old" => Value::Old,
            _ => {
                let n = operand.parse().ok()?;
                Value::Num(n)
            }
        };

        let (_, test_value) = lines.next()?.rsplit_once(" ")?;
        let test_value = test_value.parse().ok()?;
        let (_, positive_test) = lines.next()?.rsplit_once(" ")?;
        let positive_test = positive_test.parse().ok()?;
        let (_, negative_test) = lines.next()?.rsplit_once(" ")?;
        let negative_test = negative_test.parse().ok()?;

        let operation = match operator {
            "+" => OpType::Add(operand),
            "*" => OpType::Mul(operand),
            _ => panic!("Inalid input"),
        };

        let test = Test {
            test_value,
            positive_test,
            negative_test,
        };

        let monkey = Monkey {
            items,
            op: operation,
            test: test,
        };

        monkeys.push(monkey);
    }

    Some(monkeys)
}
