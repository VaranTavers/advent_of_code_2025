use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_values(reader: BufReader<File>) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut columns = Vec::new();
    let mut operations = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        if line.contains('*') {
            operations = line
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.chars().next().expect("Spanish inquisition"))
                .collect();
        } else {
            for (i, val) in line.split(' ').filter(|x| !x.is_empty()).enumerate() {
                if columns.len() == i {
                    columns.push(Vec::new());
                }
                columns[i].push(val.parse::<u64>().expect("NaN"));
            }
        }
    }

    (columns, operations)
}

pub fn solution(reader: BufReader<File>) -> u64 {
    let (columns, operators) = read_values(reader);

    columns
        .iter()
        .zip(operators)
        .map(|(vals, op)| {
            if op == '+' {
                return vals.iter().sum::<u64>();
            }
            vals.iter().product()
        })
        .sum()
}

pub fn read_values2(reader: BufReader<File>) -> (Vec<Vec<char>>, Vec<char>) {
    let mut columns = Vec::new();
    let mut operations = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        if line.contains('*') {
            operations = line.chars().to_owned().collect();
        } else {
            for (i, val) in line.chars().enumerate() {
                if columns.len() == i {
                    columns.push(Vec::new());
                }
                columns[i].push(val);
            }
        }
    }

    (columns, operations)
}

pub fn create_number(col: &[char]) -> u64 {
    col.iter()
        .filter(|x| **x != ' ')
        .map(|x| *x as u64 - '0' as u64)
        .fold(0, |acc, val| acc * 10 + val)
}

pub fn solution2(reader: BufReader<File>) -> u64 {
    let (columns, operators) = read_values2(reader);
    let mut res = 0;
    let mut last_op = '\0';
    let mut partial_res = 0;

    for (col, c) in operators.iter().enumerate() {
        if *c != ' ' {
            res += partial_res;
            if *c == '*' {
                partial_res = 1;
            } else {
                partial_res = 0;
            }
            last_op = *c;
        }
        let val = create_number(&columns[col]);
        if val != 0 && last_op == '*' {
            partial_res *= val;
        } else if last_op == '+' {
            partial_res += val
        }
    }

    res += partial_res;

    res
}
