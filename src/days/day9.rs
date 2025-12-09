use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

#[derive(Debug)]
pub struct Point2D {
    x: i64,
    y: i64,
}

impl Point2D {
    fn area(&self, other: &Self) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }

    fn from_str(line: &str, delim: char) -> Result<Self, ParseIntError> {
        let parts = line.split(delim).collect::<Vec<&str>>();
        Ok(Point2D {
            x: parts[0].parse::<i64>()?,
            y: parts[1].parse::<i64>()?,
        })
    }
}

pub fn read_lines(reader: BufReader<File>) -> Vec<Point2D> {
    reader
        .lines()
        .map_while(Result::ok)
        .flat_map(|x| Point2D::from_str(&x, ','))
        .collect()
}

pub fn solution(reader: BufReader<File>) -> i64 {
    let tiles = read_lines(reader);

    let mut max = 0;

    for (i, tile1) in tiles.iter().enumerate() {
        for tile2 in tiles.iter().skip(i + 1) {
            let area = tile1.area(tile2);
            if area > max {
                max = area;
            }
        }
    }

    max
}

pub fn intersects(
    tile1: &Point2D,
    tile2: &Point2D,
    p1: &Point2D,
    p2: &Point2D,
    out_x: i64,
    out_y: i64,
) -> bool {
    // If "out" intersects with rectangle
    if tile2.x == tile1.x {
        // If vertical
        if !((tile1.y >= p1.y.max(p2.y) && tile2.y >= p1.y.max(p2.y)) // Not (above
                || (tile1.y <= p1.y.min(p2.y) && tile2.y <= p1.y.min(p2.y))) // or below)
                && tile1.x + out_x >= p1.x.min(p2.x)
                && tile1.x + out_x <= p1.x.max(p2.x)
        {
            // println!("\tVERTICAL in FOR {tile1:?} {tile2:?} {out_x} {out_y}");
            return true;
        }
    } else if tile2.y == tile1.y {
        // If horizontal
        if !((tile1.x >= p1.x.max(p2.x) && tile2.x >= p1.x.max(p2.x)) // Not (to right
                || (tile1.x <= p1.x.min(p2.x) && tile2.x <= p1.x.min(p2.x))) // or to left)
                && tile1.y + out_y >= p1.y.min(p2.y)
                && tile1.y + out_y <= p1.y.max(p2.y)
        {
            // println!("\tHORIZ in FOR {tile1:?} {tile2:?}  {out_x} {out_y}");
            return true;
        }
    }

    false
}

pub fn turn(tile3: &Point2D, tile2: &Point2D, mut out_x: i64, mut out_y: i64) -> (i64, i64) {
    if out_x != 0 && tile3.x != tile2.x {
        // We are going up or down And we turn
        if tile3.x < tile2.x {
            out_y = 1;
        } else {
            out_y = -1;
        }

        out_x = 0;
    } else if out_y != 0 && tile3.y != tile2.y {
        if tile3.y > tile2.y {
            out_x = 1;
        } else {
            out_x = -1;
        }

        out_y = 0;
    }

    (out_x, out_y)
}

pub fn valid(tiles: &[Point2D], i: usize, j: usize) -> bool {
    // Which direction is out?
    let mut out_x = 1; // Base case: up
    let mut out_y = 0;

    if tiles[1].x > tiles[0].x {
        out_y = -1;
        out_x = 0;
    } else if tiles[1].x < tiles[0].x {
        out_y = 1;
        out_x = 0;
    } else if tiles[1].y < tiles[0].y {
        out_y = 0;
        out_x = -1;
    }

    let p1 = &tiles[i];
    let p2 = &tiles[j];
    // println!("VERIF {p1:?}, {p2:?}");

    for ((tile1, tile2), tile3) in tiles
        .iter()
        .zip(tiles.iter().skip(1))
        .zip(tiles.iter().skip(2))
    {
        // println!("\tVERIF in FOR {tile1:?} {tile2:?} {out_x} {out_y}");

        if intersects(tile1, tile2, p1, p2, out_x, out_y) {
            return false;
        }

        (out_x, out_y) = turn(tile3, tile2, out_x, out_y);
    }

    let tile1 = &tiles[tiles.len() - 2];
    let tile2 = &tiles[tiles.len() - 1];
    let tile3 = &tiles[0];

    if intersects(tile1, tile2, p1, p2, out_x, out_y) {
        return false;
    }

    (out_x, out_y) = turn(tile3, tile2, out_x, out_y);

    let tile1 = &tiles[tiles.len() - 1];
    let tile2 = &tiles[0];

    if intersects(tile1, tile2, p1, p2, out_x, out_y) {
        return false;
    }

    true
}

pub fn solution2(reader: BufReader<File>) -> i64 {
    let tiles = read_lines(reader);

    let mut max = 0;

    for (i, tile1) in tiles.iter().enumerate() {
        for (j, tile2) in tiles.iter().enumerate().skip(i + 1) {
            let area = tile1.area(tile2);
            if area > max && valid(&tiles, i, j) {
                // println!("VALID {tile1:?} {tile2:?}");
                max = area;
            }
        }
    }

    max
}
