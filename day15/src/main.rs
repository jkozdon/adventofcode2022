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
    let pnt = if args[1] == "demo" { 10 } else { 2000000 };
    let mut list = BTreeSet::new();
    let mut found = BTreeSet::new();
    // sides in rotated coordinates
    let mut right = Vec::new();
    let mut left = Vec::new();
    let mut top = Vec::new();
    let mut bottom = Vec::new();
    let mut sensors = Vec::new();
    for cap in re.captures_iter(&file) {
        let sx: i32 = (&cap[1]).parse().expect("sensor x");
        let sy: i32 = (&cap[2]).parse().expect("sensor y");
        let bx: i32 = (&cap[3]).parse().expect("beacon x");
        let by: i32 = (&cap[4]).parse().expect("beacon y");
        let dist = (sx - bx).abs() + (sy - by).abs();
        sensors.push((sx, sy, dist));
        if sy - dist <= pnt && pnt <= sy + dist {
            let del = dist - (sy - pnt).abs();
            list.insert((sx - del, sx + del + 1));
        }
        if by == pnt {
            found.insert((bx, by));
        }
        let (r, s) = (sx - sy, sx + sy);
        right.push(s + dist);
        left.push(s - dist);
        top.push(r + dist);
        bottom.push(r - dist);
    }
    let mut xs = i32::MIN;
    let mut count = -(found.len() as i32);
    for (a, b) in &list {
        let a = std::cmp::max(xs, *a);
        count += std::cmp::max(0, *b - a);
        xs = std::cmp::max(xs, *b);
    }
    println!("count:     {}", count);

    let mut ss = Vec::new();
    for rt in &right {
        for lf in &left {
            let d = lf - rt;
            if d >= 2 && d <= 4 {
                for i in 1..d {
                    ss.push(rt + i);
                }
            }
        }
    }
    let mut rr = Vec::new();
    for tp in &top {
        for bt in &bottom {
            let d = bt - tp;
            if d >= 2 && d <= 4 {
                for i in 1..d {
                    rr.push(tp + i);
                }
            }
        }
    }

    'outer: for r in &rr {
        for s in &ss {
            let r = *r;
            let s = *s;
            let (x, y) = ((r + s) / 2, (s - r) / 2);
            let mut outside = true;
            for (sx, sy, dist) in &sensors {
                if (*sx - x).abs() + (*sy - y).abs() <= *dist {
                    outside = false;
                    break;
                }
            }
            if outside {
                let x = x as i128;
                let y = y as i128;
                println!("frequency: {}", 4000000 * x + y);
                break 'outer;
            }
        }
    }
}
