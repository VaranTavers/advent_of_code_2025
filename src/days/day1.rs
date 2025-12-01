use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn process_input(reader: BufReader<File>) -> impl Iterator<Item = i64> {
    reader.lines().flatten().map(|x| {
        let vals = x.split_at(1);
        let moves = vals.1.parse::<i64>().expect("Should have been a number");
        if vals.0.chars().collect::<Vec<char>>()[0] == 'L' {
            return -1 * moves;
        }
        moves
    })
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let values = process_input(reader).fold((50, 0), |acc, next_rot| {
        let new_pos = (acc.0 + next_rot % 100 + 100) % 100;
        if new_pos == 0 {
            return (new_pos, acc.1 + 1);
        }
        return (new_pos, acc.1);
    });

    Ok(values.1)
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let values = process_input(reader).fold((50, 0), |acc, next_rot| {
        let new_pos = (acc.0 + next_rot % 100 + 100) % 100;
        //println!("{} -> {}", acc.0, new_pos);
        let d = acc.0 + next_rot;
        if d <= 0 && acc.0 != 0 {
            return (new_pos, acc.1 + d.abs() / 100 + 1);
        }
        (new_pos, acc.1 + d.abs() / 100)
    });

    Ok(values.1)
}
