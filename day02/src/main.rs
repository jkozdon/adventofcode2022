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
        // score: lose(0), draw(3), win(6)
        // play:  rock(1), paper(2), scissors(3)
        // op: ABC -> rock, paper, scissors
        // v1: XYZ -> rock, paper, scissors
        match tok[0] {
            "A" => match tok[1] {
                "X" => score1 = score1 + 1 + 3,
                "Y" => score1 = score1 + 2 + 6,
                "Z" => score1 = score1 + 3 + 0,
                _ => (),
            },
            "B" => match tok[1] {
                "X" => score1 = score1 + 1 + 0,
                "Y" => score1 = score1 + 2 + 3,
                "Z" => score1 = score1 + 3 + 6,
                _ => (),
            },
            "C" => match tok[1] {
                "X" => score1 = score1 + 1 + 6,
                "Y" => score1 = score1 + 2 + 0,
                "Z" => score1 = score1 + 3 + 3,
                _ => (),
            },
            _ => (),
        }

        // op: ABC -> rock, paper, scissors
        // v2: XYZ -> lose, draw, win
        match tok[0] {
            "A" => match tok[1] {
                "X" => score2 = score2 + 3 + 0,
                "Y" => score2 = score2 + 1 + 3,
                "Z" => score2 = score2 + 2 + 6,
                _ => (),
            },
            "B" => match tok[1] {
                "X" => score2 = score2 + 1 + 0,
                "Y" => score2 = score2 + 2 + 3,
                "Z" => score2 = score2 + 3 + 6,
                _ => (),
            },
            "C" => match tok[1] {
                "X" => score2 = score2 + 2 + 0,
                "Y" => score2 = score2 + 3 + 3,
                "Z" => score2 = score2 + 1 + 6,
                _ => (),
            },
            _ => (),
        }
    }

    println!("score1: {}", score1);
    println!("score2: {}", score2);
}
