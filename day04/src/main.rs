use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut full_overlap = 0;
    let mut part_overlap = 0;
    for line in file.lines() {
        let p: Vec<(i32, i32)> = line
            .split(',')
            .map(|x| {
                let (a, b) = x.split_once('-').unwrap();
                (a.parse().expect("number"), b.parse().expect("number"))
            })
            .collect();
        if (p[0].0 <= p[1].0 && p[0].1 >= p[1].1) || (p[0].0 >= p[1].0 && p[0].1 <= p[1].1) {
            full_overlap = full_overlap + 1;
        }
        if (p[0].0 <= p[1].0 && p[1].0 <= p[0].1)
            || (p[0].0 <= p[1].1 && p[1].1 <= p[0].1)
            || (p[1].0 <= p[0].0 && p[0].0 <= p[1].1)
            || (p[1].0 <= p[0].0 && p[0].0 <= p[1].1)
        {
            part_overlap = part_overlap + 1;
        }
    }
    println!("full overlap: {}", full_overlap);
    println!("part overlap: {}", part_overlap);
}
