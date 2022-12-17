use std::env;
use std::fs;

fn insert(grid: &mut Vec<u128>, x: usize, y: usize, span: usize) {
    let y = y * span + x / 128;
    let x = x % 128;
    grid[y] |= (1 as u128) << x;
}

fn filled(grid: &Vec<u128>, x: usize, y: usize, span: usize) -> bool {
    let y = y * span + x / 128;
    let x = x % 128;
    grid[y] & ((1 as u128) << x) != 0
}

#[allow(dead_code)]
fn print(stone: &Vec<u128>, grid: &Vec<u128>, span: usize) {
    for y in 0..stone.len() / span {
        for x in 0..span * 128 {
            if filled(&stone, x, y, span) {
                print!("#");
            } else if filled(&grid, x, y, span) {
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
                wall.push((x, y));
            }
        }
        rocks.push(wall);
    }
    let height = ymax+1;
    let span = 8;

    let mut stone = vec![0 as u128; span * height];
    for wall in rocks {
        if wall.len() > 0 {
            let (mut x0, mut y0) = wall[0];
            insert(&mut stone, x0, y0, span);
            for (x1, y1) in &wall[1..] {
                if x0 == *x1 {
                    for y in std::cmp::min(y0, *y1)..std::cmp::max(y0, *y1) + 1 {
                        insert(&mut stone, x0, y, span);
                    }
                } else if y0 == *y1 {
                    for x in std::cmp::min(x0, *x1)..std::cmp::max(x0, *x1) + 1 {
                        insert(&mut stone, x, y0, span);
                    }
                } else {
                    panic!();
                }
                (x0, y0) = (*x1, *y1);
            }
        }
    }
    let mut grid = stone.clone();
    // print(&stone, &grid, span);
    // println!("");

    let mut stack = Vec::new();
    let mut part_a = 0;
    stack.push(((500 as usize), 0 as usize));
    'outer: loop {
        let (mut x, mut y) = stack.pop().unwrap();
        loop {
            if y + 1 == height {
                break 'outer;
            } else if !filled(&grid, x, y + 1, span) {
                stack.push((x, y));
                y += 1;
            } else if !filled(&grid, x - 1, y + 1, span) {
                stack.push((x, y));
                x -= 1;
                y += 1;
            } else if !filled(&grid, x + 1, y + 1, span) {
                stack.push((x, y));
                x += 1;
                y += 1;
            } else {
                insert(&mut grid, x, y, span);
                part_a += 1;
                break;
            }
        }
    }
    // print(&stone, &grid, span);
    // println!("");

    for _ in 0..span {
        stone.push(0);
    }
    for _ in 0..span {
        stone.push(u128::MAX);
    }
    let mut grid = stone.clone();
    // print(&stone, &grid, span);

    let mut stack = Vec::new();
    let mut part_b = 0;
    stack.push(((500 as usize), 0 as usize));

    while !stack.is_empty() {
        let (mut x, mut y) = stack.pop().unwrap();
        loop {
            if !filled(&grid, x, y + 1, span) {
                stack.push((x, y));
                y += 1;
            } else if !filled(&grid, x - 1, y + 1, span) {
                stack.push((x, y));
                x -= 1;
                y += 1;
            } else if !filled(&grid, x + 1, y + 1, span) {
                stack.push((x, y));
                x += 1;
                y += 1;
            } else {
                insert(&mut grid, x, y, span);
                part_b += 1;
                break;
            }
        }
    }

    // print(&stone, &grid, span);
    println!("until abyss:   {}", part_a);
    println!("until stopped: {}", part_b);
}
