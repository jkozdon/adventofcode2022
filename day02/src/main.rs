use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut score1 = 0;
    let mut score2 = 0;

    for line in file.lines() {
        let tok: Vec<&str> = line.split(' ').collect();
        if tok[0] == "A" && tok[1] == "X" {
            score1 = score1 + 1 + 3;
            score2 = score2 + 0 + 3;
        } else if tok[0] == "A" && tok[1] == "Y" {
            score1 = score1 + 2 + 6;
            score2 = score2 + 3 + 1;
        } else if tok[0] == "A" && tok[1] == "Z" {
            score1 = score1 + 3 + 0;
            score2 = score2 + 6 + 2;
        } else if tok[0] == "B" && tok[1] == "X" {
            score1 = score1 + 1 + 0;
            score2 = score2 + 0 + 1;
        } else if tok[0] == "B" && tok[1] == "Y" {
            score1 = score1 + 2 + 3;
            score2 = score2 + 3 + 2;
        } else if tok[0] == "B" && tok[1] == "Z" {
            score1 = score1 + 3 + 6;
            score2 = score2 + 6 + 3;
        } else if tok[0] == "C" && tok[1] == "X" {
            score1 = score1 + 1 + 6;
            score2 = score2 + 0 + 2;
        } else if tok[0] == "C" && tok[1] == "Y" {
            score1 = score1 + 2 + 0;
            score2 = score2 + 3 + 3;
        } else if tok[0] == "C" && tok[1] == "Z" {
            score1 = score1 + 3 + 3;
            score2 = score2 + 6 + 1;
        }
    }

    println!("score1:   {}", score1);
    println!("score2:   {}", score2);
}
