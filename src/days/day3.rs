use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn process_input(reader: BufReader<File>) -> Vec<Vec<i8>> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().map(|c| c as i8 - '0' as i8).collect())
        .collect()
}

pub fn find_top_two(line: &[i8]) -> i8 {
    let mut prev_max = -1;
    let mut max = -1;
    let mut sec_max = -1;
    for digit in line {
        if *digit > max {
            prev_max = max;
            sec_max = -1;
            max = *digit;
        } else if *digit > sec_max {
            sec_max = *digit;
        }
    }
    if sec_max == -1 {
        return prev_max * 10 + max;
    }

    max * 10 + sec_max
}

pub fn solution(reader: BufReader<File>) -> i64 {
    process_input(reader)
        .iter()
        .fold(0, |acc, line| acc + i64::from(find_top_two(line)))
}

/*
// Use only for testing!
pub fn recursive_version(line: &[i8], k: usize, i: usize, n: i64) -> i64 {
    if k == 0 {
        return n;
    }
    if i == line.len() {
        return -1;
    }

    let a = recursive_version(line, k - 1, i + 1, n * 10 + i64::from(line[i]));
    let b = recursive_version(line, k, i + 1, n);

    a.max(b)
}
 */

pub fn calc_not_worse_than_x_and_next(line: &[i8]) -> (Vec<i8>, Vec<Option<usize>>) {
    let mut a = line.iter().map(|_x| 0).collect::<Vec<i8>>();
    let mut b = line.iter().map(|_x| None).collect::<Vec<Option<usize>>>();

    for (i, val) in line.iter().enumerate() {
        a[i] = line.iter().skip(i + 1).filter(|x| **x >= *val).count() as i8;
        b[i] = line
            .iter()
            .skip(i + 1)
            .position(|x| *x > *val)
            .map(|x| x + i + 1);
    }
    (a, b)
}

pub fn calc_x_battery(line: &[i8], mut unused_digits: i8) -> i64 {
    let mut res = 0;
    let mut i = 0;

    let (bigger, next) = calc_not_worse_than_x_and_next(line);

    while unused_digits > 0 && i < line.len() {
        if unused_digits > bigger[i] // If there are enough fewer larger digits than what is required
            && (next[i].is_none() || line.len() - next[i].unwrap() < unused_digits as usize)
        // And after the next bigger digit there aren't enough other digits
        {
            unused_digits -= 1;
            res = res * 10 + i64::from(line[i]);
        }

        i += 1;
    }

    res
}

pub fn solution2(reader: BufReader<File>) -> i64 {
    process_input(reader)
        .iter()
        .fold(0, |acc, line| acc + calc_x_battery(line, 12))
}
