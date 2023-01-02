use std::collections::HashMap;
use std::env;
use std::fs;

#[allow(dead_code)]
fn print_map(
    north: &Vec<u128>,
    south: &Vec<u128>,
    east: &Vec<u128>,
    west: &Vec<u128>,
    width: usize,
) {
    for i in 0..width {
        if i == 1 {
            print!(".");
        } else {
            print!("#");
        }
    }
    println!("");
    for j in 1..north.len() - 1 {
        let n = north[j];
        let s = south[j];
        let e = east[j];
        let w = west[j];
        print!("#");
        for i in 1..width - 1 {
            let mut c = '.';
            let mut count = 0;
            if n & 1 << i != 0 {
                c = '^';
                count += 1;
            }
            if s & 1 << i != 0 {
                c = 'v';
                count += 1;
            }
            if w & 1 << i != 0 {
                c = '<';
                count += 1;
            }
            if e & 1 << i != 0 {
                c = '>';
                count += 1;
            }
            if count > 1 {
                print!("{count}");
            } else {
                print!("{c}");
            }
        }
        println!("#");
    }
    for i in 0..width {
        if i == width - 2 {
            print!(".");
        } else {
            print!("#");
        }
    }
    println!("");
}

fn step_map(
    north: &mut Vec<u128>,
    south: &mut Vec<u128>,
    east: &mut Vec<u128>,
    west: &mut Vec<u128>,
    width: usize,
) {
    let height = north.len();
    for j in 1..north.len() {
        north[j - 1] = north[j];
    }
    north[height - 2] = north[0];
    north[0] = 0;

    for j in (1..south.len()).rev() {
        south[j] = south[j - 1];
    }
    south[1] = south[height - 1];
    south[height - 1] = 0;

    for j in 0..west.len() {
        west[j] >>= 1;
        if 1 & west[j] != 0 {
            west[j] &= !1;
            west[j] |= 1 << width - 2;
        }
    }

    for j in 0..east.len() {
        east[j] <<= 1;
        let chk = (1 as u128) << width - 1;
        if chk & east[j] != 0 {
            east[j] &= !chk;
            east[j] |= 1 << 1;
        }
    }
}

