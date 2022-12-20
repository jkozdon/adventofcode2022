use regex::Regex;
use std::collections::BTreeSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    //let mut list: LinkedList<(i32, i32)> = LinkedList::new();
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)\n")
            .unwrap();
    let pnt = 2000000;
    // let pnt = 10;
    let mut list = BTreeSet::new();
    let mut found = BTreeSet::new();
    for cap in re.captures_iter(&file) {
        let sx: i32 = (&cap[1]).parse().expect("sensor x");
        let sy: i32 = (&cap[2]).parse().expect("sensor y");
        let bx: i32 = (&cap[3]).parse().expect("beacon x");
        let by: i32 = (&cap[4]).parse().expect("beacon y");
        let dist = (sx - bx).abs() + (sy - by).abs();
        if sy - dist <= pnt && pnt <= sy + dist {
            let del = dist - (sy - pnt).abs();
            list.insert((sx - del, sx + del + 1));
        }
        if by == pnt {
            found.insert((bx, by));
        }
    }
    let mut xs = i32::MIN;
    let mut count = -(found.len() as i32);
    for (a, b) in &list {
        let a = std::cmp::max(xs, *a);
        count += std::cmp::max(0, *b - a);
        xs = std::cmp::max(xs, *b);
    }
    println!("{}", count);
}
