use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// TODO: version 2 that is more efficient?

pub fn is_silly_pattern(mut n: i64) -> bool {
    let mut uj = 0;
    let mut p = 1;
    while n > uj {
        uj += p * (n % 10);
        p *= 10;
        n /= 10;
    }
    return n == uj && p / 10 <= uj;
}

pub fn process_input(reader: BufReader<File>) -> Vec<(i64, i64)> {
    let line = reader
        .lines()
        .next()
        .expect("No line found")
        .expect("IO error");

    line.split(',')
        .map(|x| {
            let parts = x.split_once('-').expect("No - found");
            (
                parts.0.parse::<i64>().expect("NaN start"),
                parts.1.parse::<i64>().expect("NaN end"),
            )
        })
        .collect()
}

pub fn solution(reader: BufReader<File>) -> i64 {
    let values = process_input(reader).iter().fold(0, |acc, (start, end)| {
        //println!("{} {}", start, end);
        let mut s = 0;
        for i in *start..=*end {
            if is_silly_pattern(i) {
                //println!("{} {} {}", start, end, i);
                s += i;
            }
        }
        acc + s
    });

    values
}

// 2 digits -> multiple of 11
// 3 digits -> multiple of 111
// 4 digits -> multiple of 1111 or 101
// 5 digits -> multiple of 11111
// 6 digits -> multiple of 111111 or 10101 or 1001
// 7 digits -> multiple of 1111111
// 8 digits -> multiple of 11111111 or 1010101 or 10001

pub fn num_of_digits(mut n: i64) -> i64 {
    let mut db = 1;
    while n > 9 {
        db += 1;
        n /= 10;
    }

    db
}

pub fn create_divisor(mut n: i64, i: i64) -> i64 {
    let mut uj = 0;
    while n > 0 {
        let pp = (10 as i64).pow((i - 1) as u32);
        uj = uj * 10 * pp + pp;
        n -= i;
    }
    // Stopgap solution
    while uj % 10 == 0 {
        uj /= 10;
    }

    uj
}

pub fn generate_divisors(db: i64) -> Vec<i64> {
    let mut res = Vec::new();

    for i in 1..=(db / 2) {
        if db % i == 0 {
            res.push(create_divisor(db, i));
        }
    }

    res
}

pub fn is_divisible_by_any(n: i64, divs: &[i64]) -> bool {
    divs.iter().any(|x| n % x == 0)
}

pub fn solution2(reader: BufReader<File>) -> i64 {
    let values = process_input(reader);
    let max_num = values.iter().map(|x| x.1).max().expect("No max???");
    let mut divs = Vec::new();
    divs.push(Vec::new());
    for i in 1..=num_of_digits(max_num) {
        divs.push(generate_divisors(i));
    }
    println!("{:?}", divs);

    let res = values.iter().fold(0, |acc, (start, end)| {
        //println!("{} {}", start, end);
        let mut s = 0;
        for i in *start..=*end {
            let db = num_of_digits(i) as usize;
            if is_divisible_by_any(i, &divs[db]) {
                println!("{} {} {}", start, end, i);
                s += i;
            }
        }
        acc + s
    });

    res
}
