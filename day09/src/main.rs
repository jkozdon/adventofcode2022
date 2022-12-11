use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut visitbeg = HashSet::new();
    let mut visitend = HashSet::new();
    let mut rope = [[0 as i32; 2]; 10];
    visitbeg.insert(rope[1]);
    visitend.insert(rope[1]);
    for x in file.lines() {
        let (d, n) = x.split_once(' ').unwrap();
        let n: i32 = n.parse().expect("number");
        for _ in 0..n {
            match d {
                "U" => {
                    rope[0][1] += 1;
                }
                "D" => {
                    rope[0][1] -= 1;
                }
                "R" => {
                    rope[0][0] += 1;
                }
                "L" => {
                    rope[0][0] -= 1;
                }
                d => panic!("unknown direction: {}", d),
            };
            for r in 1..rope.len() {
                while (rope[r][0] - rope[r - 1][0]).abs() > 1
                    || (rope[r][1] - rope[r - 1][1]).abs() > 1
                {
                    if rope[r][0] > rope[r - 1][0] {
                        rope[r][0] -= 1;
                    } else if rope[r][0] < rope[r - 1][0] {
                        rope[r][0] += 1;
                    }
                    if rope[r][1] > rope[r - 1][1] {
                        rope[r][1] -= 1;
                    } else if rope[r][1] < rope[r - 1][1] {
                        rope[r][1] += 1;
                    }
                    if r == 1 {
                        visitbeg.insert(rope[r]);
                    }
                    if r + 1 == rope.len() {
                        visitend.insert(rope[r]);
                    }
                }
            }
        }
    }
    println!("first knot visits: {}", visitbeg.len());
    println!("last knot visits:  {}", visitend.len());
}
