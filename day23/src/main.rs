use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut elves = HashMap::new();
    let mut elf: u32 = 0;

    for (j, line) in file.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((i as i64, j as i64), elf);
                elf += 1;
            }
        }
    }
}
