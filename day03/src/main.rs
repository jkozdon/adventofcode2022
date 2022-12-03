use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut item_score = 0;
    let mut badge_score = 0;

    // set of badges
    let mut bm_badges = vec![0u64; 3];
    for (iter, line) in file.lines().enumerate() {
        let badge = iter % 3;

        // reset badges because in a new set
        if badge == 0 {
            for b in &mut bm_badges {
                *b = 0
            }
        }

        let mut bm_comp = vec![0u64; 2];

        let mut check_badge = badge == 2;
        let mut check_comp = true;

        for (i, c) in line.chars().enumerate() {

            let v: usize = if 'A' <= c && c <= 'Z' {
                (c as usize) - ('A' as usize) + 26
            } else {
                (c as usize) - ('a' as usize)
            };

            // If item is on both compartments were done
            let comp = i / (line.len() / 2);
            bm_comp[comp] = bm_comp[comp] | (1 << v);
            if comp == 1 && check_comp && bm_comp[0] & bm_comp[1] > 0 {
                item_score = item_score + v + 1;
                check_comp = false;
            }

            // If item is in all three badges were done
            bm_badges[badge] = bm_badges[badge] | (1 << v);
            if check_badge && bm_badges[0] & bm_badges[1] & bm_badges[2] > 0 {
                badge_score = badge_score + v + 1;
                check_badge = false;
            }
        }
    }

    println!("items score:  {}", item_score);
    println!("badges score: {}", badge_score);
}
