use std::env;
use std::fs;
use std::iter::zip;

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
    for ((n, s), (e, w)) in zip(zip(north, south), zip(east, west)) {
        print!("#");
        for i in 1..width - 1 {
            let mut c = '.';
            let mut count = 0;
            if *n & 1 << i != 0 {
                c = '^';
                count += 1;
            }
            if *s & 1 << i != 0 {
                c = 'v';
                count += 1;
            }
            if *w & 1 << i != 0 {
                c = '<';
                count += 1;
            }
            if *e & 1 << i != 0 {
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
    let top = north[0];
    for j in 1..north.len() {
        north[j - 1] = north[j];
    }
    let n = north.len();
    north[n - 1] = top;

    let n = south.len();
    let bot = south[n - 1];
    for j in (1..south.len()).rev() {
        south[j] = south[j - 1];
    }
    south[0] = bot;

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

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let width = file.lines().nth(0).unwrap().chars().count();
    assert!(128 >= width);
    let height = file.lines().count();
    let mut north = vec![0 as u128; height - 2];
    let mut south = vec![0 as u128; height - 2];
    let mut east = vec![0 as u128; height - 2];
    let mut west = vec![0 as u128; height - 2];
    for (j, line) in file.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '<' {
                west[j - 1] |= 1 << i;
            } else if c == '>' {
                east[j - 1] |= 1 << i;
            } else if c == '^' {
                north[j - 1] |= 1 << i;
            } else if c == 'v' {
                south[j - 1] |= 1 << i;
            }
        }
    }

    print_map(&north, &south, &east, &west, width);
    println!("");
    step_map(&mut north, &mut south, &mut east, &mut west, width);
    print_map(&north, &south, &east, &west, width);
    println!("");
    step_map(&mut north, &mut south, &mut east, &mut west, width);
    print_map(&north, &south, &east, &west, width);
    println!("");
    step_map(&mut north, &mut south, &mut east, &mut west, width);
    print_map(&north, &south, &east, &west, width);
    println!("");
    step_map(&mut north, &mut south, &mut east, &mut west, width);
    print_map(&north, &south, &east, &west, width);
    println!("");
}
