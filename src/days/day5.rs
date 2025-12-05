use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_values(reader: BufReader<File>) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut values = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        if !line.is_empty() {
            if line.contains("-") {
                let parts = line.split_once('-').expect("Spanish inquisition");
                let num1 = parts.0.parse::<u64>().expect("Number 1 can't be parsed");
                let num2 = parts.1.parse::<u64>().expect("Number 2 can't be parsed");

                ranges.push((num1, num2));
            } else {
                let num = line
                    .parse::<u64>()
                    .expect("Singular number can't be interpreted");
                values.push(num);
            }
        }
    }

    (ranges, values)
}

pub fn solution(reader: BufReader<File>) -> usize {
    let (ranges, values) = read_values(reader);

    values
        .iter()
        .filter(|x| ranges.iter().any(|(a, b)| *x >= a && *x <= b))
        .count()
}

pub fn solution2(reader: BufReader<File>) -> u64 {
    let (mut ranges_old, _) = read_values(reader);

    ranges_old.sort();

    let mut ranges_new = Vec::new();

    let (mut start, mut end) = ranges_old[0];

    for (a, b) in ranges_old {
        if a > end {
            ranges_new.push((start, end));
            start = a;
            end = b;
        } else if b > end {
            end = b;
        }
    }

    ranges_new.push((start, end));

    ranges_new.iter().map(|(a, b)| b - a + 1).sum()
}
