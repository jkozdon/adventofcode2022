use std::collections::VecDeque;
use std::env;
use std::fs;

fn steps(grid: &mut Vec<Vec<(u32, i32)>>, sx: usize, sy: usize) -> i32 {
    let mut pnts = VecDeque::<(usize, usize)>::new();
    pnts.push_back((sx, sy));
    grid[sy][sx].1 = 0;

    let steps: i32;
    'outer: loop {
        let (x0, y0) = pnts.pop_front().unwrap();
        let (cur, lvl) = grid[y0][x0];
        for k in 0..4 {
            let (x1, y1) = match k {
                0 => {
                    if x0 != 0 {
                        (x0 - 1, y0)
                    } else {
                        continue;
                    }
                }
                1 => {
                    if x0 + 1 < grid[0].len() {
                        (x0 + 1, y0)
                    } else {
                        continue;
                    }
                }
                2 => {
                    if y0 != 0 {
                        (x0, y0 - 1)
                    } else {
                        continue;
                    }
                }
                3 => {
                    if y0 + 1 < grid.len() {
                        (x0, y0 + 1)
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };
            let (nbr, nvl) = grid[y1][x1];
            if nbr == 'E' as u32 && cur + 1 >= 'z' as u32 {
                steps = lvl + 1;
                break 'outer;
            } else if cur + 1 >= nbr && nvl == i32::MAX {
                pnts.push_back((x1, y1));
                grid[y1][x1].1 = lvl + 1;
            }
        }
    }

    steps
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");
    let start = file.find("S").expect("no S???");
    let last = file.find("E").expect("no S???");

    let mut grid: Vec<Vec<(u32, i32)>> = file
        .lines()
        .map(|l| l.chars().map(|c| (c as u32, i32::MAX)).collect())
        .collect();

    // plus one to account for newline
    let sx = start % (grid[0].len() + 1);
    let sy = start / (grid[0].len() + 1);
    grid[sy][sx] = ('a' as u32, 0);
    println!("S->E: {}", steps(&mut grid, sx, sy));

    let lx = last % (grid[0].len() + 1);
    let ly = last / (grid[0].len() + 1);
    grid[sy][sx] = ('z' as u32, 0);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let (cur, _) = grid[y][x];
            grid[y][x] = (
                if cur == 'a' as u32 {
                    'E' as u32
                } else {
                    ('a' as u32) + (('z' as u32) - cur)
                },
                i32::MAX,
            );
        }
    }
    grid[ly][lx] = ('a' as u32, 0);
    println!("a->E: {}", steps(&mut grid, lx, ly));
}
