use std::{
    collections::{HashMap, HashSet},
    vec,
};

use super::DaySolver;

pub struct Solver;

fn make_grid(input_txt: &str) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    for line in input_txt.lines() {
        res.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        )
    }
    res
}

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let grid: Vec<Vec<usize>> = make_grid(input_txt);
        let m = grid[0].len();
        let n = grid.len();
        let mut results = HashMap::new();
        for i in 0..n {
            for j in 0..m {
                let item = grid[i][j] as i32;
                let mut res = 0;
                for k in (0..j).rev() {
                    res += 1;
                    let other = grid[i][k] as i32;
                    if other >= item {
                        break;
                    }
                }
                results.entry((i, j)).or_insert(vec![1]).push(res);
            }
        }
        for i in 0..n {
            for j in (0..m).rev() {
                let item = grid[i][j] as i32;
                let mut res = 0;
                for k in j + 1..m {
                    res += 1;
                    let other = grid[i][k] as i32;
                    if other >= item {
                        break;
                    }
                }
                results.entry((i, j)).or_insert(vec![1]).push(res);
            }
        }

        for j in 0..m {
            for i in 0..n {
                let item = grid[i][j] as i32;
                let mut res = 0;
                for k in (0..i).rev() {
                    res += 1;
                    let other = grid[k][j] as i32;
                    if other >= item {
                        break;
                    }
                }
                results.entry((i, j)).or_insert(vec![1]).push(res);
            }
        }
        for j in 0..m {
            for i in (0..n).rev() {
                let item = grid[i][j] as i32;
                let mut res = 0;
                for k in i + 1..n {
                    res += 1;
                    let other = grid[k][j] as i32;
                    if other >= item {
                        break;
                    }
                }
                results.entry((i, j)).or_insert(vec![1]).push(res);
            }
        }
        results
            .iter()
            .filter(|(key, _)| key.0 != 0 && key.0 != n - 1 && key.1 != 0 && key.1 != m - 1)
            .map(|(_, scores)| scores.iter().fold(1, |acc, v| acc * v))
            .max()
            .unwrap()
            .to_string()
    }

    fn q1(&self, input_txt: &str) -> String {
        let grid: Vec<Vec<usize>> = make_grid(input_txt);
        let m = grid[0].len();
        let n = grid.len();
        let mut seen = HashSet::new();
        for i in 0..n {
            let mut curr_size = -1;
            for j in 0..m {
                let item = grid[i][j] as i32;
                if item > curr_size {
                    curr_size = item;
                    seen.insert((i, j));
                }
            }
        }
        for i in 0..n {
            let mut curr_size = -1;
            for j in (0..m).rev() {
                let item = grid[i][j];
                if item as i32 > curr_size {
                    curr_size = item as i32;
                    seen.insert((i, j));
                }
            }
        }

        for j in 0..m {
            let mut curr_size = -1;
            for i in 0..n {
                let item = grid[i][j];
                if item as i32 > curr_size {
                    curr_size = item as i32;
                    seen.insert((i, j));
                }
            }
        }
        for j in 0..m {
            let mut curr_size = -1;
            for i in (0..n).rev() {
                let item = grid[i][j];
                if item as i32 > curr_size {
                    curr_size = item as i32;
                    seen.insert((i, j));
                }
            }
        }
        seen.len().to_string()
    }
}
