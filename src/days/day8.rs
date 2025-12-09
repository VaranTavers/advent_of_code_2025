use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseFloatError,
};

pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn dist(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    fn from_str(line: &str, delim: char) -> Result<Self, ParseFloatError> {
        let parts = line.split(delim).collect::<Vec<&str>>();
        Ok(Point3D {
            x: parts[0].parse::<f64>()?,
            y: parts[1].parse::<f64>()?,
            z: parts[2].parse::<f64>()?,
        })
    }
}

pub fn read_lines(reader: BufReader<File>) -> Vec<Point3D> {
    reader
        .lines()
        .map_while(Result::ok)
        .flat_map(|x| Point3D::from_str(&x, ','))
        .collect()
}

pub fn merge_components(components: &mut [usize], a: usize, b: usize) {
    for c in components.iter_mut() {
        if *c == b {
            *c = a;
        }
    }
}

pub fn solution(reader: BufReader<File>) -> usize {
    let boxes = read_lines(reader);
    let mut dists = Vec::new();
    let mut components = (0..boxes.len()).collect::<Vec<usize>>();
    let mut sizes = components.iter().map(|_x| 1).collect::<Vec<usize>>();

    for (i, b1) in boxes.iter().enumerate() {
        for (j, b2) in boxes.iter().enumerate().skip(i + 1) {
            dists.push((b2.dist(b1), i, j));
        }
    }

    dists.sort_by(|(x1, _a1, _b1), (x2, _a2, _b2)| x1.total_cmp(x2));

    let mut i = 0;

    for (_d, a, b) in &dists {
        let comp_a = components[*a];
        let comp_b = components[*b];
        //println!("{d} {a} {b} {comp_a} {comp_b}");
        merge_components(&mut components, comp_a, comp_b);
        if comp_a != comp_b {
            sizes[comp_a] += sizes[comp_b];
            sizes[comp_b] = 0;
        }
        i += 1;
        //println!("{components:?}");
        //println!("{sizes:?}");
        if i == 1000 {
            break;
        }
    }

    let mut sorted_sizes = sizes
        .iter()
        .skip(1)
        .enumerate()
        .map(|(x, y)| (*y, x + 1))
        .collect::<Vec<(usize, usize)>>();
    sorted_sizes.sort_unstable();
    let l = sorted_sizes.len();
    //println!("{sorted_sizes:?}");

    sorted_sizes[l - 1].0 * sorted_sizes[l - 2].0 * sorted_sizes[l - 3].0
}

pub fn solution2(reader: BufReader<File>) -> f64 {
    let boxes = read_lines(reader);
    let mut dists = Vec::new();
    let mut components = (0..boxes.len()).collect::<Vec<usize>>();
    let mut sizes = components.iter().map(|_x| 1).collect::<Vec<usize>>();

    for (i, b1) in boxes.iter().enumerate() {
        for (j, b2) in boxes.iter().enumerate().skip(i + 1) {
            dists.push((b2.dist(b1), i, j));
        }
    }

    dists.sort_by(|(x1, _a1, _b1), (x2, _a2, _b2)| x1.total_cmp(x2));

    let mut max_size = 0;

    for (_d, a, b) in &dists {
        let comp_a = components[*a];
        let comp_b = components[*b];
        //println!("{d} {a} {b} {comp_a} {comp_b}");
        if comp_a != comp_b {
            merge_components(&mut components, comp_a, comp_b);
            sizes[comp_a] += sizes[comp_b];
            sizes[comp_b] = 0;
            if sizes[comp_a] > max_size {
                max_size = sizes[comp_a];
            }
        }

        if max_size == boxes.len() {
            return boxes[*a].x * boxes[*b].x;
        }
    }
    println!("{max_size}");

    0.0
}
