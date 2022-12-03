use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut item_score = 0;
    let mut badge_score = 0;

    // set of badges
    let mut badges = vec![vec![0; 52]; 3];
    for (iter, line) in file.lines().enumerate() {
        let badge = iter % 3;
        // reset badges because in a new set
        if badge == 0 {
            for b in 0..3 {
                for n in &mut badges[b] {
                    *n = 0;
                }
            }
        }

        // two compartments
        let mut compartments = vec![vec![0; 52]; 2];
        for (i, c) in line.chars().enumerate() {

            let v: usize = if 'A' <= c && c <= 'Z' {
                (c as usize) - ('A' as usize) + 26
            } else {
                (c as usize) - ('a' as usize)
            };

            // If item is on both compartments were done
            let compartment = i / (line.len() / 2);
            compartments[compartment][v] = compartments[compartment][v] + 1;
            if compartment == 1 && compartments[0][v] > 0 && compartments[1][v] == 1 {
                item_score = item_score + v + 1;
            }

            // If item is in all three badges were done
            badges[badge][v] = badges[badge][v] + 1;
            if badge == 2 && badges[0][v] > 0 && badges[1][v] > 0 && badges[2][v] == 1 {
                badge_score = badge_score + v + 1;
            }
        }
    }

    println!("items score:  {}", item_score);
    println!("badges score: {}", badge_score);
}
