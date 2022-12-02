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
        score1 = score1
            + match tok[0] {
                "A" => match tok[1] {
                    "X" => 1 + 3,
                    "Y" => 2 + 6,
                    "Z" => 3 + 0,
                    _ => panic!(),
                },
                "B" => match tok[1] {
                    "X" => 1 + 0,
                    "Y" => 2 + 3,
                    "Z" => 3 + 6,
                    _ => panic!(),
                },
                "C" => match tok[1] {
                    "X" => 1 + 6,
                    "Y" => 2 + 0,
                    "Z" => 3 + 3,
                    _ => panic!(),
                },
                _ => panic!(),
            };

        // op: ABC -> rock, paper, scissors
        // v2: XYZ -> lose, draw, win
        score2 = score2
            + match tok[0] {
                "A" => match tok[1] {
                    "X" => 3 + 0,
                    "Y" => 1 + 3,
                    "Z" => 2 + 6,
                    _ => panic!(),
                },
                "B" => match tok[1] {
                    "X" => 1 + 0,
                    "Y" => 2 + 3,
                    "Z" => 3 + 6,
                    _ => panic!(),
                },
                "C" => match tok[1] {
                    "X" => 2 + 0,
                    "Y" => 3 + 3,
                    "Z" => 1 + 6,
                    _ => panic!(),
                },
                _ => panic!(),
            };
    }

    println!("score1: {}", score1);
    println!("score2: {}", score2);
}
