use std::collections::VecDeque;

use super::DaySolver;

// This is the multiple of all prime numbers between
// 1 and 23
const FACTORS: i64 = 223_092_870;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Num(i64),
    Old,
}

impl Operand {
    fn new(s: &str) -> Self {
        if let Ok(parsed) = s.parse::<i64>() {
            Self::Num(parsed)
        } else {
            Self::Old
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Operand),
    Multiply(Operand),
}

impl Operation {
    fn apply(&self, num: i64) -> i64 {
        match self {
            Operation::Add(operand) => match operand {
                Operand::Num(val) => num + val,
                Operand::Old => num + num,
            },
            Operation::Multiply(operand) => match operand {
                Operand::Num(val) => num * val,
                Operand::Old => num * num,
            },
        }
    }
}

fn reduce(num: i64) -> i64 {
    // we reduce the number to have the information we really care about.
    // which is how far away we are from the next prime multiple
    num % FACTORS
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisible_by: i64,
    true_: usize,
    false_: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    test: Test,
    inspected: usize,
}

fn get_monkeys(s: &str) -> Vec<Monkey> {
    let mut res = Vec::new();
    let mut lines = s.lines().peekable();
    while let Some(_) = lines.next() {
        // now we parse starting items
        let mut items = VecDeque::new();
        let starting_items = lines.next().unwrap();
        let mut starting_items = starting_items.split(',');
        // the first includes the text, lets trim it!
        let first = starting_items.next().unwrap();
        let first_num = first
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();
        items.push_back(first_num);
        while let Some(item) = starting_items.next() {
            items.push_back(item.trim().parse::<i64>().unwrap());
        }

        // Now we parse the operation
        let operation = lines.next().unwrap();
        let operation = if operation.contains('*') {
            let num = Operand::new(operation.split('*').last().unwrap().trim());
            Operation::Multiply(num)
        } else {
            let num = Operand::new(operation.split('+').last().unwrap().trim());
            Operation::Add(num)
        };

        // now we parse the divisible by line:
        let divisiable_by = lines.next().unwrap();
        let test_num = divisiable_by
            .split("by")
            .last()
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();
        // now we parse the true case
        let true_num = lines
            .next()
            .unwrap()
            .split("monkey")
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let false_num = lines
            .next()
            .unwrap()
            .split("monkey")
            .last()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let test = Test {
            divisible_by: test_num,
            true_: true_num,
            false_: false_num,
        };
        res.push(Monkey {
            items: items,
            operation: operation,
            test: test,
            inspected: 0,
        });
        if let Some(line) = lines.peek() {
            if line.is_empty() {
                lines.next();
            }
        }
    }
    res
}

pub struct Solver;

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let mut monkeys = get_monkeys(input_txt);
        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                let size = monkeys[i].items.len();
                let test = monkeys[i].test;
                let operation = monkeys[i].operation;
                for j in 0..size {
                    let item = monkeys[i].items[j];

                    let new_val = reduce(operation.apply(item));
                    if new_val % test.divisible_by == 0 {
                        let idx = test.true_;
                        monkeys[idx].items.push_back(new_val);
                    } else {
                        let idx = test.false_;
                        monkeys[idx].items.push_back(new_val);
                    }
                }
                monkeys[i].items.clear();
                monkeys[i].inspected += size;
            }
        }
        let mut sorted_res = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
        sorted_res.sort_by(|a, b| b.cmp(a));
        (sorted_res[0] * sorted_res[1]).to_string()
    }

    fn q1(&self, input_txt: &str) -> String {
        let mut monkeys = get_monkeys(input_txt);
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let size = monkeys[i].items.len();
                let test = monkeys[i].test;
                let operation = monkeys[i].operation;
                for j in 0..size {
                    let item = monkeys[i].items[j];

                    let new_val = operation.apply(item) / 3;
                    if new_val % test.divisible_by == 0 {
                        let idx = test.true_;
                        monkeys[idx].items.push_back(new_val);
                    } else {
                        let idx = test.false_;
                        monkeys[idx].items.push_back(new_val);
                    }
                }
                monkeys[i].items.clear();
                monkeys[i].inspected += size;
            }
        }
        let mut sorted_res = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
        sorted_res.sort_by(|a, b| b.cmp(a));
        (sorted_res[0] * sorted_res[1]).to_string()
    }
}
