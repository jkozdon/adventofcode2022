use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let stacks = (file.lines().nth(0).unwrap().len() + 1) / 4;
    let mut stack9000 = vec![VecDeque::new(); stacks];
    let mut stack9001 = vec![VecDeque::new(); stacks];

    let mut moves = false;
    for l in file.lines() {
        if !moves {
            if l.chars().nth(1).unwrap() == '1' {
                moves = true;
                continue;
            }
            for i in 0..stack9000.len() {
                let c = l.chars().nth(4 * i + 1).unwrap();
                if c != ' ' {
                    stack9000[i].push_front(c);
                    stack9001[i].push_front(c);
                }
            }
        } else if l.len() > 0 {
            let tok: Vec<&str> = l.splitn(6, ' ').collect();
            let num: usize = tok[1].parse().expect("number");
            let src: usize = tok[3].parse().expect("number");
            let dst: usize = tok[5].parse().expect("number");
            for _ in 0..num {
                let c = stack9000[src - 1].pop_back().unwrap();
                stack9000[dst - 1].push_back(c);
                stack9001[dst - 1].push_back(' ');
            }
            let n = stack9001[dst - 1].len();
            for i in 0..num {
                let c = stack9001[src - 1].pop_back().unwrap();
                stack9001[dst - 1][n - i - 1] = c;
            }
        }
    }

    print!("CrateMover 9000: ");
    for i in 0..stack9000.len() {
        print!("{}", stack9000[i].pop_back().unwrap());
    }
    println!();
    print!("CrateMover 9001: ");
    for i in 0..stack9001.len() {
        print!("{}", stack9001[i].pop_back().unwrap());
    }
    println!();
}
