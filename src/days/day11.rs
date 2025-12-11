use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_lines(reader: BufReader<File>) -> Vec<(String, Vec<String>)> {
    let mut res: Vec<(String, Vec<String>)> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let parts = line.split_once(": ").expect("Spanish inquisition");
            (
                parts.0.to_owned(),
                parts.1.split(' ').map(|s| s.to_owned()).collect(),
            )
        })
        .collect();

    res.push(("out".to_owned(), Vec::new()));

    res
}

pub fn create_mappings(nodes: &[(String, Vec<String>)]) -> HashMap<String, usize> {
    let mut res = HashMap::new();

    for (i, (k, _)) in nodes.iter().enumerate() {
        res.insert(k.clone(), i);
    }

    res
}

pub fn create_matrix(
    nodes: &[(String, Vec<String>)],
    mappings: &HashMap<String, usize>,
) -> Vec<Vec<bool>> {
    let mut res = (0..nodes.len())
        .map(|_x| vec![false; nodes.len()])
        .collect::<Vec<Vec<bool>>>();

    for (k, vs) in nodes {
        let k_i = mappings[k];
        for v in vs.iter() {
            res[k_i][mappings[v]] = true;
        }
    }

    res
}

/*
pub fn naive_bfs(
    nodes: &[(String, Vec<String>)],
    mappings: &HashMap<String, usize>,
    matrix: &[Vec<bool>],
) -> usize {
    let mut ways_to_reach = vec![0; nodes.len()];

    let mut deq = VecDeque::new();
    deq.push_back(mappings["out"]);
    while !deq.is_empty() {
        let val = deq.pop_front().expect("Spanish inquisition");
        ways_to_reach[val] += 1;
        for i in 0..nodes.len() {
            if matrix[i][val] {
                deq.push_back(i);
            }
        }
    }

    ways_to_reach[mappings["you"]]
}
*/
pub fn dfs(
    nodes: &[(String, Vec<String>)],
    mappings: &HashMap<String, usize>,
    matrix: &[Vec<bool>],
    node: usize,
    ways_to_reach: &mut [Option<usize>],
) -> usize {
    if let Some(x) = ways_to_reach[node] {
        return x;
    }
    let mut sum = 0;
    for i in 0..nodes.len() {
        if matrix[node][i] {
            sum += dfs(nodes, mappings, matrix, i, ways_to_reach);
        }
    }

    ways_to_reach[node] = Some(sum);
    return sum;
}

pub fn solution(reader: BufReader<File>) -> usize {
    let nodes = read_lines(reader);
    let mappings = create_mappings(&nodes);
    let matrix = create_matrix(&nodes, &mappings);
    let mut ways_to_reach: Vec<Option<usize>> = vec![None; nodes.len()];
    ways_to_reach[mappings["out"]] = Some(1);

    dfs(
        &nodes,
        &mappings,
        &matrix,
        mappings["you"],
        &mut ways_to_reach,
    )
}

// Paths from srv to fft with no dac * paths from fft to dac * paths from dac to out (with no fft)
// Paths from srv to dac with no fft * paths from dac to fft * paths from fft to out

pub fn solution2(reader: BufReader<File>) -> usize {
    let nodes = read_lines(reader);
    let mappings = create_mappings(&nodes);
    let matrix = create_matrix(&nodes, &mappings);
    let mut ways_to_reach_1a: Vec<Option<usize>> = vec![None; nodes.len()];
    ways_to_reach_1a[mappings["fft"]] = Some(1);
    let mut ways_to_reach_2a: Vec<Option<usize>> = vec![None; nodes.len()];
    ways_to_reach_2a[mappings["dac"]] = Some(1);
    let mut ways_to_reach_3a: Vec<Option<usize>> = vec![None; nodes.len()];
    ways_to_reach_3a[mappings["out"]] = Some(1);

    let mut ways_to_reach_1b: Vec<Option<usize>> = vec![None; nodes.len()];
    ways_to_reach_1b[mappings["dac"]] = Some(1);
    let mut ways_to_reach_2b: Vec<Option<usize>> = vec![None; nodes.len()];
    ways_to_reach_2b[mappings["fft"]] = Some(1);
    let mut ways_to_reach_3b: Vec<Option<usize>> = vec![None; nodes.len()];
    ways_to_reach_3b[mappings["out"]] = Some(1);

    let paths_from_srv_to_fft = dfs(
        &nodes,
        &mappings,
        &matrix,
        mappings["svr"],
        &mut ways_to_reach_1a,
    );
    let paths_from_fft_to_dac = dfs(
        &nodes,
        &mappings,
        &matrix,
        mappings["fft"],
        &mut ways_to_reach_2a,
    );
    let paths_from_dac_to_out = dfs(
        &nodes,
        &mappings,
        &matrix,
        mappings["dac"],
        &mut ways_to_reach_3a,
    );
    let paths_from_srv_to_dac = dfs(
        &nodes,
        &mappings,
        &matrix,
        mappings["svr"],
        &mut ways_to_reach_1b,
    );
    let paths_from_dac_to_ftt = dfs(
        &nodes,
        &mappings,
        &matrix,
        mappings["dac"],
        &mut ways_to_reach_2b,
    );
    let paths_from_ftt_to_out = dfs(
        &nodes,
        &mappings,
        &matrix,
        mappings["fft"],
        &mut ways_to_reach_3b,
    );

    paths_from_srv_to_fft * paths_from_fft_to_dac * paths_from_dac_to_out
        + paths_from_srv_to_dac * paths_from_dac_to_ftt * paths_from_ftt_to_out
}
