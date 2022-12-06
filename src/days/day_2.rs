use std::{fmt::Display, str::FromStr};

use super::DaySolver;
trait ValueHolder {
    fn get_value(&self) -> u32;
}

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Won,
    Lost,
    Tie,
}

impl From<(&Hand, Hand)> for GameResult {
    fn from(value: (&Hand, Hand)) -> Self {
        match value {
            (Hand::Rock, Hand::Paper)
            | (Hand::Paper, Hand::Scissors)
            | (Hand::Scissors, Hand::Rock) => Self::Lost,
            (Hand::Rock, Hand::Scissors)
            | (Hand::Paper, Hand::Rock)
            | (Hand::Scissors, Hand::Paper) => Self::Won,
            _ => Self::Tie,
        }
    }
}

impl ValueHolder for GameResult {
    fn get_value(&self) -> u32 {
        match self {
            Self::Won => 6,
            Self::Tie => 3,
            Self::Lost => 0,
        }
    }
}

impl FromStr for GameResult {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => GameResult::Lost,
            "Y" => GameResult::Tie,
            "Z" => GameResult::Won,
            _ => panic!("Invalid result"),
        })
    }
}

impl FromStr for Hand {
    type Err = std::str::Utf8Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Unknown hand played"),
        })
    }
}

impl Hand {
    fn get_result(&self, other: Hand) -> u32 {
        let hand_value = self.get_value();
        let game_result: GameResult = (self, other).into();
        hand_value + game_result.get_value()
    }
}

impl ValueHolder for Hand {
    fn get_value(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Rock => "Rock",
            Self::Paper => "Paper",
            Self::Scissors => "Scissors",
        })
    }
}

fn get_my_hand(thier: &Hand, expected_result: GameResult) -> Hand {
    match (thier, expected_result) {
        (Hand::Rock, GameResult::Won)
        | (Hand::Paper, GameResult::Tie)
        | (Hand::Scissors, GameResult::Lost) => Hand::Paper,
        (Hand::Rock, GameResult::Tie)
        | (Hand::Paper, GameResult::Lost)
        | (Hand::Scissors, GameResult::Won) => Hand::Rock,
        (Hand::Rock, GameResult::Lost)
        | (Hand::Paper, GameResult::Won)
        | (Hand::Scissors, GameResult::Tie) => Hand::Scissors,
    }
}

pub struct Solver;

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        input_txt
            .lines()
            .map(|line| {
                let mut split = line.split_whitespace();
                let their_hand = Hand::from_str(split.next().unwrap()).unwrap();
                let game_result = GameResult::from_str(split.next().unwrap()).unwrap();
                let our_hand = get_my_hand(&their_hand, game_result);
                (our_hand, their_hand)
            })
            .fold(0, |acc, (our_hand, their_hand)| {
                acc + our_hand.get_result(their_hand)
            })
            .to_string()
    }

    fn q1(&self, input_txt: &str) -> String {
        input_txt
            .lines()
            .map(|line| {
                let mut split = line.split_whitespace();
                let their_hand = Hand::from_str(split.next().unwrap()).unwrap();
                let our_hand = Hand::from_str(split.next().unwrap()).unwrap();
                (our_hand, their_hand)
            })
            .fold(0, |acc, (our_hand, their_hand)| {
                acc + our_hand.get_result(their_hand)
            })
            .to_string()
    }
}
