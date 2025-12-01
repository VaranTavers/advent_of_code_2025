use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn process_input(reader: BufReader<File>) -> impl Iterator<Item = i64> {
    reader.lines().map_while(Result::ok).map(|x| {
        let vals = x.split_at(1);
        let moves = vals.1.parse::<i64>().expect("Should have been a number");
        if vals.0.chars().collect::<Vec<char>>()[0] == 'L' {
            return -moves;
        }
        moves
    })
}

pub fn solution(reader: BufReader<File>) -> i64 {
    let values = process_input(reader).fold((50, 0), |acc, next_rot| {
        let new_pos = (acc.0 + next_rot % 100 + 100) % 100;
        if new_pos == 0 {
            return (new_pos, acc.1 + 1);
        }
        (new_pos, acc.1)
    });

    values.1
}

pub fn solution2(reader: BufReader<File>) -> i64 {
    let values = process_input(reader).fold((50, 0), |acc, next_rot| {
        let new_pos = (acc.0 + next_rot % 100 + 100) % 100;
        //println!("{} -> {}", acc.0, new_pos);
        let d = acc.0 + next_rot;
        if d <= 0 && acc.0 != 0 {
            return (new_pos, acc.1 + d.abs() / 100 + 1);
        }
        (new_pos, acc.1 + d.abs() / 100)
    });

    values.1
}
