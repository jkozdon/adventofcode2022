use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        if line.len() == 0 {}
        map.push(line.chars().collect());
    }
    let mut route = file.lines().last().unwrap();

    let mut pos: (usize, usize) = (0, 0);
    for (i, v) in map[0].iter().enumerate() {
        if *v == '.' {
            pos = (0, i);
            break;
        }
    }

    for mv in route.split_inclusive(|c: char| c == 'R' || c == 'L') {
        let (num, turn) = if mv.chars().last().unwrap() == 'R' {
            (mv.get(0..mv.len() - 1).unwrap(), Some('R'))
        } else if mv.chars().last().unwrap() == 'L' {
            (mv.get(0..mv.len() - 1).unwrap(), Some('L'))
        } else {
            (mv, None)
        };
        println!("{} {:?}", num, turn);
    }
}
