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

    fn new(x: i64, y: i64) -> Point2D {
        Point2D { x, y }
    }
}

pub fn read_lines(reader: BufReader<File>) -> Vec<Point2D> {
    reader
        .lines()
        .map_while(Result::ok)
        .map(|x| Point2D::from_str(&x, ','))
        .flatten()
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

pub fn is_line_inside_up(
    tiles: &[Point2D],
    i1: usize,
    p1: &Point2D,
    p2: &Point2D,
) -> (bool, usize) {
    let mut i = i1 + 1;
    let mut p = &tiles[i];
    let mut inside = true;
    let mut next = Point2D::new(p1.x, p1.y + 1);

    while p.y < p2.y {
        if inside {
            if p.x == next.x {
                next = Point2D::new(p.x, p.y + 1);
            } else {
                if p.x > next.x {
                    inside = false;
                }
            }
        } else {
            if p.y > next.y {
                return (false, 0);
            }
            if p.x <= next.x {
                inside = true;
                next = Point2D::new(next.x, p.y + 1);
            }
        }
        i += 1;
        p = &tiles[i];
    }
    // See if it comes back from too far right
    while p.x > p2.x {
        if p.y > p2.y {
            return (false, 0);
        }
        i += 1;
        p = &tiles[i];
    }
    if p.x == p2.x {
        inside = true;
    }
    // See if it doesn't dip back down before it reaches the point
    while p.x < p2.x {
        if inside {
            if p.y < p2.y {
                inside = false;
            }
        } else {
            if p.y > p2.y {
                inside = true
            }
        }
        i += 1;
        p = &tiles[i];
    }

    (inside, i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_line_exact() {
        let tiles = vec![Point2D::new(0, 0), Point2D::new(0, 5), Point2D::new(1, 5)];
        assert!(is_line_inside_up(&tiles, 0, &tiles[0], &Point2D::new(0, 5)).0);
    }
    #[test]
    fn line_line_larger() {
        let tiles = vec![Point2D::new(0, 0), Point2D::new(0, 8), Point2D::new(1, 8)];
        assert!(is_line_inside_up(&tiles, 0, &tiles[0], &Point2D::new(0, 5)).0);
    }
    #[test]
    fn line_outside() {
        let tiles = vec![
            Point2D::new(0, 0),
            Point2D::new(0, 2),
            Point2D::new(1, 2),
            Point2D::new(1, 5),
            Point2D::new(2, 5),
        ];
        assert!(!is_line_inside_up(&tiles, 0, &tiles[0], &Point2D::new(0, 5)).0);
    }
    #[test]
    fn line_outside_but_comes_back() {
        let tiles = vec![
            Point2D::new(0, 0),
            Point2D::new(0, 2),
            Point2D::new(1, 2),
            Point2D::new(1, 3),
            Point2D::new(0, 3),
            Point2D::new(0, 5),
            Point2D::new(1, 5),
        ];
        assert!(is_line_inside_up(&tiles, 0, &tiles[0], &Point2D::new(0, 5)).0);
    }
    #[test]
    fn line_outside_but_comes_back_late() {
        let tiles = vec![
            Point2D::new(0, 0),
            Point2D::new(0, 2),
            Point2D::new(1, 2),
            Point2D::new(1, 4),
            Point2D::new(0, 4),
            Point2D::new(0, 5),
            Point2D::new(1, 5),
        ];
        assert!(!is_line_inside_up(&tiles, 0, &tiles[0], &Point2D::new(0, 5)).0);
    }
    #[test]
    fn line_outside_good() {
        let tiles = vec![
            Point2D::new(0, 0),
            Point2D::new(0, 2),
            Point2D::new(-1, 2),
            Point2D::new(-1, 5),
            Point2D::new(1, 5),
            Point2D::new(2, 5),
        ];
        assert!(is_line_inside_up(&tiles, 0, &tiles[0], &Point2D::new(0, 5)).0);
    }
    #[test]
    fn line_outside_bad_ending() {
        let tiles = vec![
            Point2D::new(0, 0),
            Point2D::new(0, 2),
            Point2D::new(-2, 2),
            Point2D::new(-2, 5),
            Point2D::new(-1, 5),
            Point2D::new(-1, 4),
            Point2D::new(1, 4),
            Point2D::new(1, 5),
            Point2D::new(2, 5),
        ];
        assert!(!is_line_inside_up(&tiles, 0, &tiles[0], &Point2D::new(0, 5)).0);
    }
    #[test]
    fn line_outside_but_doesnt_come_back_jit() {
        let tiles = vec![
            Point2D::new(0, 0),
            Point2D::new(0, 2),
            Point2D::new(1, 2),
            Point2D::new(1, 6),
            Point2D::new(0, 6),
            Point2D::new(0, 8),
            Point2D::new(1, 8),
        ];
        assert!(!is_line_inside_up(&tiles, 0, &tiles[0], &Point2D::new(0, 5)).0);
    }
    #[test]
    fn line_outside_but_comes_back_jit() {
        let tiles = vec![
            Point2D::new(0, 0),
            Point2D::new(0, 2),
            Point2D::new(1, 2),
            Point2D::new(1, 5),
            Point2D::new(0, 5),
            Point2D::new(1, 5),
        ];
        assert!(is_line_inside_up(&tiles, 0, &tiles[0], &Point2D::new(0, 5)).0);
    }
}

pub fn valid(tiles: &[Point2D], i: usize, j: usize) -> bool {
    /*if tiles[i].x < tiles[j].x {
        if tiles[i].y < tiles[j].y {
            let range = i..=j;

            let mut next =

        } else {
            let range = j..=i;
        }
    } else {
        if tiles[i].y < tiles[j].y {
            let range = j..=i;
        } else {
            let range = i..=j;
        }
    };*/

    true
}

pub fn solution2(reader: BufReader<File>) -> i64 {
    let tiles = read_lines(reader);

    let mut max = 0;

    for (i, tile1) in tiles.iter().enumerate() {
        for (j, tile2) in tiles.iter().enumerate().skip(i + 1) {
            let area = tile1.area(tile2);
            if area > max && valid(&tiles, i, j) {
                max = area;
            }
        }
    }

    max
}
