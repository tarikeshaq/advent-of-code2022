use core::panic;
use std::collections::HashMap;

use super::DaySolver;

pub struct Solver;

impl Solver {
    fn find_repeating(window: usize, input_txt: &str) -> usize {
        let mut i = 0;
        let mut j = 0;
        let mut seen: HashMap<char, usize> = HashMap::new();
        let input_txt = input_txt.as_bytes();
        while j < input_txt.len() {
            let c = input_txt[j] as char;
            *seen.entry(c).or_default() += 1;
            if j - i == window - 1 {
                if seen.len() == window {
                    return j + 1;
                } else {
                    let other_c = input_txt[i] as char;
                    let entry = seen.entry(other_c).or_default();
                    *entry -= 1;
                    if *entry == 0 {
                        seen.remove(&other_c);
                    }
                    i += 1;
                }
            }
            j += 1;
        }
        panic!("Did not find a repeating {window} size window")
    }
}

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        Self::find_repeating(14, input_txt).to_string()
    }

    fn q1(&self, input_txt: &str) -> String {
        Self::find_repeating(4, input_txt).to_string()
    }
}
