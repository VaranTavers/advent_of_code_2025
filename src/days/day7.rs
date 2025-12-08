use std::{fs::File, io::BufReader};

use helper_lib::utils::{CharMap, Direction};

pub fn solution(reader: BufReader<File>) -> u64 {
    let mut cmap = CharMap::parse_map(reader);

    let mut res = 0;

    let num_rows = cmap.map.len();
    let num_cols = cmap.map[0].len();

    for i in 1..num_rows {
        for j in 0..num_cols {
            let current = cmap.get((i, j)).expect("Spanish inquisition");
            let top = Direction::Top
                .move_to((i, j))
                .and_then(|x| cmap.get(x))
                .expect("Spanish inquisition");
            if top == 'S' || top == '|' {
                if current == '.' {
                    cmap.set((i, j), '|');
                } else if current == '^' {
                    if j > 0 {
                        cmap.set((i, j - 1), '|');
                    }
                    if j < num_cols - 1 {
                        cmap.set((i, j + 1), '|');
                    }
                    res += 1;
                }
            }
        }
    }

    res
}

pub fn solution2(reader: BufReader<File>) -> u64 {
    let mut cmap = CharMap::parse_map(reader);
    let mut num_map = cmap.map_to_val(0);

    let (s_row, s_col) = cmap.find_first('S').expect("Spanish inquisition");

    num_map[s_row][s_col] = 1;

    let num_rows = cmap.map.len();
    let num_cols = cmap.map[0].len();

    for i in 1..num_rows {
        for j in 0..num_cols {
            let current = cmap.get((i, j)).expect("Spanish inquisition");
            let top = Direction::Top
                .move_to((i, j))
                .and_then(|x| cmap.get(x))
                .expect("Spanish inquisition");
            if top == 'S' || top == '|' {
                if current == '.' || current == '|' {
                    cmap.set((i, j), '|');
                    num_map[i][j] += num_map[i - 1][j];
                } else if current == '^' {
                    if j > 0 {
                        cmap.set((i, j - 1), '|');
                        num_map[i][j - 1] += num_map[i - 1][j];
                    }
                    if j < num_cols - 1 {
                        cmap.set((i, j + 1), '|');
                        num_map[i][j + 1] += num_map[i - 1][j];
                    }
                }
            }
        }
    }

    num_map[num_rows - 1].iter().sum()
}