fn recurse(
    cache: &mut HashMap<(usize, usize, u32), u32>,
    step: u32,
    mut best_step: u32,
    (i, j): (usize, usize),
    north: &mut Vec<u128>,
    south: &mut Vec<u128>,
    east: &mut Vec<u128>,
    west: &mut Vec<u128>,
    width: usize,
    target: (usize, usize),
) -> u32 {
    // Fast fail
    if step > best_step {
        return u32::MAX;
    }

    // Check if we have seen this state already
    let height = north.len();
    let chk = (i, j, step % (((width - 2) * (height - 2)) as u32));
    match cache.get(&chk) {
        Some(n) => {
            if *n <= step {
                return u32::MAX;
            }
        }
        _ => (),
    };
    cache.insert(chk, step);

    // If in the last spot we can exit!
    if (i, j) == target {
        return step + 1;
    }

    // Advance blizzard
    step_map(north, south, east, west, width);

    // If at starting position then we can only move down
    if j == 0 {
        assert!(i == 1);
        // See if we can move down
        let f = north[1] | south[1] | west[1] | east[1];
        if f & 1 << i == 0 {
            let new_step = recurse(
                cache,
                step + 1,
                best_step,
                (i, j + 1),
                north,
                south,
                east,
                west,
                width,
                target,
            );
            best_step = std::cmp::min(new_step, best_step);
        };
    } else if j == height - 1 {
        assert!(i == width - 2);
        // See if we can move down
        let f = north[height - 2] | south[height - 2] | west[height - 2] | east[height - 2];
        if f & 1 << i == 0 {
            let new_step = recurse(
                cache,
                step + 1,
                best_step,
                (i, j - 1),
                north,
                south,
                east,
                west,
                width,
                target,
            );
            best_step = std::cmp::min(new_step, best_step);
        };
    } else {
        // go south?
        let fd = north[j + 1] | south[j + 1] | west[j + 1] | east[j + 1];
        let fo = north[j] | south[j] | west[j] | east[j];
        let fu = north[j - 1] | south[j - 1] | west[j - 1] | east[j - 1];
        if j < height - 2 && fd & 1 << i == 0 {
            let new_step = recurse(
                cache,
                step + 1,
                best_step,
                (i, j + 1),
                north,
                south,
                east,
                west,
                width,
                target,
            );
            best_step = std::cmp::min(new_step, best_step);
        };

        // go east?
        if i < width - 2 && fo & 1 << (i + 1) == 0 {
            let new_step = recurse(
                cache,
                step + 1,
                best_step,
                (i + 1, j),
                north,
                south,
                east,
                west,
                width,
                target,
            );
            best_step = std::cmp::min(new_step, best_step);
        };

        // go west?
        if i > 1 && fo & 1 << (i - 1) == 0 {
            let new_step = recurse(
                cache,
                step + 1,
                best_step,
                (i - 1, j),
                north,
                south,
                east,
                west,
                width,
                target,
            );
            best_step = std::cmp::min(new_step, best_step);
        };

        // go north?
        if j > 1 && fu & 1 << i == 0 {
            let new_step = recurse(
                cache,
                step + 1,
                best_step,
                (i, j - 1),
                north,
                south,
                east,
                west,
                width,
                target,
            );
            best_step = std::cmp::min(new_step, best_step);
        };
    }
    // Stay put?
    let fo = north[j] | south[j] | west[j] | east[j];
    if fo & 1 << i == 0 {
        let new_step = recurse(
            cache,
            step + 1,
            best_step,
            (i, j),
            north,
            south,
            east,
            west,
            width,
            target,
        );
        best_step = std::cmp::min(new_step, best_step);
    }

    // reverse blizzard
    step_map(south, north, west, east, width);
    best_step
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let width = file.lines().nth(0).unwrap().chars().count();
    assert!(128 >= width);
    let height = file.lines().count();
    let mut north = vec![0 as u128; height];
    let mut south = vec![0 as u128; height];
    let mut east = vec![0 as u128; height];
    let mut west = vec![0 as u128; height];
    for (j, line) in file.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '<' {
                west[j] |= 1 << i;
            } else if c == '>' {
                east[j] |= 1 << i;
            } else if c == '^' {
                north[j] |= 1 << i;
            } else if c == 'v' {
                south[j] |= 1 << i;
            }
        }
    }

    let max_trip = 300;
    let pos: (usize, usize) = (1, 0);
    let mut cache = HashMap::<(usize, usize, u32), u32>::new();
    let first = recurse(
        &mut cache,
        0,
        max_trip, // set for faster stopping
        pos,
        &mut north,
        &mut south,
        &mut east,
        &mut west,
        width,
        (width - 2, height - 2),
    );
    assert!(first < max_trip);
    println!("first trip:           {}", first);

    for _ in 0..first {
        step_map(&mut north, &mut south, &mut east, &mut west, width);
    }
    let pos: (usize, usize) = (width - 2, height - 1);
    let mut cache = HashMap::<(usize, usize, u32), u32>::new();
    let second = recurse(
        &mut cache,
        0,
        max_trip, // set for faster stopping
        pos,
        &mut north,
        &mut south,
        &mut east,
        &mut west,
        width,
        (1, 1),
    );
    assert!(second < max_trip);

    for _ in 0..second {
        step_map(&mut north, &mut south, &mut east, &mut west, width);
    }
    let pos: (usize, usize) = (1, 0);
    let mut cache = HashMap::<(usize, usize, u32), u32>::new();
    let third = recurse(
        &mut cache,
        0,
        max_trip, // set for faster stopping
        pos,
        &mut north,
        &mut south,
        &mut east,
        &mut west,
        width,
        (width - 2, height - 2),
    );
    assert!(third < max_trip);
    println!("there and back again: {}", first + second + third);
}
