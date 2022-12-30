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
}
