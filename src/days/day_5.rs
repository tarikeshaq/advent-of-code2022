use std::{
    collections::{BTreeMap, VecDeque},
    iter::Peekable,
};

use super::DaySolver;

enum ParserState {
    Stacks,
    Moves,
}

impl Default for ParserState {
    fn default() -> Self {
        Self::Stacks
    }
}

impl ParserState {
    fn advance(&self) -> Self {
        match self {
            ParserState::Stacks => Self::Moves,
            ParserState::Moves => panic!("Can't advance from the moves state"),
        }
    }
}

struct InputParser<'a> {
    lines: Peekable<std::str::Lines<'a>>,
    state: ParserState,
    idx_in_line: Option<usize>,
}

impl Iterator for InputParser<'_> {
    type Item = ParsedItem;
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.peek()?;
        let mut idx = self.idx_in_line.unwrap_or_default();

        match self.state {
            ParserState::Stacks => {
                // when we are in the stacks,
                // the index will always be a multiple
                // of four, we move in multiples of four
                // until we get the next item, or the line ends.
                let mut res = None;
                while let Some(ch) = line.chars().nth(idx) {
                    if ch == '[' {
                        let stack_char = line.chars().nth(idx + 1).unwrap();
                        res = Some(ParsedItem::StackItem {
                            item: stack_char,
                            stack_num: (idx / 4usize) + 1,
                        });
                        break;
                    }
                    idx += 4
                }
                if let Some(res) = res {
                    self.idx_in_line = Some(idx + 4);
                    Some(res)
                } else {
                    // we reached the end of the line. We should skip this line
                    self.idx_in_line = None;
                    self.lines.next();

                    // let take this opportunity to peek into the next line, and see if
                    // we should advance our state
                    let next_line = self.lines.peek().unwrap();
                    if !next_line.trim().starts_with('[') {
                        self.state = self.state.advance();
                        // lets also skip the next two lines
                        self.lines.next();
                        self.lines.next();
                    }
                    self.next()
                }
            }
            ParserState::Moves => {
                // this is easier, lets just get the two indexes from the line
                let mut split = line.split_whitespace();
                let num = split.nth(1).unwrap().parse::<usize>().unwrap();
                let from = split.nth(1).unwrap().parse::<usize>().unwrap();
                let to = split.nth(1).unwrap().parse::<usize>().unwrap();
                self.lines.next();
                Some(ParsedItem::Move { num, from, to })
            }
        }
    }
}

enum ParsedItem {
    StackItem { item: char, stack_num: usize },
    Move { num: usize, from: usize, to: usize },
}

pub struct Solver;

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let parser = InputParser {
            lines: input_txt.lines().peekable(),
            state: Default::default(),
            idx_in_line: None,
        };

        let mut stacks = BTreeMap::new();

        for item in parser {
            match item {
                ParsedItem::StackItem { item, stack_num } => {
                    let stack: &mut VecDeque<char> = stacks.entry(stack_num).or_default();
                    stack.push_front(item)
                }
                ParsedItem::Move { num, from, to } => {
                    let from_stack: &mut VecDeque<char> = stacks.entry(from).or_default();
                    let mut nums_to_move = Vec::new();
                    for _ in 0..num {
                        let val = from_stack.pop_back().unwrap();
                        nums_to_move.push(val)
                    }
                    let to_stack: &mut VecDeque<char> = stacks.entry(to).or_default();
                    for c in nums_to_move.iter().rev().take(num) {
                        to_stack.push_back(*c)
                    }
                }
            }
        }
        stacks.values().fold(String::new(), |mut acc, e| {
            acc.push(*e.iter().last().unwrap());
            acc
        })
    }

    fn q1(&self, input_txt: &str) -> String {
        let parser = InputParser {
            lines: input_txt.lines().peekable(),
            state: Default::default(),
            idx_in_line: None,
        };

        let mut stacks = BTreeMap::new();

        for item in parser {
            match item {
                ParsedItem::StackItem { item, stack_num } => {
                    let stack: &mut VecDeque<char> = stacks.entry(stack_num).or_default();
                    stack.push_front(item)
                }
                ParsedItem::Move { num, from, to } => {
                    let from_stack: &mut VecDeque<char> = stacks.entry(from).or_default();
                    let mut nums_to_move = Vec::new();
                    for _ in 0..num {
                        let val = from_stack.pop_back().unwrap();
                        nums_to_move.push(val)
                    }
                    let to_stack: &mut VecDeque<char> = stacks.entry(to).or_default();
                    for c in nums_to_move.iter().take(num) {
                        to_stack.push_back(*c)
                    }
                }
            }
        }
        stacks.values().fold(String::new(), |mut acc, e| {
            acc.push(*e.iter().last().unwrap());
            acc
        })
    }
}
