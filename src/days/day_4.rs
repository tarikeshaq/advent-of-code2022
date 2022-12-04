use std::str::FromStr;

use super::DaySolver;

pub struct Solver;

struct ClampInterval {
    start: u32,
    end: u32,
}

impl FromStr for ClampInterval {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let start = split.next().unwrap().parse::<u32>().unwrap();
        let end = split.next().unwrap().parse::<u32>().unwrap();
        Ok(Self { start, end })
    }
}

impl ClampInterval {
    fn contains(&self, other: &ClampInterval) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &ClampInterval) -> bool {
        // we are in between the other's start and end
        (self.start <= other.end && self.end >= other.end) ||
        // the other is in between our start and end
        (other.start <= self.end && other.end >= self. end)
    }
}

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        input_txt
            .lines()
            .map(|line| {
                let mut split = line.split(',');
                let first_interval = ClampInterval::from_str(split.next().unwrap()).unwrap();
                let second_interval = ClampInterval::from_str(split.next().unwrap()).unwrap();
                (first_interval, second_interval)
            })
            .filter(|(first, second)| first.overlaps(second))
            .count()
            .to_string()
    }

    fn q1(&self, input_txt: &str) -> String {
        input_txt
            .lines()
            .map(|line| {
                let mut split = line.split(',');
                let first_interval = ClampInterval::from_str(split.next().unwrap()).unwrap();
                let second_interval = ClampInterval::from_str(split.next().unwrap()).unwrap();
                (first_interval, second_interval)
            })
            .filter(|(first, second)| first.contains(second) || second.contains(first))
            .count()
            .to_string()
    }
}
