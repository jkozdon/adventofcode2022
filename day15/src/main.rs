use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let (mut xmin, mut xmax) = (i32::MAX, i32::MIN);
    let (mut ymin, mut ymax) = (i32::MAX, i32::MIN);
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    for cap in re.captures_iter(&file) {
        let sx: i32 = (&cap[1]).parse().expect("sensor x");
        let sy: i32 = (&cap[2]).parse().expect("sensor y");
        let bx: i32 = (&cap[3]).parse().expect("beacon x");
        let by: i32 = (&cap[4]).parse().expect("beacon y");
        let dist = (sx-bx).abs() + (bx-by).abs();
        (xmin, xmax) = (std::cmp::min(xmin, sx-dist), (std::cmp::max(xmax, sx+dist)));
        (ymin, ymax) = (std::cmp::min(ymin, sy-dist), (std::cmp::max(ymax, sy+dist)));
    }
    println!("({}, {}) -- ({}, {})", xmin, ymin, xmax, ymax);
}
