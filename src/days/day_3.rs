use std::collections::HashSet;

use super::DaySolver;

pub struct Solver;

impl Solver {
    fn get_common(&self, strings: &[&str]) -> HashSet<char> {
        let mut res: Option<HashSet<char>> = None;
        for s in strings {
            let set = s.chars().collect::<HashSet<char>>();
            if let Some(inner) = res {
                res = Some(inner.intersection(&set).copied().collect())
            } else {
                res = Some(set)
            }
        }
        return res.unwrap();
    }

    fn get_priority(&self, character: &char) -> u32 {
        match character {
            c if *c >= 'a' && *c <= 'z' => *c as u32 - 'a' as u32 + 1,
            c if *c >= 'A' && *c <= 'Z' => *c as u32 - 'A' as u32 + 27,
            _ => panic!("Invalid character"),
        }
    }
}

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let mut lines = Vec::with_capacity(3);
        let mut res = 0;
        for line in input_txt.lines() {
            lines.push(line);
            if lines.len() == 3 {
                let first = lines[0];
                let second = lines[1];
                let third = lines[2];
                let common_letters = self.get_common(&[first, second, third]);
                // we have a guarantee that we only have one type misplaced per bucket
                assert_eq!(common_letters.len(), 1);
                let letter = common_letters.iter().next().unwrap();
                let priority = self.get_priority(letter);
                res += priority;
                lines.clear()
            }
        }
        res.to_string()
    }

    fn q1(&self, input_txt: &str) -> String {
        let mut res = 0;
        for line in input_txt.lines() {
            let (first, second) = line.split_at(line.len() / 2);
            let common_letters = self.get_common(&[first, second]);
            // we have a guarantee that we only have one type misplaced per bucket
            assert_eq!(common_letters.len(), 1);
            let letter = common_letters.iter().next().unwrap();
            let priority = self.get_priority(letter);
            res += priority;
        }

        res.to_string()
    }
}
