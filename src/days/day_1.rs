use super::DaySolver;

pub struct Solver;

impl Solver {
    fn get_sorted_elfs(&self, input_txt: &str) -> Vec<u32> {
        let mut elfs = Vec::new();
        let mut curr_val = 0;
        for line in input_txt.lines() {
            if line.is_empty() {
                elfs.push(curr_val);
                curr_val = 0
            } else {
                curr_val += line.parse::<u32>().unwrap();
            }
        }
        elfs.sort();
        elfs.reverse();
        elfs
    }
}

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let elfs = self.get_sorted_elfs(input_txt);
        let mut res = 0;
        for i in 0..3 {
            res += elfs[i];
        }
        return res.to_string();
    }

    fn q1(&self, input_txt: &str) -> String {
        let elfs = self.get_sorted_elfs(input_txt);
        return elfs[0].to_string();
    }
}
