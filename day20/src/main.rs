use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");
    let data: Vec<i32> = file
        .lines()
        .map(|l| l.trim().parse::<i32>().expect("number"))
        .collect();

    let n = data.len();
    let mut o2n: Vec<usize> = (0..n).collect();
    let mut n2o: Vec<usize> = (0..n).collect();

    for (old_i, v) in data.iter().enumerate() {
        let mut shift = *v % (n - 1) as i32;
        while shift != 0 {
            let new_i = o2n[old_i];
            assert!(new_i == o2n[n2o[new_i]]);
            assert!(old_i == n2o[o2n[old_i]]);
            let new_j = if shift < 0 {
                shift += 1;
                if new_i > 0 {
                    new_i - 1
                } else {
                    n - 1
                }
            } else {
                shift -= 1;
                if new_i + 1 < n {
                    new_i + 1
                } else {
                    0
                }
            };

            let old_j = n2o[new_j];
            assert!(new_j == o2n[n2o[new_j]]);
            assert!(old_j == n2o[o2n[old_j]]);
            o2n[old_i] = new_j;
            o2n[old_j] = new_i;
            n2o[new_i] = old_j;
            n2o[new_j] = old_i;
        }
    }
    let mut z = 0;
    for i in 0..n {
        if data[n2o[i]] == 0 {
            z = i;
        }
    }
    println!(
        "part a: {}",
        data[n2o[(z + 1000) % n]] + data[n2o[(z + 2000) % n]] + data[n2o[(z + 3000) % n]]
    );

    let mut o2n: Vec<usize> = (0..n).collect();
    let mut n2o: Vec<usize> = (0..n).collect();
    for _ in 0..10 {
        for (old_i, v) in data.iter().enumerate() {
            let new_v = *v as i64;
            let mut shift = (new_v * 811589153) % (n - 1) as i64;
            while shift != 0 {
                let new_i = o2n[old_i];
                assert!(new_i == o2n[n2o[new_i]]);
                assert!(old_i == n2o[o2n[old_i]]);
                let new_j = if shift < 0 {
                    shift += 1;
                    if new_i > 0 {
                        new_i - 1
                    } else {
                        n - 1
                    }
                } else {
                    shift -= 1;
                    if new_i + 1 < n {
                        new_i + 1
                    } else {
                        0
                    }
                };

                let old_j = n2o[new_j];
                assert!(new_j == o2n[n2o[new_j]]);
                assert!(old_j == n2o[o2n[old_j]]);
                o2n[old_i] = new_j;
                o2n[old_j] = new_i;
                n2o[new_i] = old_j;
                n2o[new_j] = old_i;
            }
        }
    }
    let mut z = 0;
    for i in 0..n {
        if data[n2o[i]] == 0 {
            z = i;
        }
    }
    println!(
        "part b: {}",
        811589153
            * ((data[n2o[(z + 1000) % n]] + data[n2o[(z + 2000) % n]] + data[n2o[(z + 3000) % n]])
                as i64)
    );
}
