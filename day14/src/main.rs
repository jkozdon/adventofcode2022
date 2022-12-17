use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let (mut xmin, mut xmax) = (500, 500);
    let (mut ymin, mut ymax) = (0, 0);
    for l in file.split('\n') {
        for p in l.trim().split(" -> ") {
            if let Some((x, y)) = p.split_once(',') {
                let x: i32 = x.parse().expect("number");
                let y: i32 = y.parse().expect("number");
                xmax = std::cmp::max(x, xmax);
                xmin = std::cmp::min(x, xmin);
                ymax = std::cmp::max(y, ymax);
                ymin = std::cmp::min(y, ymin);
            }
        }
    }
    println!("{} {} {} {}", xmin, xmax, ymin, ymax);
}
