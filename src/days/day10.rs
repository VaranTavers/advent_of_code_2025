use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

pub fn push_button(state: &[bool], button: &[usize]) -> Vec<bool> {
    let mut res = state.iter().cloned().collect::<Vec<bool>>();

    for id in button {
        res[*id] = !res[*id];
    }

    res
}

#[derive(Debug)]
pub struct Problem {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    jolts: Vec<u64>,
}

impl Problem {
    pub fn read_line(line: &str) -> Problem {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let target = parts
            .first()
            .expect("Spanish inquisition")
            .chars()
            .filter(|x| *x == '#' || *x == '.')
            .map(|x| x == '#')
            .collect::<Vec<bool>>();

        let jolts = parts
            .last()
            .expect("Spanish inquisition")
            .split(',')
            .map(|x| {
                x.replace('{', "")
                    .replace('}', "")
                    .parse::<u64>()
                    .expect("Spanish inquisition")
            })
            .collect::<Vec<u64>>();

        let buttons = parts
            .iter()
            .skip(1)
            .take(parts.len() - 2)
            .map(|x| {
                x.replace('(', "")
                    .replace(')', "")
                    .split(',')
                    .map(|y| y.parse::<usize>().expect("Spanish inquisition"))
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        Problem {
            target,
            buttons,
            jolts,
        }
    }

    pub fn find_solution(&self) -> Vec<usize> {
        let initial_state = vec![false; self.target.len()];
        let mut queue = self
            .buttons
            .iter()
            .enumerate()
            .map(|(i, x)| (vec![i], push_button(&initial_state, x)))
            .collect::<VecDeque<(Vec<usize>, Vec<bool>)>>();

        while !queue.is_empty() {
            let (pushed, c_state) = queue.pop_front().expect("Spanish inquisition");

            if c_state == self.target {
                return pushed;
            }

            for (i, button) in self.buttons.iter().enumerate() {
                if i != *pushed.last().expect("Spanish inquisition") {
                    let new_state = push_button(&c_state, button);
                    let mut cloned = pushed.clone();
                    cloned.push(i);

                    queue.push_back((cloned, new_state));
                }
            }
        }

        vec![]
    }
}

pub fn read_input(reader: BufReader<File>) -> Vec<Problem> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| Problem::read_line(&line))
        .collect()
}

pub fn solution(reader: BufReader<File>) -> usize {
    let problems = read_input(reader);

    problems
        .iter()
        .map(|x| {
            println!("{x:?}");
            let res = x.find_solution();
            println!("{res:?}");

            res.len()
        })
        .sum()
}

pub fn ok() {}

pub fn naive_backtracking(k: usize, nums: &mut [usize]) {}

pub fn solution2(reader: BufReader<File>) -> i64 {
    0
}
