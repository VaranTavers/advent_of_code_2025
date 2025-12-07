use std::{fs::File, io::BufReader};

use helper_lib::utils::{CharMap, Direction};

pub fn solution(reader: BufReader<File>) -> i64 {
    let cmap = CharMap::parse_map(reader);
    let mut res = 0;

    for (i, j, c) in &cmap {
        if c == '@' {
            let mut neighbors = 0;
            for dir in Direction::all_directions() {
                if let Some(new_coord) = dir.move_to((i, j)) {
                    if cmap.get(new_coord) == Some('@') {
                        neighbors += 1;
                    }
                }
            }
            if neighbors < 4 {
                res += 1;
            }
        }
    }

    res
}

pub fn solution2(reader: BufReader<File>) -> i64 {
    let mut cmap = CharMap::parse_map(reader);
    let mut res = 0;
    let mut is_new = true;

    let mut places = Vec::new();

    while is_new {
        is_new = false;
        while let Some(pos) = places.pop() {
            cmap.set(pos, 'x');
        }
        for (i, j, c) in &cmap {
            if c == '@' {
                let mut neighbors = 0;
                for dir in Direction::all_directions() {
                    if let Some(new_coord) = dir.move_to((i, j)) {
                        if cmap.get(new_coord) == Some('@') {
                            neighbors += 1;
                        }
                    }
                }
                if neighbors < 4 {
                    places.push((i, j));
                    is_new = true;
                    res += 1;
                }
            }
        }
    }

    res
}
