use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

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

    let mut time = 1;
    let mut cur = v2n["AA"];
    let mut pres = 0;
    let mut visited = vec![false; num_nodes];
    visited[cur] = true;
    while time < 30 {
        time += 1;
    }
}
