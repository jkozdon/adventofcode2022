use std::env;
use std::fs;

fn insert(grid: &mut Vec<u128>, x: usize, y: usize) {
    grid[y] |= (1 as u128) << x;
}

fn filled(grid: &Vec<u128>, x: usize, y: usize) -> bool {
    grid[y] & ((1 as u128) << x) != 0
}

fn print(stone: &Vec<u128>, grid: &Vec<u128>, width: usize) {
    for y in 0..stone.len() {
        for x in 0..width {
            if filled(&stone, x, y) {
                print!("#");
            } else if filled(&grid, x, y) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!("");
    }
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
                wall.push((x, y - 1));
            }
        }
        rocks.push(wall);
    }
    let height = ymax;
    let width = xmax - xmin + 1;
    assert!(width < 128);
    let shift = xmin - 64 + width / 2;
    let mut stone = vec![0 as u128; height];
    for wall in rocks {
        if wall.len() > 0 {
            let (mut x0, mut y0) = wall[0];
            insert(&mut stone, x0 - shift, y0);
            for (x1, y1) in &wall[1..] {
                if x0 == *x1 {
                    for y in std::cmp::min(y0, *y1)..std::cmp::max(y0, *y1) + 1 {
                        insert(&mut stone, x0 - shift, y);
                    }
                } else if y0 == *y1 {
                    for x in std::cmp::min(x0, *x1)..std::cmp::max(x0, *x1) + 1 {
                        insert(&mut stone, x - shift, y0);
                    }
                } else {
                    panic!();
                }
                (x0, y0) = (*x1, *y1);
            }
        }
    }
    let mut grid = stone.clone();
    print(&stone, &grid, 128);
    println!("");

    let mut stack = Vec::new();
    let mut part_a = 0;
    stack.push(((500 as usize) - shift, 0 as usize));
    'outer: loop {
        let (mut x, mut y) = stack.pop().unwrap();
        loop {
            if y + 1 == height {
                break 'outer;
            } else if !filled(&grid, x, y + 1) {
                stack.push((x, y));
                y += 1;
            } else if !filled(&grid, x - 1, y + 1) {
                stack.push((x, y));
                x -= 1;
                y += 1;
            } else if !filled(&grid, x + 1, y + 1) {
                stack.push((x, y));
                x += 1;
                y += 1;
            } else {
                insert(&mut grid, x, y);
                part_a += 1;
                break;
            }
        }
    }
    print(&stone, &grid, 128);
    println!("");

    let mut grid = stone.clone();
    println!("{}", part_a);
}
