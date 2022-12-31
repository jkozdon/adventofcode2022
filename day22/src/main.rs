use std::env;
use std::fs;

fn do_move(
    (x, y, d): (usize, usize, usize),
    num: u32,
    map: &Vec<Vec<char>>,
) -> (usize, usize, usize) {
    let mut x = x;
    let mut y = y;
    if d == 0 {
        for _ in 0..num {
            if x + 1 < map[y].len() && map[y][x + 1] != ' ' {
                if map[y][x + 1] == '.' {
                    x += 1;
                } else {
                    break;
                }
            } else {
                let mut xx = 0;
                while map[y][xx] == ' ' {
                    xx += 1;
                }
                if map[y][xx] == '.' {
                    x = xx;
                } else {
                    break;
                }
            }
        }
    } else if d == 1 {
        for _ in 0..num {
            if y + 1 < map.len() && x < map[y + 1].len() && map[y + 1][x] != ' ' {
                if map[y + 1][x] == '.' {
                    y += 1;
                } else {
                    break;
                }
            } else {
                let mut yy = 0;
                while map[yy][x] == ' ' {
                    yy += 1;
                }
                if map[yy][x] == '.' {
                    y = yy;
                } else {
                    break;
                }
            }
        }
    } else if d == 2 {
        for _ in 0..num {
            if x > 0 && map[y][x - 1] != ' ' {
                if map[y][x - 1] == '.' {
                    x -= 1;
                } else {
                    break;
                }
            } else {
                let mut xx = map[y].len() - 1;
                while map[y][xx] == ' ' {
                    xx -= 1;
                }
                if map[y][xx] == '.' {
                    x = xx;
                } else {
                    break;
                }
            }
        }
    } else if d == 3 {
        for _ in 0..num {
            if y > 0 && map[y - 1][x] != ' ' {
                if map[y - 1][x] == '.' {
                    y -= 1;
                } else {
                    break;
                }
            } else {
                let mut yy = map.len() - 1;
                while map[yy].len() <= x || map[yy][x] == ' ' {
                    yy -= 1;
                }
                if map[yy][x] == '.' {
                    y = yy;
                } else {
                    break;
                }
            }
        }
    } else {
        panic!("{}", d);
    }
    (x, y, d)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        if line.len() == 0 {
            break;
        }
        map.push(line.chars().collect());
    }
    let route = file.lines().last().unwrap();

    let mut pos: (usize, usize, usize) = (0, 0, 0);
    for (i, v) in map[0].iter().enumerate() {
        if *v == '.' {
            pos = (i, 0, 0);
            break;
        }
    }

    for mv in route.split_inclusive(|c: char| c == 'R' || c == 'L') {
        let (num, turn) = if mv.chars().last().unwrap() == 'R' {
            (mv.get(0..mv.len() - 1).unwrap(), Some('R'))
        } else if mv.chars().last().unwrap() == 'L' {
            (mv.get(0..mv.len() - 1).unwrap(), Some('L'))
        } else {
            (mv, None)
        };
        let num: u32 = num.trim().parse().expect("number");

        pos = do_move(pos, num, &map);
        if turn == Some('R') {
            pos.2 = if pos.2 < 3 { pos.2 + 1 } else { 0 };
        } else if turn == Some('L') {
            pos.2 = if pos.2 > 0 { pos.2 - 1 } else { 3 };
        }
    }
    println!("score: {:}", (pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + pos.2);
}
