use std::collections::{VecDeque, HashSet};

use super::DaySolver;

pub struct Solver;

fn can_go(matrix: &Vec<Vec<char>>, seen: &HashSet<(usize, usize)>, from: (usize, usize), to: (i32, i32)) -> bool {
    let (i, j) = from;
    let (new_i, new_j) = to;
    let mut c = matrix[i][j];
    if c == 'S' {
        c = 'a'
    }
    if new_i < 0 || new_i as usize >= matrix.len() || new_j < 0 || new_j as usize >= matrix[new_i as usize].len() {
        return false;
    }

    let (new_i, new_j) = (new_i as usize, new_j as usize);

    if seen.contains(&(new_i, new_j)) {
        return false;
    }

    let next_c = matrix[new_i][new_j];
    if next_c == 'E' {
        return true
    }

    next_c as i32 - c as i32 <= 1
}

fn run_bfs(matrix: &Vec<Vec<char>>, start: (usize, usize)) -> i32 {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((start, 0));

    let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while !queue.is_empty() {
        let ((i, j), dist) = queue.pop_front().unwrap();
        if seen.contains(&(i, j)) {
            continue
        }
        if matrix[i][j] == 'E' {
            return dist
        }
        seen.insert((i, j));
        for (dir_i, dir_j) in dirs {
            let new_i = i as i32 + dir_i;
            let new_j = j as i32 + dir_j;
            if can_go(&matrix, &seen, (i, j), (new_i, new_j)) {
                queue.push_back(((new_i as usize, new_j as usize), dist + 1));
            }
        }
    }
    return i32::MAX
}

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let mut matrix = Vec::new();
        for line in input_txt.lines() {
            matrix.push(line.chars().collect::<Vec<char>>())
        }
        let mut min = i32::MAX;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 'S' || matrix[i][j] == 'a' {
                    // We could do better by memoizing!.. but this got me an answer in
                    // a few seconds soooo
                    min = i32::min(min, run_bfs(&matrix, (i, j)));
                }
            }
        }
        return min.to_string();
    }

    fn q1(&self, input_txt: &str) -> String {
        let mut matrix = Vec::new();
        for line in input_txt.lines() {
            matrix.push(line.chars().collect::<Vec<char>>())
        }
        let mut start = (0, 0);
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 'S' {
                    start = (i, j);
                }
            }
        }
        run_bfs(&matrix, start).to_string()
    }
}
