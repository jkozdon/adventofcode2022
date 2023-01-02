use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn has_neighbors(i: i64, j: i64, elves: &HashSet<(i64, i64)>) -> bool {
    for jj in j - 1..j + 2 {
        for ii in i - 1..i + 2 {
            if ii != i || jj != j {
                if elves.contains(&(ii, jj)) {
                    return true;
                }
            }
        }
    }
    false
}

fn get_move(i: i64, j: i64, round: usize, elves: &HashSet<(i64, i64)>) -> (i64, i64) {
    for step in round..round + 4 {
        match step % 4 {
            // north
            0 => {
                if !elves.contains(&(i - 1, j - 1))
                    && !elves.contains(&(i + 0, j - 1))
                    && !elves.contains(&(i + 1, j - 1))
                {
                    // println!("N: ({}, {})", i, j);
                    return (i, j - 1);
                }
            }
            // south
            1 => {
                if !elves.contains(&(i - 1, j + 1))
                    && !elves.contains(&(i + 0, j + 1))
                    && !elves.contains(&(i + 1, j + 1))
                {
                    // println!("S: ({}, {})", i, j);
                    return (i, j + 1);
                }
            }
            // west
            2 => {
                if !elves.contains(&(i - 1, j - 1))
                    && !elves.contains(&(i - 1, j + 0))
                    && !elves.contains(&(i - 1, j + 1))
                {
                    // println!("W: ({}, {})", i, j);
                    return (i - 1, j);
                }
            }
            // east
            3 => {
                if !elves.contains(&(i + 1, j - 1))
                    && !elves.contains(&(i + 1, j + 0))
                    && !elves.contains(&(i + 1, j + 1))
                {
                    // println!("E: ({}, {})", i, j);
                    return (i + 1, j);
                }
            }
            _ => panic!(),
        }
    }
    (i, j)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut elves = HashSet::new();

    for (j, line) in file.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((i as i64, j as i64));
            }
        }
    }

    for round in 0..10 {
        let mut m2e = HashMap::new();
        let mut e2m = HashMap::new();
        for (i, j) in &elves {
            let mv = if has_neighbors(*i, *j, &elves) {
                let mv = get_move(*i, *j, round, &elves);
                if m2e.contains_key(&mv) {
                    let n = m2e.get(&mv).unwrap();
                    e2m.insert(*n, *n);
                    (*i, *j)
                } else {
                    mv
                }
            } else {
                (*i, *j)
            };

            m2e.insert(mv, (*i, *j));
            e2m.insert((*i, *j), mv);
        }
        elves.drain();
        for (_, (i, j)) in e2m {
            elves.insert((i, j));
        }
    }

    let (mut imin, mut imax) = (i64::MAX, i64::MIN);
    let (mut jmin, mut jmax) = (i64::MAX, i64::MIN);
    for (i, j) in &elves {
        imin = std::cmp::min(imin, *i);
        imax = std::cmp::max(imax, *i);
        jmin = std::cmp::min(jmin, *j);
        jmax = std::cmp::max(jmax, *j);
    }
    println!(
        "empty tiles: {}",
        (imax - imin + 1) * (jmax - jmin + 1) - (elves.len() as i64)
    );
    // for j in jmin..jmax+1 {
    //     for i in imin..imax+1 {
    //         if elves.contains(&(i, j)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    let mut has_move = true;
    let mut round = 9;
    while has_move {
        has_move = false;
        round += 1;
        let mut m2e = HashMap::new();
        let mut e2m = HashMap::new();
        for (i, j) in &elves {
            let mv = if has_neighbors(*i, *j, &elves) {
                has_move = true;
                let mv = get_move(*i, *j, round, &elves);
                if m2e.contains_key(&mv) {
                    let n = m2e.get(&mv).unwrap();
                    e2m.insert(*n, *n);
                    (*i, *j)
                } else {
                    mv
                }
            } else {
                (*i, *j)
            };

            m2e.insert(mv, (*i, *j));
            e2m.insert((*i, *j), mv);
        }
        elves.drain();
        for (_, (i, j)) in e2m {
            elves.insert((i, j));
        }
    }
    println!("total moves: {}", round + 1);
}
