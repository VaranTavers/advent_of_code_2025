use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
pub struct Problem {
    pub x: usize,
    pub y: usize,
    pub reqs: Vec<usize>,
}

impl FromStr for Problem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(": ").unwrap();
        let size = parts.0.split_once('x').unwrap();

        Ok(Problem {
            x: size.0.parse::<usize>().unwrap(),
            y: size.1.parse::<usize>().unwrap(),
            reqs: parts
                .1
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        })
    }
}

pub fn read_shape(lines: &mut std::io::Lines<BufReader<File>>) -> Vec<Vec<bool>> {
    let _id = lines.next();

    let mut res = Vec::new();

    for _ in 0..3 {
        res.push(
            lines
                .next()
                .unwrap()
                .unwrap()
                .chars()
                .map(|x| x == '#')
                .collect(),
        );
    }

    let _space = lines.next();

    res
}

pub fn read_shapes(lines: &mut std::io::Lines<BufReader<File>>) -> Vec<Vec<Vec<bool>>> {
    let mut res = Vec::new();

    for _ in 0..=5 {
        res.push(read_shape(lines));
    }

    res
}

pub fn solution(reader: BufReader<File>) -> usize {
    let mut lines: std::io::Lines<BufReader<File>> = reader.lines();

    let shapes = read_shapes(&mut lines);
    let mut problems = Vec::new();

    for line in lines.map_while(Result::ok) {
        problems.push(line.parse::<Problem>().unwrap());
    }

    let mut i = 0;
    for prob in &problems {
        println!(
            "{prob:?} {} {}",
            prob.x * prob.y,
            prob.reqs.iter().sum::<usize>()
        );
        if prob.x * prob.y >= 7 * prob.reqs.iter().sum::<usize>() {
            i += 1;
        }
    }

    i
}

pub fn solution2(reader: BufReader<File>) -> usize {
    0
}
