use std::collections::HashMap;

use super::DaySolver;

enum Instruction {
    Addx(i32),
    Noop,
}

impl Instruction {
    fn new(s: &str) -> Self {
        match s {
            "noop" => Self::Noop,
            s if s.starts_with("addx") => {
                let mut split = s.split_whitespace();
                split.next();
                let num = split.next().unwrap().parse::<i32>().unwrap();
                Self::Addx(num)
            }
            _ => panic!("Unknown instruction"),
        }
    }
}

pub struct Solver;

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let instructions = input_txt.lines().map(Instruction::new).collect::<Vec<_>>();
        let mut res = String::new();
        let mut x = 1;
        let mut instructions = instructions.iter();
        let mut take_next = None;
        for cycle in 1..=240 {
            let position = (cycle - 1) % 40;
            let position = position % 40;
            let x_range = [x - 1, x, x + 1];
            if position == 0 {
                res.push('\n');
            };
            if x_range.contains(&position) {
                res.push('#');
            } else {
                res.push('.');
            }
            if let Some(val) = take_next {
                x += val;
                take_next = None;
            } else {
                let instruction = instructions.next().unwrap();
                match instruction {
                    Instruction::Addx(val) => {
                        take_next = Some(*val);
                    }
                    _ => (),
                }
            }
        }
        res
    }

    fn q1(&self, input_txt: &str) -> String {
        let instructions = input_txt.lines().map(Instruction::new).collect::<Vec<_>>();
        let mut results = Vec::new();
        let mut cycle = 0;
        let mut x = 1;
        for instruction in instructions {
            match instruction {
                Instruction::Addx(val) => {
                    x += val;
                    cycle += 2
                }
                Instruction::Noop => cycle += 1,
            }
            results.push((cycle, x));
        }
        let mut res = HashMap::new();
        let candidates = [20, 60, 100, 140, 180, 220];
        let mut prev = 0;
        for result in results {
            for val in candidates {
                if result.0 >= val {
                    _ = res.entry(val).or_insert(prev * val);
                }
            }
            prev = result.1;
        }
        res.values().sum::<i32>().to_string()
    }
}
