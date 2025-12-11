use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

use z3::{
    ast::{self, Int},
    Config, Context, Optimize,
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

// Probably a bad idea, but bringing z3 along for the ride

fn do_magic_on_problem(problem: &Problem) -> usize {
    let cfg = Config::new();
    let opt = Optimize::new();

    let vars = problem
        .buttons
        .iter()
        .enumerate()
        .map(|(i, _)| ast::Int::new_const(format!("b{i}")))
        .collect::<Vec<Int>>();

    let mut big_sum = ast::Int::from_u64(0);
    for v in &vars {
        opt.assert(&v.ge(ast::Int::from_u64(0)));
        big_sum += v;
    }

    for (i, jolt) in problem.jolts.iter().enumerate() {
        let res = ast::Int::from_u64(*jolt);
        let mut buttons = ast::Int::from_u64(0);
        for (j, b) in problem.buttons.iter().enumerate() {
            if b.contains(&i) {
                buttons += &vars[j];
            }
        }
        opt.assert(&res.eq(buttons));
    }

    opt.minimize(&big_sum);

    match opt.check(&[]) {
        z3::SatResult::Unsat => {
            println!("Unsat");
        }
        z3::SatResult::Unknown => {
            println!("Unkown");
        }
        z3::SatResult::Sat => {
            let model = opt.get_model().unwrap();

            return model.eval(&big_sum, true).unwrap().as_u64().unwrap() as usize;
        }
    }

    0
}

pub fn solution2(reader: BufReader<File>) -> usize {
    let problems = read_input(reader);

    problems
        .iter()
        .map(|x| {
            //println!("{x:?}");
            let res = do_magic_on_problem(x);
            //println!("{res:?}");

            res
        })
        .sum()
}
