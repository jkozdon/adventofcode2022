use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        map.push(line.chars().collect());
    }
    let route = map.pop().unwrap();
    assert!(map.pop().unwrap().len() == 0);
    let mut pos: (usize, usize) = (0, 0);
    for (i, v) in map[0].iter().enumerate() {
        if *v == '.' {
            pos = (0, i);
            break;
        }
    }
}
