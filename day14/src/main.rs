use std::env;
use std::fs;

fn insert(grid: &mut Vec<u128>, x: usize, y: usize, shift: usize) {
    grid[y - 1] |= (1 as u128) << x - shift;
}

fn filled(grid: &Vec<u128>, x: usize, y: usize, shift: usize) -> bool {
    grid[y - 1] & ((1 as u128) << x - shift) != 0
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let (mut xmin, mut xmax) = (500, 500);
    let (mut ymin, mut ymax) = (0, 0);
    let mut rocks = Vec::new();
    for l in file.split('\n') {
        let mut wall = Vec::new();
        for p in l.trim().split(" -> ") {
            if let Some((x, y)) = p.split_once(',') {
                let x: usize = x.parse().expect("number");
                let y: usize = y.parse().expect("number");
                xmax = std::cmp::max(x, xmax);
                xmin = std::cmp::min(x, xmin);
                ymax = std::cmp::max(y, ymax);
                ymin = std::cmp::min(y, ymin);
                wall.push((x, y));
            }
        }
        rocks.push(wall);
    }
    let height = ymax;
    let width = xmax - xmin;
    assert!(width < 128);
    let shift = xmin;
    let mut grid = vec![0 as u128; height];
    for wall in rocks {
        if wall.len() > 0 {
            let (mut x0, mut y0) = wall[0];
            insert(&mut grid, x0, y0, shift);
            for (x1, y1) in &wall[1..] {
                if x0 == *x1 {
                    for y in std::cmp::min(y0, *y1)..std::cmp::max(y0, *y1) + 1 {
                        insert(&mut grid, x0, y, shift);
                    }
                } else if y0 == *y1 {
                    for x in std::cmp::min(x0, *x1)..std::cmp::max(x0, *x1) + 1 {
                        insert(&mut grid, x, y0, shift);
                    }
                } else {
                    panic!();
                }
                (x0, y0) = (*x1, *y1);
            }
        }
    }

    for y in 1..height + 1 {
        for x in shift..width + shift + 1 {
            if filled(&grid, x, y, shift) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
