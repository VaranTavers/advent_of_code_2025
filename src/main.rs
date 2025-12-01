use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::days::day1;
extern crate helper_lib;
mod days;

fn main() -> Result<(), io::Error> {
    let day = 1;
    let part = 2;

    let f = File::open(format!("inputs/input{day}.txt"))?;
    let reader = BufReader::new(f);

    print!("Day {day} (Part: {part}): ");
    match part {
        1 => {
            match day {
                1 => println!("{}", day1::solution(reader).unwrap()),
                /*2 => println!("{}", day2::solution(reader).unwrap()),
                3 => println!("{}", day3::solution(reader).unwrap()),
                4 => println!("{}", day4::solution(reader).unwrap()),
                5 => println!("{}", day5::solution(reader).unwrap()),
                6 => println!("{}", day6::solution(reader).unwrap()),
                7 => println!("{}", day7::solution(reader).unwrap()),
                8 => println!("{}", day8::solution(reader).unwrap()),
                9 => println!("{}", day9::solution(reader).unwrap()),
                10 => println!("{}", day10::solution(reader).unwrap()),
                11 => println!("{}", day11::solution(reader).unwrap()),
                12 => println!("{}", day12::solution(reader).unwrap()),*/
                _ => println!("What?"),
            };
        }
        _ => {
            match day {
                1 => println!("{}", day1::solution2(reader).unwrap()),
                /*2 => println!("{}", day2::solution2(reader).unwrap()),
                3 => println!("{}", day3::solution2(reader).unwrap()),
                4 => println!("{}", day4::solution2(reader).unwrap()),
                5 => println!("{}", day5::solution2(reader).unwrap()),
                6 => println!("{}", day6::solution2(reader).unwrap()),
                7 => println!("{}", day7::solution2(reader).unwrap()),
                8 => println!("{}", day8::solution2(reader).unwrap()),
                9 => println!("{}", day9::solution2(reader).unwrap()),
                10 => println!("{}", day10::solution2(reader).unwrap()),
                11 => println!("{}", day11::solution2(reader).unwrap()),*/
                _ => println!("What?"),
            };
        }
    };

    Ok(())
}
