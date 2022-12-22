use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn largest(
    time: u32,
    cur: usize,
    visited: &mut Vec<bool>,
    rate: &Vec<u32>,
    dist: &Vec<u32>,
    num_nodes: usize,
) -> u32 {
    let mut total = 0 as u32;
    for nxt in 0..num_nodes {
        if !visited[nxt] && rate[nxt] > 0 {
            visited[nxt] = true;
            let d = dist[cur * num_nodes + nxt];
            if d + time < 30 {
                let step = 30 - time - d;
                let rem = largest(time + d + 1, nxt, visited, rate, dist, num_nodes);
                total = std::cmp::max(total, step * rate[nxt] + rem);
            }
            visited[nxt] = false;
        }
    }
    total
}

fn elephant(
    time: u32,
    cur: usize,
    visited: &mut Vec<bool>,
    rate: &Vec<u32>,
    dist: &Vec<u32>,
    num_nodes: usize,
    nvals: u32,
) -> u32 {
    if nvals == 0 {
        largest(5, cur, visited, rate, dist, num_nodes)
    } else {
        let mut total = 0 as u32;
        for nxt in 0..num_nodes {
            if !visited[nxt] && rate[nxt] > 0 {
                visited[nxt] = true;
                let d = dist[cur * num_nodes + nxt];
                if d + time < 26 {
                    let step = 26 - time - d;
                    let rem =
                        elephant(time + d + 1, nxt, visited, rate, dist, num_nodes, nvals - 1);
                    total = std::cmp::max(total, step * rate[nxt] + rem);
                }
                visited[nxt] = false;
            }
        }
        total
    }
}

struct Node {
    rate: u32,
    nbrs: HashSet<usize>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let re = Regex::new(r"Valve (.*) has flow rate=(.*); tunnel[s]* lead[s]* to valve[s]* (.*)\n")
        .unwrap();

    let mut v2n = HashMap::new();
    let mut valves = HashMap::new();

    for cap in re.captures_iter(&file) {
        let valve = &cap[1].to_string();
        let l = v2n.len();
        let num = *v2n.entry(valve.clone()).or_insert(l);

        let rate: u32 = (&cap[2]).parse().expect("rate");
        let mut nbrs = HashSet::new();

        for n in (&cap[3]).split(',') {
            let l = v2n.len();
            let n = v2n.entry(n.trim().to_string()).or_insert(l);
            nbrs.insert(*n);
        }
        valves.insert(
            num,
            Node {
                rate: rate,
                nbrs: nbrs,
            },
        );
    }

    let num_nodes = v2n.len();
    let mut dist = vec![u32::MAX; num_nodes * num_nodes];
    let mut rate = vec![0; num_nodes];
    for (i, node) in &valves {
        rate[*i] = node.rate;
        dist[*i * num_nodes + *i] = 0;
        for j in &node.nbrs {
            dist[*i * num_nodes + *j] = 1;
        }
        let mut visited = vec![false; num_nodes];
        visited[*i] = true;
        let mut nvis = 1;
        while nvis < num_nodes {
            let mut min = u32::MAX;
            let mut min_index = usize::MAX;
            for j in 0..num_nodes {
                if !visited[j] {
                    if dist[*i * num_nodes + j] < min {
                        min = dist[*i * num_nodes + j];
                        min_index = j;
                    }
                }
            }
            visited[min_index] = true;
            nvis += 1;
            for j in &valves[&min_index].nbrs {
                dist[*i * num_nodes + j] = std::cmp::min(min + 1, dist[*i * num_nodes + j]);
            }
        }
    }

    let time = 1;
    let cur = v2n["AA"];
    let mut visited = vec![false; num_nodes];
    visited[cur] = true;
    let total = largest(time, cur, &mut visited, &rate, &dist, num_nodes);
    println!("{}", total);

    let mut nvals = 0;
    for r in &rate {
        if *r > 0 {
            nvals += 1;
        }
    }
    for nv in 0..nvals + 1 {
        let total = elephant(time, cur, &mut visited, &rate, &dist, num_nodes, nv);
        println!("{}", total);
    }
}
