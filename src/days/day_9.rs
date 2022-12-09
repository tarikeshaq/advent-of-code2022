use std::collections::HashSet;

use super::DaySolver;


enum Move {
    Up,
    Left,
    Right,
    Down
}

#[derive(Default)]
struct State {
    head: (i32, i32),
    tail: (i32, i32)
}

#[derive(Default, Debug)]
struct StateV2 {
    states: [(i32, i32); 10]
}


impl Move {
    fn new(s: &str) -> Self {
        match s {
            "R" => Move::Right,
            "L" => Move::Left,
            "U" => Move::Up,
            "D" => Move::Down,
            _ => panic!("OOF")
        }
    }
}

fn process_move2(states: &mut StateV2,  m: Move, amount: i32, res: &mut HashSet<(i32, i32)>) {
    // first we blindly move the head state
    for _ in 0..amount {
        match m {
            Move::Right => states.states[0].0 += 1,
            Move::Left => states.states[0].0 -= 1,
            Move::Down => states.states[0].1 += 1,
            Move::Up => states.states[0].1 -= 1
        }

        let mut prev = states.states[0];
        for i in 1..10 {
            // now lets reconcile the tail's position
            // If the distance is one away, we do not move, and just return early

            // if we are here, we are more than one step away!
            // lets find out what direction we should go
            // the following is if we should go up
            match (states.states[i].0 - prev.0, states.states[i].1 - prev.1) {
                (2, 0) => states.states[i].0 -=1,
                (-2, 0) => states.states[i].0 +=1,
                (0, 2) => states.states[i].1 -=1,
                (0, -2) => states.states[i].1 +=1,
                (2, 1) | (1, 2) | (2, 2) => {
                    states.states[i].0 -= 1;
                    states.states[i].1 -= 1;
                },
                (-2, -1) | (-1, -2) | (-2, -2) => {
                    states.states[i].0 += 1;
                    states.states[i].1 += 1;
                }
                (-2, 1) | (-1, 2) | (-2, 2) => {
                    states.states[i].0 += 1;
                    states.states[i].1 -= 1;
                },
                (1, -2) | (2, -1) | (2, -2) => {
                    states.states[i].0 -= 1;
                    states.states[i].1 += 1;
                },
                _ => ()
            }
            prev = states.states[i];
        }
        res.insert((prev.0, prev.1));
    }

}

fn process_move(state: &mut State,  m: Move, amount: i32, res: &mut HashSet<(i32, i32)>) {
    // first we blindly move the head state
    for _ in 0..amount {
        match m {
            Move::Right => state.head.0 += 1,
            Move::Left => state.head.0 -= 1,
            Move::Down => state.head.1 += 1,
            Move::Up => state.head.1 -= 1
        }

        // now lets reconcile the tail's position
        // If the distance is one away, we do not move, and just return early

        // if we are here, we are more than one step away!
        // lets find out what direction we should go
        // the following is if we should go up
         match (state.tail.0 - state.head.0, state.tail.1 - state.head.1) {
            (2, 0) => state.tail.0 -=1,
            (-2, 0) => state.tail.0 +=1,
            (0, 2) => state.tail.1 -=1,
            (0, -2) => state.tail.1 +=1,
            (2, 1) | (1, 2) => {
                state.tail.0 -= 1;
                state.tail.1 -= 1;
            },
            (-2, -1) | (-1, -2) => {
                state.tail.0 += 1;
                state.tail.1 += 1;
            }
            (-2, 1) | (-1, 2) => {
                state.tail.0 += 1;
                state.tail.1 -= 1;
            },
            (1, -2) | (2, -1) => {
                state.tail.0 -= 1;
                state.tail.1 += 1;
            }
            (_, _) => ()
         }
        res.insert((state.tail.0, state.tail.1));
    }

}
pub struct Solver;

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let mut res = HashSet::new();
        res.insert((0, 0));
        let mut state = StateV2::default();
        for line in input_txt.lines() {
            let mut split = line.split_whitespace();
            let m = Move::new(split.next().unwrap());
            let amount = split.next().unwrap().parse::<i32>().unwrap();
            process_move2(&mut state,  m, amount, &mut res);
        }
        res.len().to_string()
    }

    fn q1(&self, input_txt: &str) -> String {
        let mut res = HashSet::new();
        res.insert((0, 0));
        let mut state = State::default();
        for line in input_txt.lines() {
            let mut split = line.split_whitespace();
            let m = Move::new(split.next().unwrap());
            let amount = split.next().unwrap().parse::<i32>().unwrap();
            process_move(&mut state,  m, amount, &mut res);
        }
        res.len().to_string()
    }
}
